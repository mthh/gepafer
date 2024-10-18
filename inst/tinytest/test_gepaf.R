# Some first tests, the same as in gepaf package
coords1 <- data.frame(lat = c(38.5654802, 40.7448702, 43.2525405),
                     lon = c(-120.2545405, -120.9544508, -126.4536504))
coords2 <- data.frame(lat = c(38.56548, 40.74487, 43.25254),
                      lon = c(-120.25454, -120.95445, -126.45365))
encpoly1 <- encode_polyline(coords1, factor = 5)
encpoly2 <- encode_polyline(coords2, factor = 5)
expect_equal(encpoly1, encpoly2)
expect_equal(encpoly1, "gikjFze~|UethLlugC}whN~`q`@")
expect_equal(decode_polyline("gikjFze~|UethLlugC}whN~`q`@"), coords2)

# Tests for rounding
coords3 <- data.frame(
  lon = c(-112.084004, -112.083914, -112.083965),
  lat = c(36.05322, 36.053573, 36.053845)
)
expect_equal(encode_polyline(coords3, factor = 5), 'ss`{E~kbkTeAQw@J')

coords4 <- data.frame(
  lon = c(0.000006, 0.000002),
  lat = c(0, 0)
)
expect_equal(encode_polyline(coords4, factor = 5), '?A?@')

# Tests for other precision
coords5 <- data.frame(lat = c(38.5, 40.7, 43.252),
                      lon = c(-120.2, -120.95, -126.453))

expect_equal(encode_polyline(coords5, factor = 0), 'mAnFC@CH')
expect_equal(encode_polyline(coords5, factor = 6), '_izlhA~rlgdF_{geC~ywl@_kwzCn`{nI')
