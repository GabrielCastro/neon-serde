#![allow(unknown_lints)]
#![deny(unused_variables)]
#![deny(unused_mut)]
#![deny(clippy)]
#![deny(clippy_pedantic)]
#![allow(stutter)]
#![recursion_limit = "128"]

//!
//! Neon-serde
//! ==========
//!
//! This crate is a utility to easily convert values between
//!
//! A `Handle<JsValue>` from the `neon` crate
//! and any value implementing `serde::{Serialize, Deserialize}`
//!
//! ## Usage
//!
//! #### `neon_serde::from_handle`
//! Convert a `Handle<js::JsValue>` to
//! a type implementing `serde::Deserialize`
//!
//! #### `neon_serde::to_value`
//! Convert a value implementing `serde::Serialize` to
//! a `Handle<JsValue>`
//!
//!
//! ## Example
//!
//! ```rust,no_run
//! # #![allow(dead_code)]
//! extern crate neon_serde;
//! extern crate neon;
//! #[macro_use]
//! extern crate serde_derive;
//!
//! use neon::js::{JsValue, JsUndefined};
//! use neon::vm::{Call, JsResult};
//!
//! #[derive(Serialize, Debug, Deserialize)]
//! struct AnObject {
//!     a: u32,
//!     b: Vec<f64>,
//!     c: String,
//! }
//!
//! fn deserialize_something(call: Call) -> JsResult<JsValue> {
//!     let scope = call.scope;
//!     let arg0 = call.arguments
//!          .require(scope, 0)?
//!          .check::<JsValue>()?;
//!
//!     let arg0_value :AnObject = neon_serde::from_value(scope, arg0)?;
//!     println!("{:?}", arg0_value);
//!
//!     Ok(JsUndefined::new().upcast())
//! }
//!
//! fn serialize_something(call: Call) -> JsResult<JsValue> {
//!     let scope = call.scope;
//!     let value = AnObject {
//!         a: 1,
//!         b: vec![2f64, 3f64, 4f64],
//!         c: "a string".into()
//!     };
//!
//!     let js_value = neon_serde::to_value(scope, &value)?;
//!     Ok(js_value)
//! }
//!
//! # fn main () {
//! # }
//!
//! ```
//!

extern crate cast;
#[macro_use]
extern crate error_chain;
extern crate neon;
#[macro_use]
extern crate serde;

pub mod ser;
pub mod de;
pub mod errors;

pub use de::from_value;
pub use ser::to_value;



#[cfg(test)]
mod tests {
    use neon::mem::Handle;
    use neon::vm::{Call, JsResult};
    use neon::js::JsValue;
    use super::*;

    #[allow(unsed)]
    fn test_it_compiles<'j>(call: Call<'j>) -> JsResult<'j, JsValue> {
        let scope = call.scope;
        let result: () = {
            let arg: Handle<'j, JsValue> = call.arguments.require(scope, 0)?;
            let () = from_value(scope, arg)?;
            ()
        };
        let result: Handle<'j, JsValue> = to_value(&result, scope)?;
        Ok(result)
    }
}
