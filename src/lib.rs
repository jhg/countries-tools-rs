#![doc = include_str!("../README.md")]

#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "short-names")]
use core::fmt;

mod countries;

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
    #[cfg(feature = "short-names")]
    short_name: &'static str,
}

impl Country {
    // No need to make this public, since countries are created by this crate only.
    #[inline]
    const fn new(
        alpha2: CountryAlpha2,
        alpha3: CountryAlpha3,
        numeric: u16,
        #[cfg(feature = "short-names")]
        short_name: &'static str,
    ) -> Self {
        Self {
            alpha2,
            alpha3,
            numeric,
            #[cfg(feature = "short-names")]
            short_name,
        }
    }

    #[cfg(feature = "short-names")]
    #[inline]
    pub const fn short_name(&self) -> &'static str {
        self.short_name
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
}

#[cfg(feature = "short-names")]
impl fmt::Display for Country {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.short_name.fmt(f)
    }
}
