# Gepafer

The name is still subject to change :)

Does the same thing as gepaf, but in a more efficient way.

## Usage

### Encoding

The `encode_polyline` function takes a data frame with two columns, `lat` and `lon`, and returns an encoded polyline.

```R
library(gepafer)

coords <- data.frame(lat = c(38.5, 40.7, 43.252), lon = c(-120.2, -120.95, -126.453))
encpoly <- encode_polyline(coords, factor=5)
encpoly
#> [1] "_p~iF~ps|U_ulLnnqC_mqNvxq`@"
```

### Decoding

The `decode_polyline` function takes an encoded polyline and returns a data frame with two columns, `lat` and `lon`.

```R
library(gepafer)

coords <- decode_polyline(enc_polyline = "_p~iF~ps|U_ulLnnqC_mqNvxq`@", factor=5)
coords
#>      lat      lon
#> 1 38.500 -120.200
#> 2 40.700 -120.950
#> 3 43.252 -126.453
```

## Alternatives

- [gepaf](https://github.com/mthh/gepaf) - The original gepaf package
- [googlePolylines](https://cran.r-project.org/package=googlePolylines)