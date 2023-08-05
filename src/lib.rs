#![doc = include_str!("../README.md")]

#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "short-names")]
use core::fmt;

mod countries;
#[cfg(feature = "short-names")]
mod countries_short_names;

pub use countries::{CountryAlpha2, CountryAlpha3};

/// A country in the list of countries defined by ISO 3166-1.
/// 
/// # Examples
/// 
/// ```
/// use countries_tools::{Country, CountryAlpha2};
/// 
/// let united_states = Country::from(CountryAlpha2::US);
/// println!("{} is in North America.", united_states);
/// // Prints "United States of America is in North America."
/// 
/// assert_eq!(united_states.short_name(), "United States of America");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Country {
    alpha2: CountryAlpha2,
    alpha3: CountryAlpha3,
    numeric: u16,
}

impl Country {
    // No need to make this public, since countries are created by this crate only.
    #[inline]
    const fn new(
        alpha2: CountryAlpha2,
        alpha3: CountryAlpha3,
        numeric: u16,
        // NOTE: short name is not part of the struct as &'static str size is 16 bytes
        //         and that would make the struct 24 bytes, making the cache of the CPU
        //         less efficient. Instead alpha2 code enum is used as offset to get the
        //         short name from the array of short names. That make the struct 4 bytes
        //         and the cache of the CPU more efficient.
        //
        //       This has been measured with criterion before and the results show an speed
        //         improvement of 47.21%-61.14% when searching for a country by numeric code,
        //         and 0.15%-87.35% when searching for a country by alpha2 or alpha3 code.
    ) -> Self {
        Self {
            alpha2,
            alpha3,
            numeric,
        }
    }

    #[inline]
    pub const fn alpha2(&self) -> CountryAlpha2 {
        self.alpha2
    }

    #[inline]
    pub const fn alpha3(&self) -> CountryAlpha3 {
        self.alpha3
    }

    #[inline]
    pub const fn numeric(&self) -> u16 {
        self.numeric
    }

    #[cfg(feature = "short-names")]
    #[inline]
    pub const fn short_name(&self) -> &'static str {
        countries_short_names::short_name_from_alpha2(self.alpha2)
    }
}

#[cfg(feature = "short-names")]
impl fmt::Display for Country {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.short_name().fmt(f)
    }
}
