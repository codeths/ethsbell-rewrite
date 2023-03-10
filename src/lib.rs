#![feature(decl_macro)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

//! A library for running ETHSBell's logic client-side.
#[cfg(feature = "ws")]
#[macro_use]
extern crate rocket;
#[cfg(feature = "ws")]
#[macro_use]
extern crate rocket_okapi;

pub mod api;
pub mod ical;
pub mod impls;
#[cfg(feature = "ws")]
pub mod locks;
pub mod login;
#[cfg(feature = "ws")]
pub mod rocket_builder;
#[cfg(feature = "ws")]
pub use locks::SpecLock;
#[cfg(feature = "ws")]
pub mod frontend;
pub mod schedule;

/// Contains type aliases which may be useful to those consuming ETHSBell as a library
pub mod aliases;

/// Re-export things from serde.
pub use serde_json::from_str;
