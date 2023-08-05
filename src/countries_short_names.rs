#![cfg(feature = "short-names")]

// NOTE: Read the comment in Country struct in src\lib.rs about why short names are not part of the struct.

const SHORT_NAMES: &[&str] = &[
    "Afghanistan",
    "Albania",
    "Antarctica",
    "Algeria",
    "American Samoa",
    "Andorra",
    "Angola",
    "Antigua and Barbuda",
    "Azerbaijan",
    "Argentina",
    "Australia",
    "Austria",
    "Bahamas",
    "Bahrain",
    "Bangladesh",
    "Armenia",
    "Barbados",
    "Belgium",
    "Bermuda",
    "Bhutan",
    "Bolivia (Plurinational State of)",
    "Bosnia and Herzegovina",
    "Botswana",
    "Bouvet Island",
    "Brazil",
    "Belize",
    "British Indian Ocean Territory",
    "Solomon Islands",
    "Virgin Islands (British)",
    "Brunei Darussalam",
    "Bulgaria",
    "Myanmar",
    "Burundi",
    "Belarus",
    "Cambodia",
    "Cameroon",
    "Canada",
    "Cabo Verde",
    "Cayman Islands",
    "Central African Republic",
    "Sri Lanka",
    "Chad",
    "Chile",
    "China",
    "Taiwan, Province of China",
    "Christmas Island",
    "Cocos (Keeling) Islands",
    "Colombia",
    "Comoros",
    "Mayotte",
    "Congo",
    "Congo, Democratic Republic of the",
    "Cook Islands",
    "Costa Rica",
    "Croatia",
    "Cuba",
    "Cyprus",
    "Czechia",
    "Benin",
    "Denmark",
    "Dominica",
    "Dominican Republic",
    "Ecuador",
    "El Salvador",
    "Equatorial Guinea",
    "Eritrea",
    "Ethiopia",
    "Estonia",
    "Faroe Islands",
    "Falkland Islands (Malvinas)",
    "South Georgia and the South Sandwich Islands",
    "Fiji",
    "Finland",
    "Åland Islands",
    "France",
    "French Guiana",
    "French Polynesia",
    "French Southern Territories",
    "Djibouti",
    "Gabon",
    "Georgia",
    "Gambia",
    "Palestine, State of",
    "Germany",
    "Ghana",
    "Gibraltar",
    "Kiribati",
    "Greece",
    "Greenland",
    "Grenada",
    "Guadeloupe",
    "Guam",
    "Guatemala",
    "Guinea",
    "Guyana",
    "Haiti",
    "Heard Island and McDonald Islands",
    "Holy See",
    "Honduras",
    "Hong Kong",
    "Hungary",
    "Iceland",
    "India",
    "Indonesia",
    "Iran (Islamic Republic of)",
    "Iraq",
    "Ireland",
    "Israel",
    "Italy",
    "Côte d'Ivoire",
    "Jamaica",
    "Japan",
    "Kazakhstan",
    "Jordan",
    "Kenya",
    "Korea (Democratic People's Republic of)",
    "Korea, Republic of",
    "Kuwait",
    "Kyrgyzstan",
    "Laos",
    "Lebanon",
    "Lesotho",
    "Latvia",
    "Liberia",
    "Libya",
    "Liechtenstein",
    "Lithuania",
    "Luxembourg",
    "Macao",
    "Madagascar",
    "Malawi",
    "Malaysia",
    "Maldives",
    "Mali",
    "Malta",
    "Martinique",
    "Mauritania",
    "Mauritius",
    "Mexico",
    "Monaco",
    "Mongolia",
    "Moldova, Republic of",
    "Montenegro",
    "Montserrat",
    "Morocco",
    "Mozambique",
    "Oman",
    "Namibia",
    "Nauru",
    "Nepal",
    "Netherlands",
    "Curaçao",
    "Aruba",
    "Sint Maarten (Dutch part)",
    "Bonaire, Sint Eustatius and Saba",
    "New Caledonia",
    "Vanuatu",
    "New Zealand",
    "Nicaragua",
    "Niger",
    "Nigeria",
    "Niue",
    "Norfolk Island",
    "Norway",
    "Northern Mariana Islands",
    "United States Minor Outlying Islands",
    "Micronesia (Federated States of)",
    "Marshall Islands",
    "Palau",
    "Pakistan",
    "Panama",
    "Papua New Guinea",
    "Paraguay",
    "Peru",
    "Philippines",
    "Pitcairn",
    "Poland",
    "Portugal",
    "Guinea-Bissau",
    "Timor-Leste",
    "Puerto Rico",
    "Qatar",
    "Réunion",
    "Romania",
    "Russia",
    "Rwanda",
    "Saint Barthélemy",
    "Saint Helena, Ascension and Tristan da Cunha",
    "Saint Kitts and Nevis",
    "Saint Lucia",
    "Anguilla",
    "Saint Martin (French part)",
    "Saint Pierre and Miquelon",
    "Saint Vincent and the Grenadines",
    "San Marino",
    "Sao Tome and Principe",
    "Saudi Arabia",
    "Senegal",
    "Serbia",
    "Seychelles",
    "Sierra Leone",
    "Singapore",
    "Slovakia",
    "Viet Nam",
    "Slovenia",
    "Somalia",
    "South Africa",
    "Zimbabwe",
    "Spain",
    "South Sudan",
    "Sudan",
    "Western Sahara",
    "Suriname",
    "Svalbard and Jan Mayen",
    "Eswatini",
    "Sweden",
    "Switzerland",
    "Syrian Arab Republic",
    "Tajikistan",
    "Thailand",
    "Togo",
    "Tokelau",
    "Tonga",
    "United Arab Emirates",
    "Tunisia",
    "Trinidad and Tobago",
    "Turkey",
    "Turkmenistan",
    "Turks and Caicos Islands",
    "Tuvalu",
    "Uganda",
    "Ukraine",
    "North Macedonia",
    "Egypt",
    "United Kingdom of Great Britain and Northern Ireland",
    "Guernsey",
    "Jersey",
    "Isle of Man",
    "Tanzania, United Republic of",
    "United States of America",
    "Virgin Islands (U.S.)",
    "Burkina Faso",
    "Uruguay",
    "Uzbekistan",
    "Venezuela (Bolivarian Republic of)",
    "Wallis and Futuna",
    "Samoa",
    "Yemen",
    "Zambia",
];

use crate::countries::CountryAlpha2;

#[inline]
pub(crate) const fn short_name_from_alpha2(alpha2: CountryAlpha2) -> &'static str {
    SHORT_NAMES[alpha2.internal_offset()]
}
