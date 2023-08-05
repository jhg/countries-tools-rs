use parse_display::{Display, FromStr};

use super::Country;

use CountryAlpha2::*;
use CountryAlpha3::*;

struct CountriesList<'a> {
    countries: &'a [Country],
}

impl<'a> CountriesList<'a> {
    #[inline]
    const fn new(countries: &'a [Country]) -> Self {
        Self { countries }
    }

    #[inline]
    pub const fn country_by_numeric(&self, numeric: u16) -> Option<Country> {
        let mut low = 0;
        let mut high = self.countries.len() - 1;

        while low <= high {
            let mid = (low + high) / 2;
            let mid_val = self.countries[mid].numeric();

            if mid_val < numeric {
                low = mid + 1;
            } else if mid_val > numeric {
                high = mid - 1;
            } else {
                return Some(self.countries[mid]);
            }
        }

        None
    }

    #[inline]
    pub const fn country_by_alpha2(&self, alpha2: CountryAlpha2) -> Country {
        self.countries[alpha2.internal_offset()]
    }

    #[inline]
    pub const fn country_by_alpha3(&self, alpha3: CountryAlpha3) -> Country {
        self.countries[alpha3.internal_offset()]
    }
}

// NOTE: This is static as that reduces the size of the binary and makes it faster.
//       Order is important, as it is used for binary search by the `Country` numeric.
//       Also, alpha 2 and alpha 3 codes are sorted alphabetically but the values are
//       offsets into the array for fast lookup from the `CountryAlpha2` and `CountryAlpha3`
//       enums.
static COUNTRIES: CountriesList = CountriesList::new(&[
    Country::new(AF, AFG, 004),
    Country::new(AL, ALB, 008),
    Country::new(AQ, ATA, 010),
    Country::new(DZ, DZA, 012),
    Country::new(AS, ASM, 016),
    Country::new(AD, AND, 020),
    Country::new(AO, AGO, 024),
    Country::new(AG, ATG, 028),
    Country::new(AZ, AZE, 031),
    Country::new(AR, ARG, 032),
    Country::new(AU, AUS, 036),
    Country::new(AT, AUT, 040),
    Country::new(BS, BHS, 044),
    Country::new(BH, BHR, 048),
    Country::new(BD, BGD, 050),
    Country::new(AM, ARM, 051),
    Country::new(BB, BRB, 052),
    Country::new(BE, BEL, 056),
    Country::new(BM, BMU, 060),
    Country::new(BT, BTN, 064),
    Country::new(BO, BOL, 068),
    Country::new(BA, BIH, 070),
    Country::new(BW, BWA, 072),
    Country::new(BV, BVT, 074),
    Country::new(BR, BRA, 076),
    Country::new(BZ, BLZ, 084),
    Country::new(IO, IOT, 086),
    Country::new(SB, SLB, 090),
    Country::new(VG, VGB, 092),
    Country::new(BN, BRN, 096),
    Country::new(BG, BGR, 100),
    Country::new(MM, MMR, 104),
    Country::new(BI, BDI, 108),
    Country::new(BY, BLR, 112),
    Country::new(KH, KHM, 116),
    Country::new(CM, CMR, 120),
    Country::new(CA, CAN, 124),
    Country::new(CV, CPV, 132),
    Country::new(KY, CYM, 136),
    Country::new(CF, CAF, 140),
    Country::new(LK, LKA, 144),
    Country::new(TD, TCD, 148),
    Country::new(CL, CHL, 152),
    Country::new(CN, CHN, 156),
    Country::new(TW, TWN, 158),
    Country::new(CX, CXR, 162),
    Country::new(CC, CCK, 166),
    Country::new(CO, COL, 170),
    Country::new(KM, COM, 174),
    Country::new(YT, MYT, 175),
    Country::new(CG, COG, 178),
    Country::new(CD, COD, 180),
    Country::new(CK, COK, 184),
    Country::new(CR, CRI, 188),
    Country::new(HR, HRV, 191),
    Country::new(CU, CUB, 192),
    Country::new(CY, CYP, 196),
    Country::new(CZ, CZE, 203),
    Country::new(BJ, BEN, 204),
    Country::new(DK, DNK, 208),
    Country::new(DM, DMA, 212),
    Country::new(DO, DOM, 214),
    Country::new(EC, ECU, 218),
    Country::new(SV, SLV, 222),
    Country::new(GQ, GNQ, 226),
    Country::new(ER, ERI, 232),
    Country::new(ET, ETH, 231),
    Country::new(EE, EST, 233),
    Country::new(FO, FRO, 234),
    Country::new(FK, FLK, 238),
    Country::new(GS, SGS, 239),
    Country::new(FJ, FJI, 242),
    Country::new(FI, FIN, 246),
    Country::new(AX, ALA, 248),
    Country::new(FR, FRA, 250),
    Country::new(GF, GUF, 254),
    Country::new(PF, PYF, 258),
    Country::new(TF, ATF, 260),
    Country::new(DJ, DJI, 262),
    Country::new(GA, GAB, 266),
    Country::new(GE, GEO, 268),
    Country::new(GM, GMB, 270),
    Country::new(PS, PSE, 275),
    Country::new(DE, DEU, 276),
    Country::new(GH, GHA, 288),
    Country::new(GI, GIB, 292),
    Country::new(KI, KIR, 296),
    Country::new(GR, GRC, 300),
    Country::new(GL, GRL, 304),
    Country::new(GD, GRD, 308),
    Country::new(GP, GLP, 312),
    Country::new(GU, GUM, 316),
    Country::new(GT, GTM, 320),
    Country::new(GN, GIN, 324),
    Country::new(GY, GUY, 328),
    Country::new(HT, HTI, 332),
    Country::new(HM, HMD, 334),
    Country::new(VA, VAT, 336),
    Country::new(HN, HND, 340),
    Country::new(HK, HKG, 344),
    Country::new(HU, HUN, 348),
    Country::new(IS, ISL, 352),
    Country::new(IN, IND, 356),
    Country::new(ID, IDN, 360),
    Country::new(IR, IRN, 364),
    Country::new(IQ, IRQ, 368),
    Country::new(IE, IRL, 372),
    Country::new(IL, ISR, 376),
    Country::new(IT, ITA, 380),
    Country::new(CI, CIV, 384),
    Country::new(JM, JAM, 388),
    Country::new(JP, JPN, 392),
    Country::new(KZ, KAZ, 398),
    Country::new(JO, JOR, 400),
    Country::new(KE, KEN, 404),
    Country::new(KP, PRK, 408),
    Country::new(KR, KOR, 410),
    Country::new(KW, KWT, 414),
    Country::new(KG, KGZ, 417),
    Country::new(LA, LAO, 418),
    Country::new(LB, LBN, 422),
    Country::new(LS, LSO, 426),
    Country::new(LV, LVA, 428),
    Country::new(LR, LBR, 430),
    Country::new(LY, LBY, 434),
    Country::new(LI, LIE, 438),
    Country::new(LT, LTU, 440),
    Country::new(LU, LUX, 442),
    Country::new(MO, MAC, 446),
    Country::new(MG, MDG, 450),
    Country::new(MW, MWI, 454),
    Country::new(MY, MYS, 458),
    Country::new(MV, MDV, 462),
    Country::new(ML, MLI, 466),
    Country::new(MT, MLT, 470),
    Country::new(MQ, MTQ, 474),
    Country::new(MR, MRT, 478),
    Country::new(MU, MUS, 480),
    Country::new(MX, MEX, 484),
    Country::new(MC, MCO, 492),
    Country::new(MN, MNG, 496),
    Country::new(MD, MDA, 498),
    Country::new(ME, MNE, 499),
    Country::new(MS, MSR, 500),
    Country::new(MA, MAR, 504),
    Country::new(MZ, MOZ, 508),
    Country::new(OM, OMN, 512),
    Country::new(NA, NAM, 516),
    Country::new(NR, NRU, 520),
    Country::new(NP, NPL, 524),
    Country::new(NL, NLD, 528),
    Country::new(CW, CUW, 531),
    Country::new(AW, ABW, 533),
    Country::new(SX, SXM, 534),
    Country::new(BQ, BES, 535),
    Country::new(NC, NCL, 540),
    Country::new(VU, VUT, 548),
    Country::new(NZ, NZL, 554),
    Country::new(NI, NIC, 558),
    Country::new(NE, NER, 562),
    Country::new(NG, NGA, 566),
    Country::new(NU, NIU, 570),
    Country::new(NF, NFK, 574),
    Country::new(NO, NOR, 578),
    Country::new(MP, MNP, 580),
    Country::new(UM, UMI, 581),
    Country::new(FM, FSM, 583),
    Country::new(MH, MHL, 584),
    Country::new(PW, PLW, 585),
    Country::new(PK, PAK, 586),
    Country::new(PA, PAN, 591),
    Country::new(PG, PNG, 598),
    Country::new(PY, PRY, 600),
    Country::new(PE, PER, 604),
    Country::new(PH, PHL, 608),
    Country::new(PN, PCN, 612),
    Country::new(PL, POL, 616),
    Country::new(PT, PRT, 620),
    Country::new(GW, GNB, 624),
    Country::new(TL, TLS, 626),
    Country::new(PR, PRI, 630),
    Country::new(QA, QAT, 634),
    Country::new(RE, REU, 638),
    Country::new(RO, ROU, 642),
    Country::new(RU, RUS, 643),
    Country::new(RW, RWA, 646),
    Country::new(BL, BLM, 652),
    Country::new(SH, SHN, 654),
    Country::new(KN, KNA, 659),
    Country::new(LC, LCA, 662),
    Country::new(AI, AIA, 660),
    Country::new(MF, MAF, 663),
    Country::new(PM, SPM, 666),
    Country::new(VC, VCT, 670),
    Country::new(SM, SMR, 674),
    Country::new(ST, STP, 678),
    Country::new(SA, SAU, 682),
    Country::new(SN, SEN, 686),
    Country::new(RS, SRB, 688),
    Country::new(SC, SYC, 690),
    Country::new(SL, SLE, 694),
    Country::new(SG, SGP, 702),
    Country::new(SK, SVK, 703),
    Country::new(VN, VNM, 704),
    Country::new(SI, SVN, 705),
    Country::new(SO, SOM, 706),
    Country::new(ZA, ZAF, 710),
    Country::new(ZW, ZWE, 716),
    Country::new(ES, ESP, 724),
    Country::new(SS, SSD, 728),
    Country::new(SD, SDN, 729),
    Country::new(EH, ESH, 732),
    Country::new(SR, SUR, 740),
    Country::new(SJ, SJM, 744),
    Country::new(SZ, SWZ, 748),
    Country::new(SE, SWE, 752),
    Country::new(CH, CHE, 756),
    Country::new(SY, SYR, 760),
    Country::new(TJ, TJK, 762),
    Country::new(TH, THA, 764),
    Country::new(TG, TGO, 768),
    Country::new(TK, TKL, 772),
    Country::new(TO, TON, 776),
    Country::new(AE, ARE, 784),
    Country::new(TN, TUN, 788),
    Country::new(TT, TTO, 780),
    Country::new(TR, TUR, 792),
    Country::new(TM, TKM, 795),
    Country::new(TC, TCA, 796),
    Country::new(TV, TUV, 798),
    Country::new(UG, UGA, 800),
    Country::new(UA, UKR, 804),
    Country::new(MK, MKD, 807),
    Country::new(EG, EGY, 818),
    Country::new(GB, GBR, 826),
    Country::new(GG, GGY, 831),
    Country::new(JE, JEY, 832),
    Country::new(IM, IMN, 833),
    Country::new(TZ, TZA, 834),
    Country::new(US, USA, 840),
    Country::new(VI, VIR, 850),
    Country::new(BF, BFA, 854),
    Country::new(UY, URY, 858),
    Country::new(UZ, UZB, 860),
    Country::new(VE, VEN, 862),
    Country::new(WF, WLF, 876),
    Country::new(WS, WSM, 882),
    Country::new(YE, YEM, 887),
    Country::new(ZM, ZMB, 894),
]);

