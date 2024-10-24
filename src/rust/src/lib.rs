use extendr_api::prelude::*;

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

/// @name encode_polyline
/// @title Encode Coordinates to Google Polylines
/// @description Encode a data frame of coordinates to a Google polyline.
/// @param df_coords a data frame of coordinates with two columns: 'lat' and
/// 'lon'. Coordinates must be in decimal degrees (WGS84).
/// @param factor number of decimal digits to be used.
/// @return An encoded polyline is returned.
/// @examples
/// coords <- data.frame(
///   lat = c(38.5, 40.7, 43.252),
///   lon = c(-120.2, -120.95, -126.453)
/// )
/// encpoly <- encode_polyline(coords)
/// encpoly
/// @export
#[extendr]
fn encode_polyline(df_coords: List, #[default = "5"] factor: u32) -> Robj {
  // We need to convert the DataFrame (that we handle directly as a List) to a Vec<(f64, f64)>
  // So, because DataFrame<T> don't offer Iterators for now
  // (see https://github.com/extendr/extendr/issues/714),
  // we need to handle it as a List, then we can convert it to a Rust HashMap...
  let coords = df_coords.into_hashmap();

  // We need to check if the List has the names "lon" and "lat"
  if !coords.contains_key(&"lon") || !coords.contains_key(&"lat") {
      return Robj::from(Error::from("DataFrame must have columns 'lon' and 'lat'"));
  }

  // Extract the coordinates from the input R object
  let coords_lon = coords.get(&"lon").unwrap().as_real_vector();
  let coords_lat = coords.get(&"lat").unwrap().as_real_vector();
  if coords_lat.is_none() || coords_lon.is_none() {
      return Robj::from(Error::from("Invalid coordinates"));
  }
  let coords_lat = coords_lat.unwrap();
  let coords_lon = coords_lon.unwrap();

  // Check if the coordinates have the same length
  if coords_lat.len() != coords_lon.len() {
      return Robj::from(Error::from("Latitude and longitude vectors must have the same length"));
  }

  // Actually encode the coordinates
  let mut result = String::new();
  let factor = 10i32.pow(factor);
  let mut prev_lat = 0;
  let mut prev_lon = 0;
  for (i, (lon, lat)) in coords_lon.into_iter().zip(coords_lat.into_iter()).enumerate() {
    // Note that checking bounds here add some (minor) overhead, so it could be removed in the future
    // if performance is really critical.
    if (lat < -90.0 || lat > 90.0) || (lon < -180.0 || lon > 180.0) || lat.is_nan() || lon.is_nan() {
      return Robj::from(Error::from(format!("Invalid coordinates at index {}", i + 1)));
    }
    let lat = (lat * factor as f64).round() as i64;
    let lon = (lon * factor as f64).round() as i64;
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

/// @name decode_polyline
/// @title Decode a Google Polyline to a Data Frame
/// @description Decode a Google polyline to a data frame of coordinates.
/// @param enc_polyline a Google polyline.
/// @param factor number of decimal digits to be used.
/// @return A data frame of latitudes and longitudes is returned.
/// @examples
/// coords <- decode_polyline(enc_polyline = "_p~iF~ps|U_ulLnnqC_mqNvxq`@")
/// coords
/// @export
#[extendr]
pub fn decode_polyline(enc_polyline: &str, #[default = "5"] factor: u32) -> Robj {
    let mut lat: i64 = 0;
    let mut lon: i64 = 0;
    let factor = 10i64.pow(factor);
    let inv_factor = 1.0 / factor as f64;

    let mut coordinates_lat = vec![];
    let mut coordinates_lon = vec![];
    let bytes = enc_polyline.as_bytes();
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
        let lat_f64 = (lat as f64).round() * inv_factor;
        let lon_f64 = (lon as f64).round() * inv_factor;

        // Note that checking bounds here is not strictly necessary, as the polyline
        // encoding itself should prevent invalid coordinates.
        // However, we are checking it in case a corrupted polyline is passed to the function.
        // This also add some (minor) overhead, so it could be removed in the future
        // if performance is critical.
        if lat_f64 < -90.0 || lat_f64 > 90.0 || lon_f64 < -180.0 || lon_f64 > 180.0 {
            return Robj::from(Error::from("Invalid coordinates"));
        }

        coordinates_lon.push(lon_f64);
        coordinates_lat.push(lat_f64);
    }

    // Create a List with the decoded coordinates
    let length = coordinates_lat.len();
    let mut l = list!(lat = coordinates_lat, lon = coordinates_lon);
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
