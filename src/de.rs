use serde;
use errors::Result as LibResult;
use errors::Error as LibError;
use serde::de::Visitor;
use neon::mem::Handle;
use neon::scope::RootScope;
use neon::js;
use neon::js::binary::JsBuffer;
use serde::de::{MapAccess,DeserializeSeed};

use neon::js::Variant::*;
use cast;


pub fn from_handle<'a, T>(input: Handle<'a, js::JsValue>, scope: &'a RootScope<'a>) -> LibResult<T>
where
    T: serde::Deserialize<'a> + ?Sized,
{
    let mut deserializer = Deserializer::new(input, scope);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}


pub struct Deserializer<'de> {
    input: Handle<'de, js::JsValue>,
    scope: &'de RootScope<'de>
}

impl<'de> Deserializer<'de> {
    fn new(input: Handle<'de, js::JsValue>, scope: &'de RootScope<'de>) -> Self {
        Deserializer { input, scope }
    }

    fn visit_js_string(&mut self, input: Handle<'de, js::JsString>) {}
}

impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = LibError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Null(_) => visitor.visit_unit(),
            String(val) => visitor.visit_string("A".into()),
            _ => unimplemented!(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Null(_) => visitor.visit_bool(false),
            Undefined(_) => visitor.visit_bool(false),
            Boolean(val) => {
                visitor.visit_bool(val.value())
            }
            Number(val) => {
                let num = val.value();
                visitor.visit_bool(num != 0.0)
            }
            _ => {
                self.input.check::<js::JsNumber>()?;
                unreachable!();
            }
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_num = self.input.check::<js::JsNumber>()?;
        let input_num_value = cast::i8(input_num.value())?;
        visitor.visit_i8(input_num_value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_num = self.input.check::<js::JsNumber>()?;
        let input_num_value = cast::i16(input_num.value())?;
        visitor.visit_i16(input_num_value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_num = self.input.check::<js::JsNumber>()?;
        let input_num_value = cast::i32(input_num.value())?;
        visitor.visit_i32(input_num_value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_num = self.input.check::<js::JsNumber>()?;
        let input_num_value = cast::i64(input_num.value())?;
        visitor.visit_i64(input_num_value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_num = self.input.check::<js::JsNumber>()?;
        let input_num_value = cast::u8(input_num.value())?;
        visitor.visit_u8(input_num_value)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_num = self.input.check::<js::JsNumber>()?;
        let input_num_value = cast::u16(input_num.value())?;
        visitor.visit_u16(input_num_value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_num = self.input.check::<js::JsNumber>()?;
        let input_num_value = cast::u32(input_num.value())?;
        visitor.visit_u32(input_num_value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_num = self.input.check::<js::JsNumber>()?;
        let input_num_value = cast::u64(input_num.value())?;
        visitor.visit_u64(input_num_value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_num = self.input.check::<js::JsNumber>()?;
        let input_num_value = cast::f32(input_num.value())?;
        visitor.visit_f32(input_num_value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_num = self.input.check::<js::JsNumber>()?;
        let input_num_value = cast::f64(input_num.value());
        visitor.visit_f64(input_num_value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_str = self.input.check::<js::JsString>()?;
        let input_string = input_str.value();
        visitor.visit_string(input_string)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {

        unimplemented!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

