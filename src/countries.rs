use num_enum::TryFromPrimitive;
use parse_display::{Display, FromStr};

use super::Country;

impl From<CountryAlpha2> for Country {
    fn from(value: CountryAlpha2) -> Self {
        let alpha2 = value;

        let short_name = match alpha2 {
            CountryAlpha2::AD => "Andorra",
            CountryAlpha2::AE => "United Arab Emirates",
            CountryAlpha2::AF => "Afghanistan",
            CountryAlpha2::AG => "Antigua and Barbuda",
            CountryAlpha2::AI => "Anguilla",
            CountryAlpha2::AL => "Albania",
            CountryAlpha2::AM => "Armenia",
            CountryAlpha2::AO => "Angola",
            CountryAlpha2::AQ => "Antarctica",
            CountryAlpha2::AR => "Argentina",
            CountryAlpha2::AS => "American Samoa",
            CountryAlpha2::AT => "Austria",
            CountryAlpha2::AU => "Australia",
            CountryAlpha2::AW => "Aruba",
            CountryAlpha2::AX => "Åland Islands",
            CountryAlpha2::AZ => "Azerbaijan",
            CountryAlpha2::BA => "Bosnia and Herzegovina",
            CountryAlpha2::BB => "Barbados",
            CountryAlpha2::BD => "Bangladesh",
            CountryAlpha2::BE => "Belgium",
            CountryAlpha2::BF => "Burkina Faso",
            CountryAlpha2::BG => "Bulgaria",
            CountryAlpha2::BH => "Bahrain",
            CountryAlpha2::BI => "Burundi",
            CountryAlpha2::BJ => "Benin",
            CountryAlpha2::BL => "Saint Barthélemy",
            CountryAlpha2::BM => "Bermuda",
            CountryAlpha2::BN => "Brunei Darussalam",
            CountryAlpha2::BO => "Bolivia (Plurinational State of)",
            CountryAlpha2::BQ => "Bonaire, Sint Eustatius and Saba",
            CountryAlpha2::BR => "Brazil",
            CountryAlpha2::BS => "Bahamas",
            CountryAlpha2::BT => "Bhutan",
            CountryAlpha2::BV => "Bouvet Island",
            CountryAlpha2::BW => "Botswana",
            CountryAlpha2::BY => "Belarus",
            CountryAlpha2::BZ => "Belize",
            CountryAlpha2::CA => "Canada",
            CountryAlpha2::CC => "Cocos (Keeling) Islands",
            CountryAlpha2::CD => "Congo, Democratic Republic of the",
            CountryAlpha2::CF => "Central African Republic",
            CountryAlpha2::CG => "Congo",
            CountryAlpha2::CH => "Switzerland",
            CountryAlpha2::CI => "Côte d'Ivoire",
            CountryAlpha2::CK => "Cook Islands",
            CountryAlpha2::CL => "Chile",
            CountryAlpha2::CM => "Cameroon",
            CountryAlpha2::CN => "China",
            CountryAlpha2::CO => "Colombia",
            CountryAlpha2::CR => "Costa Rica",
            CountryAlpha2::CU => "Cuba",
            CountryAlpha2::CV => "Cabo Verde",
            CountryAlpha2::CW => "Curaçao",
            CountryAlpha2::CX => "Christmas Island",
            CountryAlpha2::CY => "Cyprus",
            CountryAlpha2::CZ => "Czechia",
            CountryAlpha2::DE => "Germany",
            CountryAlpha2::DJ => "Djibouti",
            CountryAlpha2::DK => "Denmark",
            CountryAlpha2::DM => "Dominica",
            CountryAlpha2::DO => "Dominican Republic",
            CountryAlpha2::DZ => "Algeria",
            CountryAlpha2::EC => "Ecuador",
            CountryAlpha2::EE => "Estonia",
            CountryAlpha2::EG => "Egypt",
            CountryAlpha2::EH => "Western Sahara",
            CountryAlpha2::ER => "Eritrea",
            CountryAlpha2::ES => "Spain",
            CountryAlpha2::ET => "Ethiopia",
            CountryAlpha2::FI => "Finland",
            CountryAlpha2::FJ => "Fiji",
            CountryAlpha2::FK => "Falkland Islands (Malvinas)",
            CountryAlpha2::FM => "Micronesia (Federated States of)",
            CountryAlpha2::FO => "Faroe Islands",
            CountryAlpha2::FR => "France",
            CountryAlpha2::GA => "Gabon",
            CountryAlpha2::GB => "United Kingdom of Great Britain and Northern Ireland",
            CountryAlpha2::GD => "Grenada",
            CountryAlpha2::GE => "Georgia",
            CountryAlpha2::GF => "French Guiana",
            CountryAlpha2::GG => "Guernsey",
            CountryAlpha2::GH => "Ghana",
            CountryAlpha2::GI => "Gibraltar",
            CountryAlpha2::GL => "Greenland",
            CountryAlpha2::GM => "Gambia",
            CountryAlpha2::GN => "Guinea",
            CountryAlpha2::GP => "Guadeloupe",
            CountryAlpha2::GQ => "Equatorial Guinea",
            CountryAlpha2::GR => "Greece",
            CountryAlpha2::GS => "South Georgia and the South Sandwich Islands",
            CountryAlpha2::GT => "Guatemala",
            CountryAlpha2::GU => "Guam",
            CountryAlpha2::GW => "Guinea-Bissau",
            CountryAlpha2::GY => "Guyana",
            CountryAlpha2::HK => "Hong Kong",
            CountryAlpha2::HM => "Heard Island and McDonald Islands",
            CountryAlpha2::HN => "Honduras",
            CountryAlpha2::HR => "Croatia",
            CountryAlpha2::HT => "Haiti",
            CountryAlpha2::HU => "Hungary",
            CountryAlpha2::ID => "Indonesia",
            CountryAlpha2::IE => "Ireland",
            CountryAlpha2::IL => "Israel",
            CountryAlpha2::IM => "Isle of Man",
            CountryAlpha2::IN => "India",
            CountryAlpha2::IO => "British Indian Ocean Territory",
            CountryAlpha2::IQ => "Iraq",
            CountryAlpha2::IR => "Iran (Islamic Republic of)",
            CountryAlpha2::IS => "Iceland",
            CountryAlpha2::IT => "Italy",
            CountryAlpha2::JE => "Jersey",
            CountryAlpha2::JM => "Jamaica",
            CountryAlpha2::JO => "Jordan",
            CountryAlpha2::JP => "Japan",
            CountryAlpha2::KE => "Kenya",
            CountryAlpha2::KG => "Kyrgyzstan",
            CountryAlpha2::KH => "Cambodia",
            CountryAlpha2::KI => "Kiribati",
            CountryAlpha2::KM => "Comoros",
            CountryAlpha2::KN => "Saint Kitts and Nevis",
            CountryAlpha2::KP => "Korea (Democratic People's Republic of)",
            CountryAlpha2::KR => "Korea, Republic of",
            CountryAlpha2::KW => "Kuwait",
            CountryAlpha2::KY => "Cayman Islands",
            CountryAlpha2::KZ => "Kazakhstan",
            CountryAlpha2::LA => "Laos",
            CountryAlpha2::LB => "Lebanon",
            CountryAlpha2::LC => "Saint Lucia",
            CountryAlpha2::LI => "Liechtenstein",
            CountryAlpha2::LK => "Sri Lanka",
            CountryAlpha2::LR => "Liberia",
            CountryAlpha2::LS => "Lesotho",
            CountryAlpha2::LT => "Lithuania",
            CountryAlpha2::LU => "Luxembourg",
            CountryAlpha2::LV => "Latvia",
            CountryAlpha2::LY => "Libya",
            CountryAlpha2::MA => "Morocco",
            CountryAlpha2::MC => "Monaco",
            CountryAlpha2::MD => "Moldova, Republic of",
            CountryAlpha2::ME => "Montenegro",
            CountryAlpha2::MF => "Saint Martin (French part)",
            CountryAlpha2::MG => "Madagascar",
            CountryAlpha2::MH => "Marshall Islands",
            CountryAlpha2::MK => "North Macedonia",
            CountryAlpha2::ML => "Mali",
            CountryAlpha2::MM => "Myanmar",
            CountryAlpha2::MN => "Mongolia",
            CountryAlpha2::MO => "Macao",
            CountryAlpha2::MP => "Northern Mariana Islands",
            CountryAlpha2::MQ => "Martinique",
            CountryAlpha2::MR => "Mauritania",
            CountryAlpha2::MS => "Montserrat",
            CountryAlpha2::MT => "Malta",
            CountryAlpha2::MU => "Mauritius",
            CountryAlpha2::MV => "Maldives",
            CountryAlpha2::MW => "Malawi",
            CountryAlpha2::MX => "Mexico",
            CountryAlpha2::MY => "Malaysia",
            CountryAlpha2::MZ => "Mozambique",
            CountryAlpha2::NA => "Namibia",
            CountryAlpha2::NC => "New Caledonia",
            CountryAlpha2::NE => "Niger",
            CountryAlpha2::NF => "Norfolk Island",
            CountryAlpha2::NG => "Nigeria",
            CountryAlpha2::NI => "Nicaragua",
            CountryAlpha2::NL => "Netherlands",
            CountryAlpha2::NO => "Norway",
            CountryAlpha2::NP => "Nepal",
            CountryAlpha2::NR => "Nauru",
            CountryAlpha2::NU => "Niue",
            CountryAlpha2::NZ => "New Zealand",
            CountryAlpha2::OM => "Oman",
            CountryAlpha2::PA => "Panama",
            CountryAlpha2::PE => "Peru",
            CountryAlpha2::PF => "French Polynesia",
            CountryAlpha2::PG => "Papua New Guinea",
            CountryAlpha2::PH => "Philippines",
            CountryAlpha2::PK => "Pakistan",
            CountryAlpha2::PL => "Poland",
            CountryAlpha2::PM => "Saint Pierre and Miquelon",
            CountryAlpha2::PN => "Pitcairn",
            CountryAlpha2::PR => "Puerto Rico",
            CountryAlpha2::PS => "Palestine, State of",
            CountryAlpha2::PT => "Portugal",
            CountryAlpha2::PW => "Palau",
            CountryAlpha2::PY => "Paraguay",
            CountryAlpha2::QA => "Qatar",
            CountryAlpha2::RE => "Réunion",
            CountryAlpha2::RO => "Romania",
            CountryAlpha2::RS => "Serbia",
            CountryAlpha2::RU => "Russia",
            CountryAlpha2::RW => "Rwanda",
            CountryAlpha2::SA => "Saudi Arabia",
            CountryAlpha2::SB => "Solomon Islands",
            CountryAlpha2::SC => "Seychelles",
            CountryAlpha2::SD => "Sudan",
            CountryAlpha2::SE => "Sweden",
            CountryAlpha2::SG => "Singapore",
            CountryAlpha2::SH => "Saint Helena, Ascension and Tristan da Cunha",
            CountryAlpha2::SI => "Slovenia",
            CountryAlpha2::SJ => "Svalbard and Jan Mayen",
            CountryAlpha2::SK => "Slovakia",
            CountryAlpha2::SL => "Sierra Leone",
            CountryAlpha2::SM => "San Marino",
            CountryAlpha2::SN => "Senegal",
            CountryAlpha2::SO => "Somalia",
            CountryAlpha2::SR => "Suriname",
            CountryAlpha2::SS => "South Sudan",
            CountryAlpha2::ST => "Sao Tome and Principe",
            CountryAlpha2::SV => "El Salvador",
            CountryAlpha2::SX => "Sint Maarten (Dutch part)",
            CountryAlpha2::SY => "Syrian Arab Republic",
            CountryAlpha2::SZ => "Eswatini",
            CountryAlpha2::TC => "Turks and Caicos Islands",
            CountryAlpha2::TD => "Chad",
            CountryAlpha2::TF => "French Southern Territories",
            CountryAlpha2::TG => "Togo",
            CountryAlpha2::TH => "Thailand",
            CountryAlpha2::TJ => "Tajikistan",
            CountryAlpha2::TK => "Tokelau",
            CountryAlpha2::TL => "Timor-Leste",
            CountryAlpha2::TM => "Turkmenistan",
            CountryAlpha2::TN => "Tunisia",
            CountryAlpha2::TO => "Tonga",
            CountryAlpha2::TR => "Turkey",
            CountryAlpha2::TT => "Trinidad and Tobago",
            CountryAlpha2::TV => "Tuvalu",
            CountryAlpha2::TW => "Taiwan, Province of China",
            CountryAlpha2::TZ => "Tanzania, United Republic of",
            CountryAlpha2::UA => "Ukraine",
            CountryAlpha2::UG => "Uganda",
            CountryAlpha2::UM => "United States Minor Outlying Islands",
            CountryAlpha2::US => "United States of America",
            CountryAlpha2::UY => "Uruguay",
            CountryAlpha2::UZ => "Uzbekistan",
            CountryAlpha2::VA => "Holy See",
            CountryAlpha2::VC => "Saint Vincent and the Grenadines",
            CountryAlpha2::VE => "Venezuela (Bolivarian Republic of)",
            CountryAlpha2::VG => "Virgin Islands (British)",
            CountryAlpha2::VI => "Virgin Islands (U.S.)",
            CountryAlpha2::VN => "Viet Nam",
            CountryAlpha2::VU => "Vanuatu",
            CountryAlpha2::WF => "Wallis and Futuna",
            CountryAlpha2::WS => "Samoa",
            CountryAlpha2::YE => "Yemen",
            CountryAlpha2::YT => "Mayotte",
            CountryAlpha2::ZA => "South Africa",
            CountryAlpha2::ZM => "Zambia",
            CountryAlpha2::ZW => "Zimbabwe",
        };

        Self::new(alpha2, short_name)
    }
}

