# Gepafer

> The name is still subject to discussion.

Does the same thing as [`gepaf`](https://github.com/mthh/gepaf) (which stands for *Google Encoded Polyline Algorithm Format*) but in a more efficient way, using Rust.

## Motivation

Some packages, such as `osrm`, need to encode / decode coordinates in order to communicate with the OSRM API.

There are already some R packages that provide this functionality:

- The [`gepaf`](https://github.com/mthh/gepaf) package is a great package (disclaimer: I'm its co-author with [@rcarto](https://github.com/rcarto)), but it is written in pure R and it is relatively slow.
- The [`googlePolylines`](https://github.com/SymbolixAU/googlePolylines) package is fast but, on 10/10/2024, the CRAN announced that the package will be archived on 24/10/2024 if nothing is done by its maintainer.

This package aims to provide the same functionality as `gepaf` while being as fast as `googlePolylines`, using Rust.

Moreover, it's a great opportunity to learn how to write R packages with Rust.
To do so, `gepafer` is using the [`extendr`](https://github.com/extendr/extendr) Rust crate and the [`rextendr`](https://github.com/extendr/rextendr) R package
(but other alternatives such as [`savvy`](https://github.com/yutannihilation/savvy) exist and could be considered in the future).

## Installation

You can install the development version of `gepafer` from GitHub with:

```R
# install.packages("remotes")
remotes::install_github("mthh/gepafer")
```

You will need the Rust toolchain to compile the Rust code.
The Minimum Supported Rust Version (MSRV) is 1.61.0.

The package is not yet on CRAN.

## Usage

This packages aims to be a drop-in replacement for the `gepaf` package.
It provides two functions, `encode_polyline` and `decode_polyline`, that do the same thing as the functions with the same name in the `gepaf` package. 

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

## License

GPL-3 (as the `gepaf` package)
