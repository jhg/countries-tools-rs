use criterion::{black_box, criterion_group, criterion_main, Criterion};
use countries_tools::{Country, CountryAlpha2, CountryAlpha3};

use std::str::FromStr;

pub fn conversions_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Conversions");

    group.sample_size(5000);
    group.noise_threshold(0.20);
    group.warm_up_time(std::time::Duration::from_secs(10));

    group.bench_function("AD to AND", |b| b.iter(|| {
        CountryAlpha3::from(black_box(CountryAlpha2::AD))
    }));

    group.bench_function("AND to AD", |b| b.iter(|| {
        CountryAlpha2::from(black_box(CountryAlpha3::AND))
    }));

    group.bench_function("AD to 020", |b| b.iter(|| {
        u16::from(black_box(CountryAlpha2::AD))
    }));

    group.bench_function("020 to AD", |b| b.iter(|| {
        CountryAlpha2::try_from(black_box(020))
    }));

    group.bench_function("AND to 020", |b| b.iter(|| {
        u16::from(black_box(CountryAlpha3::AND))
    }));

    group.bench_function("020 to AND", |b| b.iter(|| {
        CountryAlpha3::try_from(black_box(020))
    }));

    group.bench_function("AD to Country", |b| b.iter(|| {
        Country::from(black_box(CountryAlpha2::AD))
    }));

    group.bench_function("AND to Country", |b| b.iter(|| {
        Country::from(black_box(CountryAlpha3::AND))
    }));

    group.bench_function("020 to Country", |b| b.iter(|| {
        Country::try_from(black_box(020))
    }));

    group.bench_function("AD from str", |b| b.iter(|| {
        CountryAlpha2::from_str(black_box("AD"))
    }));

    group.bench_function("AND from str", |b| b.iter(|| {
        CountryAlpha3::from_str(black_box("AND"))
    }));

    group.bench_function("AD to string", |b| b.iter(|| {
        black_box(CountryAlpha2::AD).to_string()
    }));

    group.bench_function("AND to string", |b| b.iter(|| {
        black_box(CountryAlpha3::AND).to_string()
    }));

    group.bench_function("Country to string", |b| b.iter(|| {
        black_box(Country::from(CountryAlpha2::AD)).to_string()
    }));

    group.bench_function("Country to str (short name)", |b| b.iter(|| {
        black_box(Country::from(CountryAlpha2::AD)).short_name()
    }));

    group.finish();
}

criterion_group!(benches, conversions_benchmark);
criterion_main!(benches);