impl From<CountryAlpha3> for Country {
    fn from(value: CountryAlpha3) -> Self {
        let alpha2: CountryAlpha2 = value.into();
        alpha2.into()
    }
}

impl TryFrom<u16> for Country {
    type Error = num_enum::TryFromPrimitiveError<CountryAlpha2>;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let alpha2 = CountryAlpha2::try_from(value)?;

        Ok(alpha2.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive, Display, FromStr)]
#[repr(u16)]
pub enum CountryAlpha2 {
    AD = 020,
    AE = 784,
    AF = 004,
    AG = 028,
    AI = 660,
    AL = 008,
    AM = 051,
    AO = 024,
    AQ = 010,
    AR = 032,
    AS = 016,
    AT = 040,
    AU = 036,
    AW = 533,
    AX = 248,
    AZ = 031,
    BA = 070,
    BB = 052,
    BD = 050,
    BE = 056,
    BF = 854,
    BG = 100,
    BH = 048,
    BI = 108,
    BJ = 204,
    BL = 652,
    BM = 060,
    BN = 096,
    BO = 068,
    BQ = 535,
    BR = 076,
    BS = 044,
    BT = 064,
    BV = 074,
    BW = 072,
    BY = 112,
    BZ = 084,
    CA = 124,
    CC = 166,
    CD = 180,
    CF = 140,
    CG = 178,
    CH = 756,
    CI = 384,
    CK = 184,
    CL = 152,
    CM = 120,
    CN = 156,
    CO = 170,
    CR = 188,
    CU = 192,
    CV = 132,
    CW = 531,
    CX = 162,
    CY = 196,
    CZ = 203,
    DE = 276,
    DJ = 262,
    DK = 208,
    DM = 212,
    DO = 214,
    DZ = 012,
    EC = 218,
    EE = 233,
    EG = 818,
    EH = 732,
    ER = 232,
    ES = 724,
    ET = 231,
    FI = 246,
    FJ = 242,
    FK = 238,
    FM = 583,
    FO = 234,
    FR = 250,
    GA = 266,
    GB = 826,
    GD = 308,
    GE = 268,
    GF = 254,
    GG = 831,
    GH = 288,
    GI = 292,
    GL = 304,
    GM = 270,
    GN = 324,
    GP = 312,
    GQ = 226,
    GR = 300,
    GS = 239,
    GT = 320,
    GU = 316,
    GW = 624,
    GY = 328,
    HK = 344,
    HM = 334,
    HN = 340,
    HR = 191,
    HT = 332,
    HU = 348,
    ID = 360,
    IE = 372,
    IL = 376,
    IM = 833,
    IN = 356,
    IO = 086,
    IQ = 368,
    IR = 364,
    IS = 352,
    IT = 380,
    JE = 832,
    JM = 388,
    JO = 400,
    JP = 392,
    KE = 404,
    KG = 417,
    KH = 116,
    KI = 296,
    KM = 174,
    KN = 659,
    KP = 408,
    KR = 410,
    KW = 414,
    KY = 136,
    KZ = 398,
    LA = 418,
    LB = 422,
    LC = 662,
    LI = 438,
    LK = 144,
    LR = 430,
    LS = 426,
    LT = 440,
    LU = 442,
    LV = 428,
    LY = 434,
    MA = 504,
    MC = 492,
    MD = 498,
    ME = 499,
    MF = 663,
    MG = 450,
    MH = 584,
    MK = 807,
    ML = 466,
    MM = 104,
    MN = 496,
    MO = 446,
    MP = 580,
    MQ = 474,
    MR = 478,
    MS = 500,
    MT = 470,
    MU = 480,
    MV = 462,
    MW = 454,
    MX = 484,
    MY = 458,
    MZ = 508,
    NA = 516,
    NC = 540,
    NE = 562,
    NF = 574,
    NG = 566,
    NI = 558,
    NL = 528,
    NO = 578,
    NP = 524,
    NR = 520,
    NU = 570,
    NZ = 554,
    OM = 512,
    PA = 591,
    PE = 604,
    PF = 258,
    PG = 598,
    PH = 608,
    PK = 586,
    PL = 616,
    PM = 666,
    PN = 612,
    PR = 630,
    PS = 275,
    PT = 620,
    PW = 585,
    PY = 600,
    QA = 634,
    RE = 638,
    RO = 642,
    RS = 688,
    RU = 643,
    RW = 646,
    SA = 682,
    SB = 090,
    SC = 690,
    SD = 729,
    SE = 752,
    SG = 702,
    SH = 654,
    SI = 705,
    SJ = 744,
    SK = 703,
    SL = 694,
    SM = 674,
    SN = 686,
    SO = 706,
    SR = 740,
    SS = 728,
    ST = 678,
    SV = 222,
    SX = 534,
    SY = 760,
    SZ = 748,
    TC = 796,
    TD = 148,
    TF = 260,
    TG = 768,
    TH = 764,
    TJ = 762,
    TK = 772,
    TL = 626,
    TM = 795,
    TN = 788,
    TO = 776,
    TR = 792,
    TT = 780,
    TV = 798,
    TW = 158,
    TZ = 834,
    UA = 804,
    UG = 800,
    UM = 581,
    US = 840,
    UY = 858,
    UZ = 860,
    VA = 336,
    VC = 670,
    VE = 862,
    VG = 092,
    VI = 850,
    VN = 704,
    VU = 548,
    WF = 876,
    WS = 882,
    YE = 887,
    YT = 175,
    ZA = 710,
    ZM = 894,
    ZW = 716,
}

