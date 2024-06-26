#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2016-05")]
pub mod package_2016_05;
#[cfg(feature = "package-2017-07")]
pub mod package_2017_07;
#[cfg(feature = "package-2017-08-beta")]
pub mod package_2017_08_beta;
#[cfg(all(feature = "default_tag", feature = "package-2017-08-beta"))]
pub use package_2017_08_beta::*;
