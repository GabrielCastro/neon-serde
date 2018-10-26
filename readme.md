Neon-serde
==========

[![Build Status](https://travis-ci.org/GabrielCastro/neon-serde.svg?branch=master)](https://travis-ci.org/GabrielCastro/neon-serde)
[![](https://meritbadge.herokuapp.com/neon-serde)](https://crates.io/crates/neon-serde)

This crate is a utility to easily convert values between

A `Handle<JsValue>` from the [https://github.com/neon-bindings/neon](neon) crate
and any value implementing `serde::{Serialize, Deserialize}`

## Versions support

neon-serde targets node >= 6.0
`Uint8ClampedArray` only works on node >= 8

## Usage

#### `neon_serde::from_handle`
Convert a `Handle<js::JsValue>` to
a type implementing `serde::Deserialize`

#### `neon_serde::to_value`˚
Convert a value implementing `serde::Serialize` to
a `Handle<JsValue>`

## Export Macro example
The export! macro allows you to quickly define functions automatically convert thier arguments

```rust,no_run

#[macro_use]
extern crate neon;
#[macro_use]
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u16,
}

export! {
    fn say_hello(name: String) -> String {
        format!("Hello, {}!", name)
    }

    fn greet(user: User) -> String {
        format!("{} is {} years old", user.name, user.age)
    }

    fn fibonacci(n: i32) -> i32 {
        match n {
            1 | 2 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
}

```


## Direct Usage Example

```rust,no_run
extern crate neon_serde;
extern crate neon;
#[macro_use]
extern crate serde_derive;

use neon::prelude::*;

#[derive(Serialize, Debug, Deserialize)]
struct AnObject {
    a: u32,
    b: Vec<f64>,
    c: String,
}

fn deserialize_something(mut cx: FunctionContext) -> JsResult<JsValue> {
    let arg0 = cx.argument::<JsValue>(0)?;

    let arg0_value :AnObject = neon_serde::from_value(&mut cx, arg0)?;
    println!("{:?}", arg0_value);

    Ok(JsUndefined::new().upcast())
}

fn serialize_something(mut cx: FunctionContext) -> JsResult<JsValue> {
    let value = AnObject {
        a: 1,
        b: vec![2f64, 3f64, 4f64],
        c: "a string".into()
    };

    let js_value = neon_serde::to_value(&mut cx, &value)?;
    Ok(js_value)
}
```

## Limitations

### Data ownership
All Deserialize Values must own all their data (they must have the trait `serde::DererializeOwned`)
