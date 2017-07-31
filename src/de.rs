use serde;
use errors::Result as LibResult;
use errors::Error as LibError;
use errors::ErrorKind::*;
use serde::de::Visitor;
use neon::mem::Handle;
use neon::scope::{RootScope, Scope};
use neon::js;

use serde::de::{MapAccess, DeserializeSeed, SeqAccess};
use neon::js::Object;
use neon::js::Variant::*;
use cast;

use serde::Deserializer as _0;

pub fn from_handle<'a, T>(
    input: Handle<'a, js::JsValue>,
    scope: &'a mut RootScope<'a>,
) -> LibResult<T>
where
    T: serde::Deserialize<'a> + ?Sized,
{
    let mut deserializer: Deserializer<RootScope<'a>> = Deserializer::new(input, scope);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}


pub struct Deserializer<'de, S: 'de + Scope<'de>> {
    input: Handle<'de, js::JsValue>,
    scope: &'de mut S,
}

impl<'de, S: 'de + Scope<'de>> Deserializer<'de, S> {
    fn new(input: Handle<'de, js::JsValue>, scope: &'de mut S) -> Self {
        Deserializer { input, scope }
    }
}

impl<'de, 'a, S: 'de + Scope<'de>> serde::de::Deserializer<'de> for &'a mut Deserializer<'de, S> {
    type Error = LibError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Undefined(_) => visitor.visit_none(),
            Null(_) => visitor.visit_unit(),
            Boolean(val) => visitor.visit_bool(val.value()),
            String(val) => visitor.visit_string("A".into()),
            Integer(val) => visitor.visit_i64(val.value()), // TO is u32 or i32,
            Number(val) => visitor.visit_f64(val.value()),
            Array(val) => self.deserialize_seq(visitor),
            Object(val) => self.deserialize_map(visitor),
            _ => {
                println!("deserialize_any: unimplmented");
                unimplemented!()
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Null(_) => visitor.visit_bool(false),
            Undefined(_) => visitor.visit_bool(false),
            Boolean(val) => visitor.visit_bool(val.value()),
            Number(val) => {
                let num = val.value();
                visitor.visit_bool(num != 0.0)
            }
            _ => Err(UnableToCoerce("type cannot be made into bool"))?,
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
        let input_str = self.input.check::<js::JsString>()?;
        let input_string = input_str.value();
        let mut chars = input_string.chars();

        let result = match chars.next() {
            Some(ch) => visitor.visit_char(ch),
            None => Err(EmptyString)?,
        };

        let num_left = chars.count();

        if num_left > 0 {
            Err(StringTooLongForChar(num_left + 1))?
        }

        result
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let input_str = self.input.check::<js::JsString>()?;
        let input_string = input_str.value();
        visitor.visit_string(input_string)
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
        eprintln!("deserialize_bytes: unimplmented");
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        eprintln!("deserialize_byte_buf: unimplmented");
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Null(_) => visitor.visit_none(),
            Undefined(_) => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Null(_) => visitor.visit_unit(),
            Undefined(_) => visitor.visit_unit(),
            _ => Err(ExpectingNull)?,
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(JsArrayAccess::new(&mut self)?)
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(JsArrayAccess::new(&mut self)?)
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
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(JsObjectAccess::new(&mut self)?)
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
        eprintln!("deserialize_enum: unimplmented");
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            String(val) => visitor.visit_string(val.value()),
            Number(val) => visitor.visit_f64(val.value()),
            _ => Err(InvalidKeyType)?,
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}


struct JsArrayAccess<'a, 'de: 'a, S: 'de + Scope<'de>> {
    de: &'a mut Deserializer<'de, S>,
    idx: u32,
    len: u32,
}

impl<'a, 'de, S: 'de + Scope<'de>> JsArrayAccess<'a, 'de, S> {
    fn new(de: &'a mut Deserializer<'de, S>) -> LibResult<Self> {
        let len = de.input.check::<js::JsArray>()?.len();
        Ok(JsArrayAccess {
            de: de,
            idx: 0,
            len,
        })
    }
}

impl<'de, 'a, S: 'de + Scope<'de>> SeqAccess<'de> for JsArrayAccess<'a, 'de, S> {
    type Error = LibError;

    fn next_element_seed<T>(&mut self, seed: T) -> LibResult<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.idx >= self.len {
            return Ok(None);
        }
        let as_array = self.de.input.check::<js::JsArray>()?;
        let v = as_array.get(self.de.scope, self.idx)?;
        self.idx += 1;


        let old_input = self.de.input;
        self.de.input = v;

        let res = seed.deserialize(&mut *self.de).map(Some);

        self.de.input = old_input;

        res
    }
}


struct JsObjectAccess<'a, 'de: 'a, S: 'de + Scope<'de>> {
    de: &'a mut Deserializer<'de, S>,
    prop_names: Handle<'a, js::JsArray>,
    idx: u32,
    len: u32,
}

impl<'a, 'de, S: 'de + Scope<'de>> JsObjectAccess<'a, 'de, S> {
    fn new(de: &'a mut Deserializer<'de, S>) -> LibResult<Self> {
        let obj = de.input.check::<js::JsObject>()?;
        let prop_names = obj.get_own_property_names(de.scope)?;
        let len = prop_names.len();

        Ok(JsObjectAccess {
            de: de,
            idx: 0,
            prop_names,
            len,
        })
    }
}

impl<'de, 'a, S: 'de + Scope<'de>> MapAccess<'de> for JsObjectAccess<'a, 'de, S> {
    type Error = LibError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.idx >= self.len {
            return Ok(None);
        }

        let prop_name = self.prop_names.get(self.de.scope, self.idx)?;

        let old_input = self.de.input;
        self.de.input = prop_name;

        let res = seed.deserialize(&mut *self.de).map(Some);

        self.de.input = old_input;

        res
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if self.idx >= self.len {
            return Err(ArrayIndexOutOfBounds(self.len, self.idx))?;
        }
        let prop_name = self.prop_names.get(self.de.scope, self.idx)?;
        let obj = self.de.input.check::<js::JsObject>()?;

        let value = obj.get(self.de.scope, prop_name)?;


        let old_input = self.de.input;
        self.de.input = value;

        let res = seed.deserialize(&mut *self.de)?;

        self.de.input = old_input;

        self.idx += 1;
        Ok(res)
    }
}
