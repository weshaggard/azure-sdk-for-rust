#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2020-10-30-preview")]
pub mod package_2020_10_30_preview;
#[cfg(all(feature = "package-2020-10-30-preview", not(feature = "no-default-version")))]
pub use package_2020_10_30_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};