impl From<CountryAlpha2> for Country {
    #[inline]
    fn from(value: CountryAlpha2) -> Self {
        COUNTRIES.country_by_alpha2(value)
    }
}

impl From<CountryAlpha3> for Country {
    #[inline]
    fn from(value: CountryAlpha3) -> Self {
        COUNTRIES.country_by_alpha3(value)
    }
}

impl TryFrom<u16> for Country {
    type Error = ();

    #[inline]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        COUNTRIES.country_by_numeric(value).ok_or(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromStr)]
pub enum CountryAlpha2 {
    AF,
    AL,
    AQ,
    DZ,
    AS,
    AD,
    AO,
    AG,
    AZ,
    AR,
    AU,
    AT,
    BS,
    BH,
    BD,
    AM,
    BB,
    BE,
    BM,
    BT,
    BO,
    BA,
    BW,
    BV,
    BR,
    BZ,
    IO,
    SB,
    VG,
    BN,
    BG,
    MM,
    BI,
    BY,
    KH,
    CM,
    CA,
    CV,
    KY,
    CF,
    LK,
    TD,
    CL,
    CN,
    TW,
    CX,
    CC,
    CO,
    KM,
    YT,
    CG,
    CD,
    CK,
    CR,
    HR,
    CU,
    CY,
    CZ,
    BJ,
    DK,
    DM,
    DO,
    EC,
    SV,
    GQ,
    ER,
    ET,
    EE,
    FO,
    FK,
    GS,
    FJ,
    FI,
    AX,
    FR,
    GF,
    PF,
    TF,
    DJ,
    GA,
    GE,
    GM,
    PS,
    DE,
    GH,
    GI,
    KI,
    GR,
    GL,
    GD,
    GP,
    GU,
    GT,
    GN,
    GY,
    HT,
    HM,
    VA,
    HN,
    HK,
    HU,
    IS,
    IN,
    ID,
    IR,
    IQ,
    IE,
    IL,
    IT,
    CI,
    JM,
    JP,
    KZ,
    JO,
    KE,
    KP,
    KR,
    KW,
    KG,
    LA,
    LB,
    LS,
    LV,
    LR,
    LY,
    LI,
    LT,
    LU,
    MO,
    MG,
    MW,
    MY,
    MV,
    ML,
    MT,
    MQ,
    MR,
    MU,
    MX,
    MC,
    MN,
    MD,
    ME,
    MS,
    MA,
    MZ,
    OM,
    NA,
    NR,
    NP,
    NL,
    CW,
    AW,
    SX,
    BQ,
    NC,
    VU,
    NZ,
    NI,
    NE,
    NG,
    NU,
    NF,
    NO,
    MP,
    UM,
    FM,
    MH,
    PW,
    PK,
    PA,
    PG,
    PY,
    PE,
    PH,
    PN,
    PL,
    PT,
    GW,
    TL,
    PR,
    QA,
    RE,
    RO,
    RU,
    RW,
    BL,
    SH,
    KN,
    LC,
    AI,
    MF,
    PM,
    VC,
    SM,
    ST,
    SA,
    SN,
    RS,
    SC,
    SL,
    SG,
    SK,
    VN,
    SI,
    SO,
    ZA,
    ZW,
    ES,
    SS,
    SD,
    EH,
    SR,
    SJ,
    SZ,
    SE,
    CH,
    SY,
    TJ,
    TH,
    TG,
    TK,
    TO,
    AE,
    TN,
    TT,
    TR,
    TM,
    TC,
    TV,
    UG,
    UA,
    MK,
    EG,
    GB,
    GG,
    JE,
    IM,
    TZ,
    US,
    VI,
    BF,
    UY,
    UZ,
    VE,
    WF,
    WS,
    YE,
    ZM,
}

impl CountryAlpha2 {
    #[inline]
    pub(crate) const fn internal_offset(&self) -> usize {
        *self as usize
    }
}

impl From<CountryAlpha3> for CountryAlpha2 {
    #[inline]
    fn from(value: CountryAlpha3) -> Self {
        COUNTRIES.country_by_alpha3(value).alpha2()
    }
}

impl From<CountryAlpha2> for u16 {
    #[inline]
    fn from(value: CountryAlpha2) -> Self {
        COUNTRIES.country_by_alpha2(value).numeric()
    }
}

impl TryFrom<u16> for CountryAlpha2 {
    type Error = ();

