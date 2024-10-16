library(microbenchmark)
library(mapsf)
library(sf)
library(gepafer)

m <- mf_get_mtq()
m <- st_transform(m, "EPSG:4326")
xx <- st_coordinates(m)
df <- data.frame(lat = round(xx[1:1000,2], 5), lon = round(xx[1:1000,1], 5))
poly <- encode_coordinates(df, 5)

microbenchmark(
  gepafeur_encode = encode_coordinates(df, 5),
  gepafeur_decode = decode_coordinates(poly, 5),
  times = 100,
  unit = 'microseconds'
)

# Test string from https://github.com/riatelab/osrm/issues/71
teststr <- "}}~_GbuzpLSIc@e@KQMYo@_C}AhAkAz@s@j@wAjA{@t@m@n@_AbAc@d@UX[^[`@{@nAILQTo@`ASTEHMLSRWPQLUJMDIB]HWFE@o@N]JMDMDSJMHKFMLMJKLMPOXEJEHQd@Wr@IPAD[z@e@nAo@xAs@rAmAtBABe@p@m@v@y@fAKNQRMJKJURy@v@OLe@b@QNKJGFEBKHMHKFKFOHMFQHSHcBn@cDhAg@R_@Nc@Tw@`@YNWLYP]V[TMJWV_@\\i@l@]b@c@f@ONEFcAjAYPMBQACAAAOGE\\M`AIb@Mr@GXW`AMf@Qn@ELKZGRk@fBELCHCDEHILKR_@l@ILKPMPORCBW^WZKHEDA@i@n@CBs@z@MNMLGB[Pg@^UL]NiAh@CDCDABCFAF?F?J@DBL@@@@Rp@HRFPXz@Nd@BDJ\\L\\@DHPDN@HBF@HBH?NAJCZKn@G\\GZg@dCu@lDEXYdAI\\Id@ERMp@If@CJERETI`@WpAEPEX[bBCNKd@If@UfAI\\Oz@CLMr@AB[|AMt@UjA]hBUlAGZAFAHAHCFCLG^Kd@?BCNKb@ADG^Kf@Qz@GXAHAFCHYtA?BADOv@WvAuAjHm@bDg@hCEVOz@u@vDMj@{@|EI\\q@dDwAlHIb@EPi@rCEJEDCBKJ?N?^?b@Ej@Gl@Ir@EZCTAP?X?`@AN@HDT?FLl@H\\BN@HFZ@PBNBT@`@@J?L@J?d@?ZAVATAJA?CVAFAJCJEPEPQj@c@vAUz@IZI^Op@G^AFERGf@AJ?HI~@[~DM~AAFOtBK`AOn@Wt@c@bAcAxAOPg@l@EF[f@Yn@Q\\MTY|AETSxAObBGvAA`@?|AA^ARAXAd@?NCx@Ep@?DGr@IjACRIr@?FE`@E`@ANKn@AHG^GXCLIZENYx@Wp@S`@[j@OTKLKLEFOPuApA]TiAhAy@p@e@f@]^q@|@k@`AINININQf@ENQd@Ul@ADK\\GTERMn@ERADKd@?@Ih@CHETCPAFCPAFWKECg@SSGGEKEiA_@i@SYK}@[QGEACAi@I[C]?a@Bw@JA?A?q@Je@Dw@HcAL_@FiALq@L_AJaBDk@B]?g@?[@k@@K?UBQDSFQHQHOJMLQN[\\]XGDOJMHe@Pe@LODu@ReAVuB`@m@Fi@@k@C_@KYMg@]GEcAw@y@g@aA]o@KYGm@KgBa@QGm@UUIICw@c@OIIGECAAQK[UQMOK_@]We@]k@AAGI_@y@Sh@GZKj@i@xDQhAQnAYVw@`AUf@e@zA?@GZGRCLIt@OvBCHKnAId@Oj@M^Ud@CFSZWXEFm@`@_@P[J]F[BW?]ESEUGcAYg@O[EWAS@i@DmAN_@FOBEBEFAFCPCNEJGFMFa@Lc@LKFQF_@R[RURGFg@h@CBGFGFEFw@bASVW\\OP_@\\QLw@l@KFa@\\OLe@f@a@f@INWh@O\\Sd@a@|@Sf@u@vAg@v@k@~@GJy@jAi@x@]h@SZGJ}@pAa@l@m@|@QVg@t@ILm@`A[b@CDsAnBeA|A_@l@W^iBpCe@p@aAzAgC|Di@v@i@v@Y^iA|Ai@t@i@z@QLQPg@|@cF|H}AfCgBxCe@x@k@dA_@p@uA`CmApBa@p@Ub@S^kA|BgBlC_BrBMPW`@w@v@gBhBqClCaB~AsApA{BtBKLgAbAo@l@]\\RvABPl@`HLzANpANbBHlADrABpA?LC~BEvAMxAG^EPWdAQj@m@~AIT_@`AKh@I`@I`@Ej@C^?\\BdB@p@DvA@F@^@d@XPh@\\lAt@xBxAn@b@TN@?NJZRjAv@~CrBvCjBVPnGbEp@d@bAr@xExCf@^r@b@BB^TpB|@rAn@pAf@RHTFRH|@Xh@N~@VrA^j@N~@V~@V^J\\HVHTF^J^JVHr@R^J`@J\\JpAVZHXHn@R\\Lf@NjBj@LVFV?PCb@AR?BIl@If@If@IRKn@"

microbenchmark::microbenchmark(
  GepafeurDecodeTestIssueGH = decode_coordinates(teststr, 5),
  GooglePolylineDecodeTestIssueGH = googlePolylines::decode(teststr),
  times = 100,
  unit = 'microseconds'
)

