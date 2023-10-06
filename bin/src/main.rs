#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::complexity,
    clippy::nursery,
    clippy::unwrap_used,
    unused_qualifications,
    rust_2018_idioms,
    clippy::expect_used,
    trivial_casts,
    trivial_numeric_casts,
    unused_allocation,
    clippy::as_conversions,
    clippy::dbg_macro,
    clippy::deprecated_cfg_attr,
    clippy::separated_literal_suffix,
    deprecated
)]
#![forbid(unsafe_code, deprecated_in_future)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

fn main() {
    println!("{}", 1u8 + 2u8);
}