    #[inline]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        COUNTRIES.country_by_numeric(value)
        .map(|country| country.alpha2())
        .ok_or(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromStr)]
pub enum CountryAlpha3 {
    AFG,
    ALB,
    ATA,
    DZA,
    ASM,
    AND,
    AGO,
    ATG,
    AZE,
    ARG,
    AUS,
    AUT,
    BHS,
    BHR,
    BGD,
    ARM,
    BRB,
    BEL,
    BMU,
    BTN,
    BOL,
    BIH,
    BWA,
    BVT,
    BRA,
    BLZ,
    IOT,
    SLB,
    VGB,
    BRN,
    BGR,
    MMR,
    BDI,
    BLR,
    KHM,
    CMR,
    CAN,
    CPV,
    CYM,
    CAF,
    LKA,
    TCD,
    CHL,
    CHN,
    TWN,
    CXR,
    CCK,
    COL,
    COM,
    MYT,
    COG,
    COD,
    COK,
    CRI,
    HRV,
    CUB,
    CYP,
    CZE,
    BEN,
    DNK,
    DMA,
    DOM,
    ECU,
    SLV,
    GNQ,
    ERI,
    ETH,
    EST,
    FRO,
    FLK,
    SGS,
    FJI,
    FIN,
    ALA,
    FRA,
    GUF,
    PYF,
    ATF,
    DJI,
    GAB,
    GEO,
    GMB,
    PSE,
    DEU,
    GHA,
    GIB,
    KIR,
    GRC,
    GRL,
    GRD,
    GLP,
    GUM,
    GTM,
    GIN,
    GUY,
    HTI,
    HMD,
    VAT,
    HND,
    HKG,
    HUN,
    ISL,
    IND,
    IDN,
    IRN,
    IRQ,
    IRL,
    ISR,
    ITA,
    CIV,
    JAM,
    JPN,
    KAZ,
    JOR,
    KEN,
    PRK,
    KOR,
    KWT,
    KGZ,
    LAO,
    LBN,
    LSO,
    LVA,
    LBR,
    LBY,
    LIE,
    LTU,
    LUX,
    MAC,
    MDG,
    MWI,
    MYS,
    MDV,
    MLI,
    MLT,
    MTQ,
    MRT,
    MUS,
    MEX,
    MCO,
    MNG,
    MDA,
    MNE,
    MSR,
    MAR,
    MOZ,
    OMN,
    NAM,
    NRU,
    NPL,
    NLD,
    CUW,
    ABW,
    SXM,
    BES,
    NCL,
    VUT,
    NZL,
    NIC,
    NER,
    NGA,
    NIU,
    NFK,
    NOR,
    MNP,
    UMI,
    FSM,
    MHL,
    PLW,
    PAK,
    PAN,
    PNG,
    PRY,
    PER,
    PHL,
    PCN,
    POL,
    PRT,
    GNB,
    TLS,
    PRI,
    QAT,
    REU,
    ROU,
    RUS,
    RWA,
    BLM,
    SHN,
    KNA,
    LCA,
    AIA,
    MAF,
    SPM,
    VCT,
    SMR,
    STP,
    SAU,
    SEN,
    SRB,
    SYC,
    SLE,
    SGP,
    SVK,
    VNM,
    SVN,
    SOM,
    ZAF,
    ZWE,
    ESP,
    SSD,
    SDN,
    ESH,
    SUR,
    SJM,
    SWZ,
    SWE,
    CHE,
    SYR,
    TJK,
    THA,
    TGO,
    TKL,
    TON,
    ARE,
    TUN,
    TTO,
    TUR,
    TKM,
    TCA,
    TUV,
    UGA,
    UKR,
    MKD,
    EGY,
    GBR,
    GGY,
    JEY,
    IMN,
    TZA,
    USA,
    VIR,
    BFA,
    URY,
    UZB,
    VEN,
    WLF,
    WSM,
    YEM,
    ZMB,
}

impl CountryAlpha3 {
    #[inline]
    pub(crate) const fn internal_offset(&self) -> usize {
        *self as usize
    }
}

impl From<CountryAlpha2> for CountryAlpha3 {
    #[inline]
    fn from(value: CountryAlpha2) -> Self {
        COUNTRIES.country_by_alpha2(value).alpha3()
    }
}

impl From<CountryAlpha3> for u16 {
    #[inline]
    fn from(value: CountryAlpha3) -> Self {
        COUNTRIES.country_by_alpha3(value).numeric()
    }
}

impl TryFrom<u16> for CountryAlpha3 {
    type Error = ();

