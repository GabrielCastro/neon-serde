Neon-serde
==========

[![Build Status](https://travis-ci.org/GabrielCastro/neon-serde.svg?branch=master)](https://travis-ci.org/GabrielCastro/neon-serde)
[![](https://meritbadge.herokuapp.com/neon-serde)](https://crates.io/crates/neon-serde)

This crate is a utility to easily convert values between

A `Handle<JsValue>` from the `neon` crate
and any value implementing `serde::{Serialize, Deserialize}`

## Usage

#### `neon_serde::from_handle`
Convert a `Handle<js::JsValue>` to
a type implementing `serde::Deserialize`

#### `neon_serde::to_value`
Convert a value implementing `serde::Serialize` to
a `Handle<JsValue>`


## Example

```rust,no_run
extern crate neon_serde;
extern crate neon;
#[macro_use]
extern crate serde_derive;

use neon::js::{JsValue, JsUndefined};
use neon::vm::{Call, JsResult};

#[derive(Serialize, Debug, Deserialize)]
struct AnObject {
    a: u32,
    b: Vec<f64>,
    c: String,
}

fn deserialize_something(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let arg0 = call.arguments
         .require(scope, 0)?
         .check::<JsValue>()?;

    let arg0_value :AnObject = neon_serde::from_handle(arg0, scope)?;
    println!("{:?}", arg0_value);

    Ok(JsUndefined::new().upcast())
}

fn serialize_something(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let value = AnObject {
        a: 1,
        b: vec![2f64, 3f64, 4f64],
        c: "a string".into()
    };

    let js_value = neon_serde::to_value(&value, scope)?;
    Ok(js_value)
}

```
