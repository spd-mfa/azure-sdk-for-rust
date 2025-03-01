#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-preview-2022-03")]
pub mod package_preview_2022_03;
#[cfg(all(feature = "package-preview-2022-03", not(feature = "no-default-tag")))]
pub use package_preview_2022_03::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2022-02-01-preview")]
pub mod package_2022_02_01_preview;
#[cfg(all(feature = "package-2022-02-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_02_01_preview::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-10-01-preview")]
pub mod package_2021_10_01_preview;
#[cfg(all(feature = "package-2021-10-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_10_01_preview::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-07-01-preview")]
pub mod package_2021_07_01_preview;
#[cfg(all(feature = "package-2021-07-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_07_01_preview::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-05-01-preview")]
pub mod package_2021_05_01_preview;
#[cfg(all(feature = "package-2021-05-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_05_01_preview::{models, operations, operations::Client, operations::ClientBuilder};
