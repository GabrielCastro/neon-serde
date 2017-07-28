#[macro_use]
extern crate neon;
extern crate neon_serde;

use neon::vm::{Call, JsResult};
use neon::js::JsValue;

fn hello(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let value = 77u32;

    let handle = neon_serde::to_value(&value, scope).unwrap();
    Ok(handle)
}

register_module!(m, {
    m.export("hello", hello)
});
