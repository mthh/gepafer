coords1 <- data.frame(lat = c(38.5654802, 40.7448702, 43.2525405),
                     lon = c(-120.2545405, -120.9544508, -126.4536504))
coords2 <- data.frame(lat = c(38.56548, 40.74487, 43.25254),
                      lon = c(-120.25454, -120.95445, -126.45365))
encpoly1 <- encode_polyline(coords1, factor = 5)
encpoly2 <- encode_polyline(coords2, factor = 5)
expect_equal(encpoly1, encpoly2)
expect_equal(encpoly1, "gikjFze~|UethLlugC}whN~`q`@")
expect_equal(decode_polyline("gikjFze~|UethLlugC}whN~`q`@"), coords2)
