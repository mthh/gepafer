use extendr_api::prelude::*;

fn scale(n: f64, factor: i32) -> i64 {
    let scaled = n * (f64::from(factor));
    scaled.round() as i64
}

#[inline(always)]
fn encode(delta: i64, output: &mut String) {
    let mut value = delta << 1;
    if value < 0 {
        value = !value;
    }
    while value >= 0x20 {
        // We can unwrap here because the caller checked coordinates bounds
        let from_char = char::from_u32(((0x20 | (value & 0x1f)) + 63) as u32).unwrap();
        output.push(from_char);
        value >>= 5;
    }
    // We can unwrap here because the caller checked coordinates bounds
    let from_char = char::from_u32((value + 63) as u32).unwrap();
    output.push(from_char);
}

/// Encode coordinates to polyline.
/// @export
#[extendr]
fn encode_polyline(coords: List, factor: u32) -> Robj {
  // We need to convert the DataFrame (that we handle directly as a List) to a Vec<(f64, f64)>
  // So, because DataFrame<T> don't offer Iterators for now
  // (see https://github.com/extendr/extendr/issues/714),
  // we need to handle it as a List, then we can convert it to a Rust HashMap...
  let coords = coords.into_hashmap();

  // We need to check if the List has the names "lon" and "lat"
  if !coords.contains_key(&"lon") || !coords.contains_key(&"lat") {
      return Robj::from(Error::from("DataFrame must have columns 'lon' and 'lat'"));
  }

  // Build a Vec<(f64, f64)> of coords from the DataFrame
  let coords = coords.get(&"lon").unwrap().as_real_vector().unwrap().iter()
      .zip(coords.get(&"lat").unwrap().as_real_vector().unwrap().iter())
      .map(|(a, b)| (*a, *b))
      .collect::<Vec<(f64, f64)>>();

  // Actually encode the coordinates
  let mut result = String::new();
  let factor = 10i32.pow(factor);
  let mut prev_lat = 0;
  let mut prev_lon = 0;
  for (i, (lon, lat)) in coords.into_iter().enumerate() {
    if (lat < -90.0 || lat > 90.0) || (lon < -180.0 || lon > 180.0) || lat.is_nan() || lon.is_nan() {
      return Robj::from(Error::from(format!("Invalid coordinates at index {}", i + 1)));
    }
    let lat = scale(lat, factor);
    let lon = scale(lon, factor);
    let delta_lat = lat - prev_lat;
    let delta_lon = lon - prev_lon;
    encode(delta_lat, &mut result);
    encode(delta_lon, &mut result);
    prev_lat = lat;
    prev_lon = lon;
  }

  // Return the encoded polyline as an R object
  Robj::from(result)
}

/// Decode coordinates
/// @export
#[extendr]
pub fn decode_polyline(polyline: &str, factor: u32) -> Robj {
    let mut lat: i64 = 0;
    let mut lon: i64 = 0;
    let factor = 10i64.pow(factor);
    let inv_factor = 1.0 / factor as f64;

    let mut coordinates_lat = vec![];
    let mut coordinates_lon = vec![];
    let bytes = polyline.as_bytes();
    let len = bytes.len();
    let mut index = 0;

    while index < len {
        // Decode latitude
        let mut shift = 0;
        let mut result = 0;
        loop {
            let byte = bytes[index] - 63;
            index += 1;
            result |= ((byte & 0x1f) as i64) << shift;
            shift += 5;
            if byte < 0x20 {
                break;
            }
        }
        let dlat = if result & 1 != 0 { !(result >> 1) } else { result >> 1 };
        lat += dlat;

        // Decode longitude
        shift = 0;
        result = 0;
        loop {
            let byte = bytes[index] - 63;
            index += 1;
            result |= ((byte & 0x1f) as i64) << shift;
            shift += 5;
            if byte < 0x20 {
                break;
            }
        }
        let dlng = if result & 1 != 0 { !(result >> 1) } else { result >> 1 };
        lon += dlng;

        // Convert and store the result in f64, then check bounds
        let lat_f64 = lat as f64 * inv_factor;
        let lon_f64 = lon as f64 * inv_factor;

        if lat_f64 < -90.0 || lat_f64 > 90.0 || lon_f64 < -180.0 || lon_f64 > 180.0 {
            return Robj::from(Error::from("Invalid coordinates"));
        }

        coordinates_lon.push(lon_f64);
        coordinates_lat.push(lat_f64);
    }

    // Create a List with the decoded coordinates
    let length = coordinates_lat.len();
    let mut l = list!(lon = coordinates_lon, lat = coordinates_lat);
    // Set the class to "data.frame" (this is way cheaper than creating a Dataframe,
    // both in R with as.data.frame() and in Rust with Dataframe<T>)
    l.set_class(&["data.frame"]).unwrap();
    // Also set row.names to coordinates indexes
    l.set_attrib(&["row.names"], &Robj::from(1..=length as i32)).unwrap();
    l.into()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod gepafer;
    fn encode_polyline;
    fn decode_polyline;
}
