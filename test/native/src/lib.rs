extern crate neon;
extern crate neon_serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;

use neon::prelude::*;

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

#[derive(Serialize, Debug, Deserialize, PartialEq)]
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
    p: Vec<f64>,
    q: u128,
    r: i128,
}

macro_rules! make_test {
    ($name:ident, $val:expr) => {
        fn $name(mut cx: FunctionContext) -> JsResult<JsValue> {
            let value = $val;

            neon_serde::to_value(&mut cx, &value).or_else(|e| cx.throw_error(e.to_string()))
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
        p: vec![1., 2., 3.5],
        q: 999,
        r: 333,
    };
    value
});

const NUMBER_BYTES: &'static [u8] = &[255u8, 254, 253];

make_test!(make_buff, { serde_bytes::Bytes::new(NUMBER_BYTES) });

macro_rules! make_expect {
    ($name:ident, $val:expr, $val_type:ty) => {
        fn $name(mut cx: FunctionContext) -> JsResult<JsValue> {
            let value = $val;
            let arg0 = cx.argument::<JsValue>(0)?;

            let de_serialized: $val_type = match neon_serde::from_value(&mut cx, arg0) {
                Ok(value) => value,
                Err(e) => {
                    return cx.throw_error(e.to_string());
                }
            };
            assert_eq!(value, de_serialized);
            Ok(JsUndefined::new().upcast())
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
        p: vec![1., 2., 3.5],
        q: 999,
        r: 333
    },
    AnObjectTwo
);

make_expect!(expect_num_array, vec![0, 1, 2, 3], Vec<i32>);

make_expect!(
    expect_buffer,
    serde_bytes::ByteBuf::from(vec![252u8, 251, 250]),
    serde_bytes::ByteBuf
);

fn roundtrip_object(mut cx: FunctionContext) -> JsResult<JsValue> {
    let arg0 = cx.argument::<JsValue>(0)?;

    let de_serialized: AnObjectTwo = neon_serde::from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();
    let handle = neon_serde::to_value(&mut cx, &de_serialized)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();
    Ok(handle)
}

register_module!(mut m, {
    m.export_function("make_num_77", make_num_77)?;
    m.export_function("make_num_32", make_num_32)?;
    m.export_function("make_str_hello", make_str_hello)?;
    m.export_function("make_num_array", make_num_array)?;
    m.export_function("make_buff", make_buff)?;
    m.export_function("make_obj", make_obj)?;
    m.export_function("make_object", make_object)?;
    m.export_function("make_map", make_map)?;

    m.export_function("expect_hello_world", expect_hello_world)?;
    m.export_function("expect_obj", expect_obj)?;
    m.export_function("expect_num_array", expect_num_array)?;
    m.export_function("expect_buffer", expect_buffer)?;

    m.export_function("roundtrip_object", roundtrip_object)?;
    Ok(())
});
