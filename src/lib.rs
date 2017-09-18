#![allow(unknown_lints)]
#![deny(unused_variables)]
#![deny(unused_mut)]
#![deny(clippy)]
#![deny(clippy_pedantic)]
#![allow(stutter)]

extern crate cast;
#[macro_use]
extern crate error_chain;
extern crate neon;
extern crate serde;

pub mod ser;
pub mod de;
pub mod errors;

pub use de::from_handle;
pub use ser::to_value;
