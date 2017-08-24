#![deny(unused_variables)]
#![deny(unused_mut)]
extern crate cast;
#[macro_use]
extern crate error_chain;
extern crate neon;
extern crate serde;

pub mod ser;
pub mod de;
pub mod errors;

pub use ser::to_value;
pub use de::from_handle;
