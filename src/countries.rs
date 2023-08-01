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
        if numeric > 999 {
            return None;
        }

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
        self.countries[alpha2 as usize]
    }

    #[inline]
    pub const fn country_by_alpha3(&self, alpha3: CountryAlpha3) -> Country {
        self.countries[alpha3 as usize]
    }
}

// NOTE: This is static as that reduces the size of the binary and makes it faster.
//       Order is important, as it is used for binary search by the `Country` numeric.
//       Also, alpha 2 and alpha 3 codes are sorted alphabetically but the values are
//       offsets into the array for fast lookup from the `CountryAlpha2` and `CountryAlpha3`
//       enums.
static COUNTRIES: CountriesList = CountriesList::new(&[
    Country::new(AF, AFG, 004, #[cfg(feature = "short-names")] "Afghanistan"),
    Country::new(AL, ALB, 008, #[cfg(feature = "short-names")] "Albania"),
    Country::new(AQ, ATA, 010, #[cfg(feature = "short-names")] "Antarctica"),
    Country::new(DZ, DZA, 012, #[cfg(feature = "short-names")] "Algeria"),
    Country::new(AS, ASM, 016, #[cfg(feature = "short-names")] "American Samoa"),
    Country::new(AD, AND, 020, #[cfg(feature = "short-names")] "Andorra"),
    Country::new(AO, AGO, 024, #[cfg(feature = "short-names")] "Angola"),
    Country::new(AG, ATG, 028, #[cfg(feature = "short-names")] "Antigua and Barbuda"),
    Country::new(AZ, AZE, 031, #[cfg(feature = "short-names")] "Azerbaijan"),
    Country::new(AR, ARG, 032, #[cfg(feature = "short-names")] "Argentina"),
    Country::new(AU, AUS, 036, #[cfg(feature = "short-names")] "Australia"),
    Country::new(AT, AUT, 040, #[cfg(feature = "short-names")] "Austria"),
    Country::new(BS, BHS, 044, #[cfg(feature = "short-names")] "Bahamas"),
    Country::new(BH, BHR, 048, #[cfg(feature = "short-names")] "Bahrain"),
    Country::new(BD, BGD, 050, #[cfg(feature = "short-names")] "Bangladesh"),
    Country::new(AM, ARM, 051, #[cfg(feature = "short-names")] "Armenia"),
    Country::new(BB, BRB, 052, #[cfg(feature = "short-names")] "Barbados"),
    Country::new(BE, BEL, 056, #[cfg(feature = "short-names")] "Belgium"),
    Country::new(BM, BMU, 060, #[cfg(feature = "short-names")] "Bermuda"),
    Country::new(BT, BTN, 064, #[cfg(feature = "short-names")] "Bhutan"),
    Country::new(BO, BOL, 068, #[cfg(feature = "short-names")] "Bolivia (Plurinational State of)"),
    Country::new(BA, BIH, 070, #[cfg(feature = "short-names")] "Bosnia and Herzegovina"),
    Country::new(BW, BWA, 072, #[cfg(feature = "short-names")] "Botswana"),
    Country::new(BV, BVT, 074, #[cfg(feature = "short-names")] "Bouvet Island"),
    Country::new(BR, BRA, 076, #[cfg(feature = "short-names")] "Brazil"),
    Country::new(BZ, BLZ, 084, #[cfg(feature = "short-names")] "Belize"),
    Country::new(IO, IOT, 086, #[cfg(feature = "short-names")] "British Indian Ocean Territory"),
    Country::new(SB, SLB, 090, #[cfg(feature = "short-names")] "Solomon Islands"),
    Country::new(VG, VGB, 092, #[cfg(feature = "short-names")] "Virgin Islands (British)"),
    Country::new(BN, BRN, 096, #[cfg(feature = "short-names")] "Brunei Darussalam"),
    Country::new(BG, BGR, 100, #[cfg(feature = "short-names")] "Bulgaria"),
    Country::new(MM, MMR, 104, #[cfg(feature = "short-names")] "Myanmar"),
    Country::new(BI, BDI, 108, #[cfg(feature = "short-names")] "Burundi"),
    Country::new(BY, BLR, 112, #[cfg(feature = "short-names")] "Belarus"),
    Country::new(KH, KHM, 116, #[cfg(feature = "short-names")] "Cambodia"),
    Country::new(CM, CMR, 120, #[cfg(feature = "short-names")] "Cameroon"),
    Country::new(CA, CAN, 124, #[cfg(feature = "short-names")] "Canada"),
    Country::new(CV, CPV, 132, #[cfg(feature = "short-names")] "Cabo Verde"),
    Country::new(KY, CYM, 136, #[cfg(feature = "short-names")] "Cayman Islands"),
    Country::new(CF, CAF, 140, #[cfg(feature = "short-names")] "Central African Republic"),
    Country::new(LK, LKA, 144, #[cfg(feature = "short-names")] "Sri Lanka"),
    Country::new(TD, TCD, 148, #[cfg(feature = "short-names")] "Chad"),
    Country::new(CL, CHL, 152, #[cfg(feature = "short-names")] "Chile"),
    Country::new(CN, CHN, 156, #[cfg(feature = "short-names")] "China"),
    Country::new(TW, TWN, 158, #[cfg(feature = "short-names")] "Taiwan, Province of China"),
    Country::new(CX, CXR, 162, #[cfg(feature = "short-names")] "Christmas Island"),
    Country::new(CC, CCK, 166, #[cfg(feature = "short-names")] "Cocos (Keeling) Islands"),
    Country::new(CO, COL, 170, #[cfg(feature = "short-names")] "Colombia"),
    Country::new(KM, COM, 174, #[cfg(feature = "short-names")] "Comoros"),
    Country::new(YT, MYT, 175, #[cfg(feature = "short-names")] "Mayotte"),
    Country::new(CG, COG, 178, #[cfg(feature = "short-names")] "Congo"),
    Country::new(CD, COD, 180, #[cfg(feature = "short-names")] "Congo, Democratic Republic of the"),
    Country::new(CK, COK, 184, #[cfg(feature = "short-names")] "Cook Islands"),
    Country::new(CR, CRI, 188, #[cfg(feature = "short-names")] "Costa Rica"),
    Country::new(HR, HRV, 191, #[cfg(feature = "short-names")] "Croatia"),
    Country::new(CU, CUB, 192, #[cfg(feature = "short-names")] "Cuba"),
    Country::new(CY, CYP, 196, #[cfg(feature = "short-names")] "Cyprus"),
    Country::new(CZ, CZE, 203, #[cfg(feature = "short-names")] "Czechia"),
    Country::new(BJ, BEN, 204, #[cfg(feature = "short-names")] "Benin"),
    Country::new(DK, DNK, 208, #[cfg(feature = "short-names")] "Denmark"),
    Country::new(DM, DMA, 212, #[cfg(feature = "short-names")] "Dominica"),
    Country::new(DO, DOM, 214, #[cfg(feature = "short-names")] "Dominican Republic"),
    Country::new(EC, ECU, 218, #[cfg(feature = "short-names")] "Ecuador"),
    Country::new(SV, SLV, 222, #[cfg(feature = "short-names")] "El Salvador"),
    Country::new(GQ, GNQ, 226, #[cfg(feature = "short-names")] "Equatorial Guinea"),
    Country::new(ER, ERI, 232, #[cfg(feature = "short-names")] "Eritrea"),
    Country::new(ET, ETH, 231, #[cfg(feature = "short-names")] "Ethiopia"),
    Country::new(EE, EST, 233, #[cfg(feature = "short-names")] "Estonia"),
    Country::new(FO, FRO, 234, #[cfg(feature = "short-names")] "Faroe Islands"),
    Country::new(FK, FLK, 238, #[cfg(feature = "short-names")] "Falkland Islands (Malvinas)"),
    Country::new(GS, SGS, 239, #[cfg(feature = "short-names")] "South Georgia and the South Sandwich Islands"),
    Country::new(FJ, FJI, 242, #[cfg(feature = "short-names")] "Fiji"),
    Country::new(FI, FIN, 246, #[cfg(feature = "short-names")] "Finland"),
    Country::new(AX, ALA, 248, #[cfg(feature = "short-names")] "Åland Islands"),
    Country::new(FR, FRA, 250, #[cfg(feature = "short-names")] "France"),
    Country::new(GF, GUF, 254, #[cfg(feature = "short-names")] "French Guiana"),
    Country::new(PF, PYF, 258, #[cfg(feature = "short-names")] "French Polynesia"),
    Country::new(TF, ATF, 260, #[cfg(feature = "short-names")] "French Southern Territories"),
    Country::new(DJ, DJI, 262, #[cfg(feature = "short-names")] "Djibouti"),
    Country::new(GA, GAB, 266, #[cfg(feature = "short-names")] "Gabon"),
    Country::new(GE, GEO, 268, #[cfg(feature = "short-names")] "Georgia"),
    Country::new(GM, GMB, 270, #[cfg(feature = "short-names")] "Gambia"),
    Country::new(PS, PSE, 275, #[cfg(feature = "short-names")] "Palestine, State of"),
    Country::new(DE, DEU, 276, #[cfg(feature = "short-names")] "Germany"),
    Country::new(GH, GHA, 288, #[cfg(feature = "short-names")] "Ghana"),
    Country::new(GI, GIB, 292, #[cfg(feature = "short-names")] "Gibraltar"),
    Country::new(KI, KIR, 296, #[cfg(feature = "short-names")] "Kiribati"),
    Country::new(GR, GRC, 300, #[cfg(feature = "short-names")] "Greece"),
    Country::new(GL, GRL, 304, #[cfg(feature = "short-names")] "Greenland"),
    Country::new(GD, GRD, 308, #[cfg(feature = "short-names")] "Grenada"),
    Country::new(GP, GLP, 312, #[cfg(feature = "short-names")] "Guadeloupe"),
    Country::new(GU, GUM, 316, #[cfg(feature = "short-names")] "Guam"),
    Country::new(GT, GTM, 320, #[cfg(feature = "short-names")] "Guatemala"),
    Country::new(GN, GIN, 324, #[cfg(feature = "short-names")] "Guinea"),
    Country::new(GY, GUY, 328, #[cfg(feature = "short-names")] "Guyana"),
    Country::new(HT, HTI, 332, #[cfg(feature = "short-names")] "Haiti"),
    Country::new(HM, HMD, 334, #[cfg(feature = "short-names")] "Heard Island and McDonald Islands"),
    Country::new(VA, VAT, 336, #[cfg(feature = "short-names")] "Holy See"),
    Country::new(HN, HND, 340, #[cfg(feature = "short-names")] "Honduras"),
    Country::new(HK, HKG, 344, #[cfg(feature = "short-names")] "Hong Kong"),
    Country::new(HU, HUN, 348, #[cfg(feature = "short-names")] "Hungary"),
    Country::new(IS, ISL, 352, #[cfg(feature = "short-names")] "Iceland"),
    Country::new(IN, IND, 356, #[cfg(feature = "short-names")] "India"),
    Country::new(ID, IDN, 360, #[cfg(feature = "short-names")] "Indonesia"),
    Country::new(IR, IRN, 364, #[cfg(feature = "short-names")] "Iran (Islamic Republic of)"),
    Country::new(IQ, IRQ, 368, #[cfg(feature = "short-names")] "Iraq"),
    Country::new(IE, IRL, 372, #[cfg(feature = "short-names")] "Ireland"),
    Country::new(IL, ISR, 376, #[cfg(feature = "short-names")] "Israel"),
    Country::new(IT, ITA, 380, #[cfg(feature = "short-names")] "Italy"),
    Country::new(CI, CIV, 384, #[cfg(feature = "short-names")] "Côte d'Ivoire"),
    Country::new(JM, JAM, 388, #[cfg(feature = "short-names")] "Jamaica"),
    Country::new(JP, JPN, 392, #[cfg(feature = "short-names")] "Japan"),
    Country::new(KZ, KAZ, 398, #[cfg(feature = "short-names")] "Kazakhstan"),
    Country::new(JO, JOR, 400, #[cfg(feature = "short-names")] "Jordan"),
    Country::new(KE, KEN, 404, #[cfg(feature = "short-names")] "Kenya"),
    Country::new(KP, PRK, 408, #[cfg(feature = "short-names")] "Korea (Democratic People's Republic of)"),
    Country::new(KR, KOR, 410, #[cfg(feature = "short-names")] "Korea, Republic of"),
    Country::new(KW, KWT, 414, #[cfg(feature = "short-names")] "Kuwait"),
    Country::new(KG, KGZ, 417, #[cfg(feature = "short-names")] "Kyrgyzstan"),
    Country::new(LA, LAO, 418, #[cfg(feature = "short-names")] "Laos"),
    Country::new(LB, LBN, 422, #[cfg(feature = "short-names")] "Lebanon"),
    Country::new(LS, LSO, 426, #[cfg(feature = "short-names")] "Lesotho"),
    Country::new(LV, LVA, 428, #[cfg(feature = "short-names")] "Latvia"),
    Country::new(LR, LBR, 430, #[cfg(feature = "short-names")] "Liberia"),
    Country::new(LY, LBY, 434, #[cfg(feature = "short-names")] "Libya"),
    Country::new(LI, LIE, 438, #[cfg(feature = "short-names")] "Liechtenstein"),
    Country::new(LT, LTU, 440, #[cfg(feature = "short-names")] "Lithuania"),
    Country::new(LU, LUX, 442, #[cfg(feature = "short-names")] "Luxembourg"),
    Country::new(MO, MAC, 446, #[cfg(feature = "short-names")] "Macao"),
    Country::new(MG, MDG, 450, #[cfg(feature = "short-names")] "Madagascar"),
    Country::new(MW, MWI, 454, #[cfg(feature = "short-names")] "Malawi"),
    Country::new(MY, MYS, 458, #[cfg(feature = "short-names")] "Malaysia"),
    Country::new(MV, MDV, 462, #[cfg(feature = "short-names")] "Maldives"),
    Country::new(ML, MLI, 466, #[cfg(feature = "short-names")] "Mali"),
    Country::new(MT, MLT, 470, #[cfg(feature = "short-names")] "Malta"),
    Country::new(MQ, MTQ, 474, #[cfg(feature = "short-names")] "Martinique"),
    Country::new(MR, MRT, 478, #[cfg(feature = "short-names")] "Mauritania"),
    Country::new(MU, MUS, 480, #[cfg(feature = "short-names")] "Mauritius"),
    Country::new(MX, MEX, 484, #[cfg(feature = "short-names")] "Mexico"),
    Country::new(MC, MCO, 492, #[cfg(feature = "short-names")] "Monaco"),
    Country::new(MN, MNG, 496, #[cfg(feature = "short-names")] "Mongolia"),
    Country::new(MD, MDA, 498, #[cfg(feature = "short-names")] "Moldova, Republic of"),
    Country::new(ME, MNE, 499, #[cfg(feature = "short-names")] "Montenegro"),
    Country::new(MS, MSR, 500, #[cfg(feature = "short-names")] "Montserrat"),
    Country::new(MA, MAR, 504, #[cfg(feature = "short-names")] "Morocco"),
    Country::new(MZ, MOZ, 508, #[cfg(feature = "short-names")] "Mozambique"),
    Country::new(OM, OMN, 512, #[cfg(feature = "short-names")] "Oman"),
    Country::new(NA, NAM, 516, #[cfg(feature = "short-names")] "Namibia"),
    Country::new(NR, NRU, 520, #[cfg(feature = "short-names")] "Nauru"),
    Country::new(NP, NPL, 524, #[cfg(feature = "short-names")] "Nepal"),
    Country::new(NL, NLD, 528, #[cfg(feature = "short-names")] "Netherlands"),
    Country::new(CW, CUW, 531, #[cfg(feature = "short-names")] "Curaçao"),
    Country::new(AW, ABW, 533, #[cfg(feature = "short-names")] "Aruba"),
    Country::new(SX, SXM, 534, #[cfg(feature = "short-names")] "Sint Maarten (Dutch part)"),
    Country::new(BQ, BES, 535, #[cfg(feature = "short-names")] "Bonaire, Sint Eustatius and Saba"),
    Country::new(NC, NCL, 540, #[cfg(feature = "short-names")] "New Caledonia"),
    Country::new(VU, VUT, 548, #[cfg(feature = "short-names")] "Vanuatu"),
    Country::new(NZ, NZL, 554, #[cfg(feature = "short-names")] "New Zealand"),
    Country::new(NI, NIC, 558, #[cfg(feature = "short-names")] "Nicaragua"),
    Country::new(NE, NER, 562, #[cfg(feature = "short-names")] "Niger"),
    Country::new(NG, NGA, 566, #[cfg(feature = "short-names")] "Nigeria"),
    Country::new(NU, NIU, 570, #[cfg(feature = "short-names")] "Niue"),
    Country::new(NF, NFK, 574, #[cfg(feature = "short-names")] "Norfolk Island"),
    Country::new(NO, NOR, 578, #[cfg(feature = "short-names")] "Norway"),
    Country::new(MP, MNP, 580, #[cfg(feature = "short-names")] "Northern Mariana Islands"),
    Country::new(UM, UMI, 581, #[cfg(feature = "short-names")] "United States Minor Outlying Islands"),
    Country::new(FM, FSM, 583, #[cfg(feature = "short-names")] "Micronesia (Federated States of)"),
    Country::new(MH, MHL, 584, #[cfg(feature = "short-names")] "Marshall Islands"),
    Country::new(PW, PLW, 585, #[cfg(feature = "short-names")] "Palau"),
    Country::new(PK, PAK, 586, #[cfg(feature = "short-names")] "Pakistan"),
    Country::new(PA, PAN, 591, #[cfg(feature = "short-names")] "Panama"),
    Country::new(PG, PNG, 598, #[cfg(feature = "short-names")] "Papua New Guinea"),
    Country::new(PY, PRY, 600, #[cfg(feature = "short-names")] "Paraguay"),
    Country::new(PE, PER, 604, #[cfg(feature = "short-names")] "Peru"),
    Country::new(PH, PHL, 608, #[cfg(feature = "short-names")] "Philippines"),
    Country::new(PN, PCN, 612, #[cfg(feature = "short-names")] "Pitcairn"),
    Country::new(PL, POL, 616, #[cfg(feature = "short-names")] "Poland"),
    Country::new(PT, PRT, 620, #[cfg(feature = "short-names")] "Portugal"),
    Country::new(GW, GNB, 624, #[cfg(feature = "short-names")] "Guinea-Bissau"),
    Country::new(TL, TLS, 626, #[cfg(feature = "short-names")] "Timor-Leste"),
    Country::new(PR, PRI, 630, #[cfg(feature = "short-names")] "Puerto Rico"),
    Country::new(QA, QAT, 634, #[cfg(feature = "short-names")] "Qatar"),
    Country::new(RE, REU, 638, #[cfg(feature = "short-names")] "Réunion"),
    Country::new(RO, ROU, 642, #[cfg(feature = "short-names")] "Romania"),
    Country::new(RU, RUS, 643, #[cfg(feature = "short-names")] "Russia"),
    Country::new(RW, RWA, 646, #[cfg(feature = "short-names")] "Rwanda"),
    Country::new(BL, BLM, 652, #[cfg(feature = "short-names")] "Saint Barthélemy"),
    Country::new(SH, SHN, 654, #[cfg(feature = "short-names")] "Saint Helena, Ascension and Tristan da Cunha"),
    Country::new(KN, KNA, 659, #[cfg(feature = "short-names")] "Saint Kitts and Nevis"),
    Country::new(LC, LCA, 662, #[cfg(feature = "short-names")] "Saint Lucia"),
    Country::new(AI, AIA, 660, #[cfg(feature = "short-names")] "Anguilla"),
    Country::new(MF, MAF, 663, #[cfg(feature = "short-names")] "Saint Martin (French part)"),
    Country::new(PM, SPM, 666, #[cfg(feature = "short-names")] "Saint Pierre and Miquelon"),
    Country::new(VC, VCT, 670, #[cfg(feature = "short-names")] "Saint Vincent and the Grenadines"),
    Country::new(SM, SMR, 674, #[cfg(feature = "short-names")] "San Marino"),
    Country::new(ST, STP, 678, #[cfg(feature = "short-names")] "Sao Tome and Principe"),
    Country::new(SA, SAU, 682, #[cfg(feature = "short-names")] "Saudi Arabia"),
    Country::new(SN, SEN, 686, #[cfg(feature = "short-names")] "Senegal"),
    Country::new(RS, SRB, 688, #[cfg(feature = "short-names")] "Serbia"),
    Country::new(SC, SYC, 690, #[cfg(feature = "short-names")] "Seychelles"),
    Country::new(SL, SLE, 694, #[cfg(feature = "short-names")] "Sierra Leone"),
    Country::new(SG, SGP, 702, #[cfg(feature = "short-names")] "Singapore"),
    Country::new(SK, SVK, 703, #[cfg(feature = "short-names")] "Slovakia"),
    Country::new(VN, VNM, 704, #[cfg(feature = "short-names")] "Viet Nam"),
    Country::new(SI, SVN, 705, #[cfg(feature = "short-names")] "Slovenia"),
    Country::new(SO, SOM, 706, #[cfg(feature = "short-names")] "Somalia"),
    Country::new(ZA, ZAF, 710, #[cfg(feature = "short-names")] "South Africa"),
    Country::new(ZW, ZWE, 716, #[cfg(feature = "short-names")] "Zimbabwe"),
    Country::new(ES, ESP, 724, #[cfg(feature = "short-names")] "Spain"),
    Country::new(SS, SSD, 728, #[cfg(feature = "short-names")] "South Sudan"),
    Country::new(SD, SDN, 729, #[cfg(feature = "short-names")] "Sudan"),
    Country::new(EH, ESH, 732, #[cfg(feature = "short-names")] "Western Sahara"),
    Country::new(SR, SUR, 740, #[cfg(feature = "short-names")] "Suriname"),
    Country::new(SJ, SJM, 744, #[cfg(feature = "short-names")] "Svalbard and Jan Mayen"),
    Country::new(SZ, SWZ, 748, #[cfg(feature = "short-names")] "Eswatini"),
    Country::new(SE, SWE, 752, #[cfg(feature = "short-names")] "Sweden"),
    Country::new(CH, CHE, 756, #[cfg(feature = "short-names")] "Switzerland"),
    Country::new(SY, SYR, 760, #[cfg(feature = "short-names")] "Syrian Arab Republic"),
    Country::new(TJ, TJK, 762, #[cfg(feature = "short-names")] "Tajikistan"),
    Country::new(TH, THA, 764, #[cfg(feature = "short-names")] "Thailand"),
    Country::new(TG, TGO, 768, #[cfg(feature = "short-names")] "Togo"),
    Country::new(TK, TKL, 772, #[cfg(feature = "short-names")] "Tokelau"),
    Country::new(TO, TON, 776, #[cfg(feature = "short-names")] "Tonga"),
    Country::new(AE, ARE, 784, #[cfg(feature = "short-names")] "United Arab Emirates"),
    Country::new(TN, TUN, 788, #[cfg(feature = "short-names")] "Tunisia"),
    Country::new(TT, TTO, 780, #[cfg(feature = "short-names")] "Trinidad and Tobago"),
    Country::new(TR, TUR, 792, #[cfg(feature = "short-names")] "Turkey"),
    Country::new(TM, TKM, 795, #[cfg(feature = "short-names")] "Turkmenistan"),
    Country::new(TC, TCA, 796, #[cfg(feature = "short-names")] "Turks and Caicos Islands"),
    Country::new(TV, TUV, 798, #[cfg(feature = "short-names")] "Tuvalu"),
    Country::new(UG, UGA, 800, #[cfg(feature = "short-names")] "Uganda"),
    Country::new(UA, UKR, 804, #[cfg(feature = "short-names")] "Ukraine"),
    Country::new(MK, MKD, 807, #[cfg(feature = "short-names")] "North Macedonia"),
    Country::new(EG, EGY, 818, #[cfg(feature = "short-names")] "Egypt"),
    Country::new(GB, GBR, 826, #[cfg(feature = "short-names")] "United Kingdom of Great Britain and Northern Ireland"),
    Country::new(GG, GGY, 831, #[cfg(feature = "short-names")] "Guernsey"),
    Country::new(JE, JEY, 832, #[cfg(feature = "short-names")] "Jersey"),
    Country::new(IM, IMN, 833, #[cfg(feature = "short-names")] "Isle of Man"),
    Country::new(TZ, TZA, 834, #[cfg(feature = "short-names")] "Tanzania, United Republic of"),
    Country::new(US, USA, 840, #[cfg(feature = "short-names")] "United States of America"),
    Country::new(VI, VIR, 850, #[cfg(feature = "short-names")] "Virgin Islands (U.S.)"),
    Country::new(BF, BFA, 854, #[cfg(feature = "short-names")] "Burkina Faso"),
    Country::new(UY, URY, 858, #[cfg(feature = "short-names")] "Uruguay"),
    Country::new(UZ, UZB, 860, #[cfg(feature = "short-names")] "Uzbekistan"),
    Country::new(VE, VEN, 862, #[cfg(feature = "short-names")] "Venezuela (Bolivarian Republic of)"),
    Country::new(WF, WLF, 876, #[cfg(feature = "short-names")] "Wallis and Futuna"),
    Country::new(WS, WSM, 882, #[cfg(feature = "short-names")] "Samoa"),
    Country::new(YE, YEM, 887, #[cfg(feature = "short-names")] "Yemen"),
    Country::new(ZM, ZMB, 894, #[cfg(feature = "short-names")] "Zambia"),
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
