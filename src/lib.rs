//! A library for parsing and incrementing arbitrarily-formatted versions.
//!
//! Instead of conforming to a specific versioning scheme, this library allows you to define your
//! own version format, parse version strings against it, and increment versions according to
//! semantic and/or calendar rules.
//!
//! # Quick start
//! 
//! ```
//! use version_bump::{Version, VersionBumpError, SemanticLevel};
//! 
//! let version = Version::from_parsed_format("[MAJOR].[MINOR].[PATCH]", "1.2.3").unwrap();
//! let incremented = version.increment(Some(&SemanticLevel::Minor), None).unwrap();
//! assert_eq!("1.3.0", incremented.to_string());
//! assert!(version < incremented);
//! 
//! // you can use arbitrary literals
//! let arbitrary = Version::from_parsed_format("v[MAJOR]-[PATCH]", "v1-2").unwrap();
//! 
//! // you can use dates
//! let dated = Version::from_parsed_format("[YYYY].[PATCH]", "2024.18").unwrap();
//! 
//! // errors tell you what went wrong
//! let invalid = Version::from_parsed_format("[MAJOR].[MINOR].[PATCH]", "1.foo.3");
//! assert!(matches!(invalid, Err(VersionBumpError::VersionFormatMismatch {..})));
//! ```
//! 
//! Jump to the specifiers table [here](struct.Format.html#specifier-table).
#![feature(lazy_cell)]

mod error;
mod scheme;
mod format;
mod specifier;
mod version;

pub use crate::error::VersionBumpError;
pub use crate::format::Format;
pub use crate::specifier::SemanticLevel;
// pub use crate::version::{Date, Version};
pub use crate::version::{Date};
