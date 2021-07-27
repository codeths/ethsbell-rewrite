#![feature(decl_macro)]

#[cfg(feature = "ws")]
#[macro_use]
extern crate rocket;
#[cfg(feature = "ws")]
#[macro_use]
extern crate rocket_okapi;

#[cfg(feature = "ws")]
pub mod api;
pub mod ical;
pub mod impls;
#[cfg(feature = "ws")]
pub mod locks;
#[cfg(feature = "ws")]
pub mod login;
#[cfg(feature = "ws")]
pub mod rocket_builder;
#[cfg(feature = "ws")]
pub use locks::SpecLock;
#[cfg(feature = "ws")]
pub mod frontend;
pub mod schedule;

/// Re-export things from serde.
pub use serde_json::from_str;