impl From<CountryAlpha3> for CountryAlpha2 {
    fn from(value: CountryAlpha3) -> Self {
        let Ok(alpha2) = Self::try_from(value as u16) else {
            unreachable!("CountryAlpha3 does not match any CountryAlpha2, but this should never happen")
        };

        alpha2
    }
}

impl From<CountryAlpha2> for u16 {
    #[inline]
    fn from(value: CountryAlpha2) -> Self {
        value as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive, Display, FromStr)]
#[repr(u16)]
pub enum CountryAlpha3 {
    AND = 020,
    ARE = 784,
    AFG = 004,
    ATG = 028,
    AIA = 660,
    ALB = 008,
    ARM = 051,
    AGO = 024,
    ATA = 010,
    ARG = 032,
    ASM = 016,
    AUT = 040,
    AUS = 036,
    ABW = 533,
    ALA = 248,
    AZE = 031,
    BIH = 070,
    BRB = 052,
    BGD = 050,
    BEL = 056,
    BFA = 854,
    BGR = 100,
    BHR = 048,
    BDI = 108,
    BEN = 204,
    BLM = 652,
    BMU = 060,
    BRN = 096,
    BOL = 068,
    BES = 535,
    BRA = 076,
    BHS = 044,
    BTN = 064,
    BVT = 074,
    BWA = 072,
    BLR = 112,
    BLZ = 084,
    CAN = 124,
    CCK = 166,
    COD = 180,
    CAF = 140,
    COG = 178,
    CHE = 756,
    CIV = 384,
    COK = 184,
    CHL = 152,
    CMR = 120,
    CHN = 156,
    COL = 170,
    CRI = 188,
    CUB = 192,
    CPV = 132,
    CUW = 531,
    CXR = 162,
    CYP = 196,
    CZE = 203,
    DEU = 276,
    DJI = 262,
    DNK = 208,
    DMA = 212,
    DOM = 214,
    DZA = 012,
    ECU = 218,
    EST = 233,
    EGY = 818,
    ESH = 732,
    ERI = 232,
    ESP = 724,
    ETH = 231,
    FIN = 246,
    FJI = 242,
    FLK = 238,
    FSM = 583,
    FRO = 234,
    FRA = 250,
    GAB = 266,
    GBR = 826,
    GRD = 308,
    GEO = 268,
    GUF = 254,
    GGY = 831,
    GHA = 288,
    GIB = 292,
    GRL = 304,
    GMB = 270,
    GIN = 324,
    GLP = 312,
    GNQ = 226,
    GRC = 300,
    SGS = 239,
    GTM = 320,
    GUM = 316,
    GNB = 624,
    GUY = 328,
    HKG = 344,
    HMD = 334,
    HND = 340,
    HRV = 191,
    HTI = 332,
    HUN = 348,
    IDN = 360,
    IRL = 372,
    ISR = 376,
    IMN = 833,
    IND = 356,
    IOT = 086,
    IRQ = 368,
    IRN = 364,
    ISL = 352,
    ITA = 380,
    JEY = 832,
    JAM = 388,
    JOR = 400,
    JPN = 392,
    KEN = 404,
    KGZ = 417,
    KHM = 116,
    KIR = 296,
    COM = 174,
    KNA = 659,
    PRK = 408,
    KOR = 410,
    KWT = 414,
    CYM = 136,
    KAZ = 398,
    LAO = 418,
    LBN = 422,
    LCA = 662,
    LIE = 438,
    LKA = 144,
    LBR = 430,
    LSO = 426,
    LTU = 440,
    LUX = 442,
    LVA = 428,
    LBY = 434,
    MAR = 504,
    MCO = 492,
    MDA = 498,
    MNE = 499,
    MAF = 663,
    MDG = 450,
    MHL = 584,
    MKD = 807,
    MLI = 466,
    MMR = 104,
    MNG = 496,
    MAC = 446,
    MNP = 580,
    MTQ = 474,
    MRT = 478,
    MSR = 500,
    MLT = 470,
    MUS = 480,
    MDV = 462,
    MWI = 454,
    MEX = 484,
    MYS = 458,
    MOZ = 508,
    NAM = 516,
    NCL = 540,
    NER = 562,
    NFK = 574,
    NGA = 566,
    NIC = 558,
    NLD = 528,
    NOR = 578,
    NPL = 524,
    NRU = 520,
    NIU = 570,
    NZL = 554,
    OMN = 512,
    PAN = 591,
    PER = 604,
    PYF = 258,
    PNG = 598,
    PHL = 608,
    PAK = 586,
    POL = 616,
    SPM = 666,
    PCN = 612,
    PRI = 630,
    PSE = 275,
    PRT = 620,
    PLW = 585,
    PRY = 600,
    QAT = 634,
    REU = 638,
    ROU = 642,
    SRB = 688,
    RUS = 643,
    RWA = 646,
    SAU = 682,
    SLB = 090,
    SYC = 690,
    SDN = 729,
    SWE = 752,
    SGP = 702,
    SHN = 654,
    SVN = 705,
    SJM = 744,
    SVK = 703,
    SLE = 694,
    SMR = 674,
    SEN = 686,
    SOM = 706,
    SUR = 740,
    SSD = 728,
    STP = 678,
    SLV = 222,
    SXM = 534,
    SYR = 760,
    SWZ = 748,
    TCA = 796,
    TCD = 148,
    ATF = 260,
    TGO = 768,
    THA = 764,
    TJK = 762,
    TKL = 772,
    TLS = 626,
    TKM = 795,
    TUN = 788,
    TON = 776,
    TUR = 792,
    TTO = 780,
    TUV = 798,
    TWN = 158,
    TZA = 834,
    UKR = 804,
    UGA = 800,
    UMI = 581,
    USA = 840,
    URY = 858,
    UZB = 860,
    VAT = 336,
    VCT = 670,
    VEN = 862,
    VGB = 092,
    VIR = 850,
    VNM = 704,
    VUT = 548,
    WLF = 876,
    WSM = 882,
    YEM = 887,
    MYT = 175,
    ZAF = 710,
    ZMB = 894,
    ZWE = 716,
}

impl From<CountryAlpha2> for CountryAlpha3 {
    #[inline]
    fn from(value: CountryAlpha2) -> Self {
        let Ok(alpha3) = Self::try_from(value as u16) else {
            unreachable!("CountryAlpha2 does not match any CountryAlpha3, but this should never happen")
        };

        alpha3
    }
}

impl From<CountryAlpha3> for u16 {
    #[inline]
    fn from(value: CountryAlpha3) -> Self {
        value as u16
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
        let numeric = alpha2 as u16;
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
        let numeric = alpha3 as u16;
        let alpha3_from_numeric = CountryAlpha3::try_from(numeric).unwrap();
        assert_eq!(alpha3, alpha3_from_numeric);
    }

    #[test]
    fn test_numeric_from_alpha2() {
        let alpha2 = CountryAlpha2::AD;
        let numeric = alpha2 as u16;
        assert_eq!(numeric, 020);
    }

    #[test]
    fn test_numeric_from_alpha3() {
        let alpha3 = CountryAlpha3::AND;
        let numeric = alpha3 as u16;
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
