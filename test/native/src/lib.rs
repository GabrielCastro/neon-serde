#[macro_use]
extern crate neon;
extern crate neon_serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;

use neon::js::{JsUndefined, JsValue};
use neon::mem::Handle;
use neon::vm::{Call, JsResult};

#[derive(Serialize, Debug, Deserialize)]
struct AnObject {
    a: u32,
    b: Vec<f64>,
    c: String,
}

#[derive(Serialize, Debug, Deserialize, Eq, PartialEq)]
struct Inner;

#[derive(Serialize, Debug, Deserialize, Eq, PartialEq)]
struct Inner2(i32, bool, String);

#[derive(Serialize, Debug, Deserialize, Eq, PartialEq)]
enum TypeEnum {
    Empty,
    Tuple(u32, String),
    Struct { a: u8, b: Vec<u8> },
    Value(Vec<char>),
}


#[derive(Serialize, Debug, Deserialize, Eq, PartialEq)]
struct AnObjectTwo {
    a: u32,
    b: Vec<i64>,
    c: String,
    d: Option<bool>,
    e: Option<bool>,
    f: Inner,
    g: Inner2,
    h: char,
    i: TypeEnum,
    j: TypeEnum,
    k: TypeEnum,
    l: String,
    m: Vec<u8>,
    o: TypeEnum,
}

macro_rules! make_test {
    ($name:ident, $val:expr) => {
        fn $name(call: Call) -> JsResult<JsValue> {
            fn inner(call: Call) -> neon_serde::errors::Result<Handle<JsValue>> {
                let scope = call.scope;
                let value = $val;

                let handle = neon_serde::to_value(&value, scope)?;
                Ok(handle)
            }

            Ok(inner(call)?)
        }
    };
}

make_test!(make_num_77, 77i32);
make_test!(make_num_32, 32u8);
make_test!(make_str_hello, "Hello World");
make_test!(make_num_array, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
make_test!(
    make_obj,
    AnObject {
        a: 1,
        b: vec![0.1f64, 1.1, 2.2, 3.3],
        c: "Hi".into(),
    }
);
make_test!(make_map, {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map
});

make_test!(make_object, {
    let value = AnObjectTwo {
        a: 1,
        b: vec![1, 2],
        c: "abc".into(),
        d: Some(false),
        e: None,
        f: Inner,
        g: Inner2(9, false, "efg".into()),
        h: '🤷',
        i: TypeEnum::Empty,
        j: TypeEnum::Tuple(27, "hij".into()),
        k: TypeEnum::Struct {
            a: 128,
            b: vec![9, 8, 7],
        },
        l: "jkl".into(),
        m: vec![0, 1, 2, 3, 4],
        o: TypeEnum::Value(vec!['z', 'y', 'x']),
    };
    value
});

const NUMBER_BYTES: &'static [u8] = &[255u8, 254, 253];

make_test!(make_buff, { serde_bytes::Bytes::new(NUMBER_BYTES) });

macro_rules! make_expect {
    ($name:ident, $val:expr, $val_type:ty) => {
        fn $name(call: Call) -> JsResult<JsValue> {
            fn inner(call: Call) -> neon_serde::errors::Result<Handle<JsValue>> {
                let scope = call.scope;
                let value = $val;
                let arg0 = call.arguments
                    .require(scope, 0)?;

                let de_serialized: $val_type = neon_serde::from_handle(arg0, scope)?;
                assert_eq!(value, de_serialized);
                Ok(JsUndefined::new().upcast())
            }
            Ok(inner(call)?)
        }
    };
}

make_expect!(expect_hello_world, "hello world", String);

make_expect!(
    expect_obj,
    AnObjectTwo {
        a: 1,
        b: vec![1, 2],
        c: "abc".into(),
        d: Some(false),
        e: None,
        f: Inner,
        g: Inner2(9, false, "efg".into()),
        h: '🤷',
        i: TypeEnum::Empty,
        j: TypeEnum::Tuple(27, "hij".into()),
        k: TypeEnum::Struct {
            a: 128,
            b: vec![9, 8, 7],
        },
        l: "jkl".into(),
        m: vec![0, 1, 2, 3, 4],
        o: TypeEnum::Value(vec!['z', 'y', 'x']),
    },
    AnObjectTwo
);

make_expect!(expect_num_array, vec![0, 1, 2, 3], Vec<i32>);

make_expect!(
    expect_buffer,
    serde_bytes::ByteBuf::from(vec![252u8, 251, 250]),
    serde_bytes::ByteBuf
);

register_module!(m, {
    m.export("make_num_77", make_num_77)?;
    m.export("make_num_32", make_num_32)?;
    m.export("make_str_hello", make_str_hello)?;
    m.export("make_num_array", make_num_array)?;
    m.export("make_buff", make_buff)?;
    m.export("make_obj", make_obj)?;
    m.export("make_object", make_object)?;
    m.export("make_map", make_map)?;

    m.export("expect_hello_world", expect_hello_world)?;
    m.export("expect_obj", expect_obj)?;
    m.export("expect_num_array", expect_num_array)?;
    m.export("expect_buffer", expect_buffer)?;
    Ok(())
});
