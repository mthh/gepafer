use extendr_api::prelude::*;
use geo_types::LineString;

#[derive(IntoDataFrameRow)]
struct LonLatStruct {
    lon: Rfloat,
    lat: Rfloat,
}

/// Encode coordinates to polyline.
/// @export
#[extendr]
fn encode_coordinates(coords: Dataframe<LonLatStruct>, factor: u32) -> Robj {
  // We need to convert the DataFrame to a Vec<(f64, f64)>
  // So, because DataFrame<T> don't offer Iterators for now
  // (see https://github.com/extendr/extendr/issues/714),
  // we need to convert it to a R List first, then we can convert it to a Rust HashMap...
  let coords = coords.as_list().unwrap().into_hashmap();
  // We need to check if the DataFrame has the columns "lon" and "lat"
  if !coords.contains_key(&"lon") || !coords.contains_key(&"lat") {
      return Robj::from(Error::from("DataFrame must have columns 'lon' and 'lat'"));
  }
  // Build a Vec<(f64, f64)> of coords from the DataFrame
  let coords = coords.get(&"lon").unwrap().as_real_vector().unwrap().iter()
      .zip(coords.get(&"lat").unwrap().as_real_vector().unwrap().iter())
      .map(|(a, b)| (*a, *b))
      .collect::<Vec<(f64, f64)>>();
  // Actually encode the coordinates
  let result = polyline::encode_coordinates(Into::<LineString<_>>::into(coords), factor);
  // Return the encoded polyline, or an error if the encoding failed
  if result.is_err() {
    Robj::from(Error::from(format!("Failed to encode polyline: {}", result.unwrap_err())))
  } else {
    Robj::from(result.unwrap())
  }
}

/// Decode coordinates
/// @export
#[extendr]
fn decode_coordinates(polyline: String, factor: u32) -> Robj {
  // Decode the polyline (it returns a geo_types::LineString)
  let result = polyline::decode_polyline(polyline.as_str(), factor);
  // If the decoding failed, return the error message
  if result.is_err() {
    return Robj::from(Error::from(format!("Failed to decode polyline: {}", result.unwrap_err())));
  }
  // Convert the LineString to a DataFrame with columns "lon" and "lat"
  result.unwrap().0.iter()
    .map(|c| LonLatStruct { lon: c.x.into(), lat: c.y.into() })
    .collect::<Vec<LonLatStruct>>()
    .into_dataframe()
    .unwrap()
    .into_robj()
}

/*
/// Encode coordinates to polyline.
/// This function assumes that their is no need to check the validity of the coordinates
/// (and so it should only be used after being wrapped in a safe R function).
/// @export
#[extendr]
fn encode_coordinates_raw(coords_lat: Doubles, coords_lon: Doubles, factor: u32) -> Robj {
  let coords = coords_lon.iter().zip(coords_lat.iter())
      .map(|(a, b)| (a.inner(), b.inner()))
      .collect::<Vec<(f64, f64)>>();
  let result = polyline::encode_coordinates(Into::<LineString<_>>::into(coords), factor);
  if result.is_err() {
    Robj::from(Error::from(format!("Failed to encode polyline: {}", result.unwrap_err())))
  } else {
    Robj::from(result.unwrap())
  }
}
*/

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod gepafer;
    fn encode_coordinates;
    fn decode_coordinates;
    // fn encode_coordinates_w;
}