    #[inline]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        COUNTRIES.country_by_numeric(value)
        .map(|country| country.alpha3())
        .ok_or(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate alloc;
    use alloc::string::ToString;

    use core::str::FromStr;

    #[test]
    fn test_alpha2_from_alpha3() {
        let alpha3 = CountryAlpha3::AND;
        let alpha2 = CountryAlpha2::from(alpha3);
        assert_eq!(alpha2, CountryAlpha2::AD);
    }

    #[test]
    fn test_alpha2_from_numeric() {
        let alpha2 = CountryAlpha2::AD;
        let numeric = Country::from(alpha2).numeric();
        let alpha2_from_numeric = CountryAlpha2::try_from(numeric).unwrap();
        assert_eq!(alpha2, alpha2_from_numeric);
    }

    #[test]
    fn test_alpha3_from_alpha2() {
        let alpha2 = CountryAlpha2::AD;
        let alpha3 = CountryAlpha3::from(alpha2);
        assert_eq!(alpha3, CountryAlpha3::AND);
    }

    #[test]
    fn test_alpha3_from_numeric() {
        let alpha3 = CountryAlpha3::AND;
        let numeric = Country::from(alpha3).numeric();
        let alpha3_from_numeric = CountryAlpha3::try_from(numeric).unwrap();
        assert_eq!(alpha3, alpha3_from_numeric);
    }

    #[test]
    fn test_numeric_from_alpha2() {
        let alpha2 = CountryAlpha2::AD;
        let numeric = u16::from(alpha2);
        assert_eq!(numeric, 020);
    }

    #[test]
    fn test_numeric_from_alpha3() {
        let alpha3 = CountryAlpha3::AND;
        let numeric = u16::from(alpha3);
        assert_eq!(numeric, 020);
    }

    #[test]
    fn test_country_from_alpha2() {
        let alpha2 = CountryAlpha2::AD;
        let country = Country::from(alpha2);
        assert_eq!(country.alpha2(), alpha2);
    }

    #[test]
    fn test_country_from_alpha3() {
        let alpha3 = CountryAlpha3::AND;
        let country = Country::from(alpha3);
        assert_eq!(country.alpha3(), alpha3);
    }

    #[test]
    fn test_country_from_numeric() {
        let numeric = 020;
        let country = Country::try_from(numeric).unwrap();
        assert_eq!(country.numeric(), numeric);
    }

    #[test]
    fn test_alpha2_from_str() {
        let alpha2 = CountryAlpha2::AD;
        let alpha2_from_str = CountryAlpha2::from_str("AD").unwrap();
        assert_eq!(alpha2, alpha2_from_str);
    }

    #[test]
    fn test_alpha3_from_str() {
        let alpha3 = CountryAlpha3::AND;
        let alpha3_from_str = CountryAlpha3::from_str("AND").unwrap();
        assert_eq!(alpha3, alpha3_from_str);
    }

    #[test]
    fn test_alpha2_to_string() {
        let alpha2 = CountryAlpha2::AD;
        let alpha2_string = alpha2.to_string();
        assert_eq!(alpha2_string, "AD");
    }

    #[test]
    fn test_alpha3_to_string() {
        let alpha3 = CountryAlpha3::AND;
        let alpha3_string = alpha3.to_string();
        assert_eq!(alpha3_string, "AND");
    }
}
