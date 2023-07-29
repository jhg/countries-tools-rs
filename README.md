# Countries list from ISO 3166-1

Countries list from ISO 3166-1

## Usage

After `cargo install countries-tools`:

```rust
use countries_tools::{Country, CountryAlpha2};

assert_eq!(Country::from(CountryAlpha2::FR).short_name(), "France");
```

## About this crate

As much methods as possible are `const fn` to allow compile-time computation and optimize runtime performance.

Also this crate is `no_std` compatible. The crate size is kept as small as possible and also the impact on the binary size.
