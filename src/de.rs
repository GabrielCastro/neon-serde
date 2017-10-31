//!
//! Deserialize a `JsValue` into a Rust data structure
//!

use errors::Error as LibError;
use errors::ErrorKind;
use errors::Result as LibResult;
use neon::js;
use neon::js::Value;
use neon::mem::Handle;
use neon::scope::Scope;
use neon::vm::Lock;
use serde;
use serde::de::Visitor;

use cast;
use neon::js::Object;
use neon::js::Variant;
use serde::Deserializer as __0;
use serde::de::{DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess};

/// Deserialize an instance of type `T` from a `Handle<js::JsValue>`
///
/// # Errors
///
/// Can fail for various reasons see `ErrorKind`
///
pub fn from_handle<'a, T, S>(
    input: Handle<'a, js::JsValue>,
    scope: &'a mut S,
) -> LibResult<T>
where
    T: serde::Deserialize<'a> + ?Sized,
    S: Scope<'a> + 'a
{
    let mut deserializer: Deserializer<S> = Deserializer::new(input, scope);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

#[doc(hidden)]
pub struct Deserializer<'de, S: 'de + Scope<'de>> {
    input: Handle<'de, js::JsValue>,
    scope: &'de mut S,
}

#[doc(hidden)]
impl<'de, S: 'de + Scope<'de>> Deserializer<'de, S> {
    fn new(input: Handle<'de, js::JsValue>, scope: &'de mut S) -> Self {
        Deserializer { input, scope }
    }
}

#[doc(hidden)]
impl<'de, 'a, S: 'de + Scope<'de>> serde::de::Deserializer<'de> for &'a mut Deserializer<'de, S> {
    type Error = LibError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Variant::Undefined(_) => visitor.visit_none(),
            Variant::Null(_) => visitor.visit_unit(),
            Variant::Boolean(val) => visitor.visit_bool(val.value()),
            Variant::String(val) => visitor.visit_string(val.value()),
            Variant::Integer(val) => visitor.visit_i64(val.value()), // TODO is u32 or i32,
            Variant::Number(val) => visitor.visit_f64(val.value()),
            Variant::Array(_) => self.deserialize_seq(visitor),
            Variant::Object(_) => self.deserialize_map(visitor),
            Variant::Function(_) => {
                bail!(ErrorKind::NotImplemented(
                    "unimplemented Deserializer::Deserializer(Function)",
                ));
            }
            Variant::Other(_) => {
                bail!(ErrorKind::NotImplemented(
                    "unimplemented Deserializer::Deserializer(Other)",
                ));
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Variant::Null(_) |
            Variant::Undefined(_) => visitor.visit_bool(false),
            Variant::Boolean(val) => visitor.visit_bool(val.value()),
            Variant::Number(val) => {
                let num = val.value();
                visitor.visit_bool(num != 0.0)
            }
            _ => Err(ErrorKind::UnableToCoerce("type cannot be made into bool"))?,
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
            None => Err(ErrorKind::EmptyString)?,
        };

        let num_left = chars.count();

        if num_left > 0 {
            Err(ErrorKind::StringTooLongForChar(num_left + 1))?
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
        let mut buff = self.input.check::<js::binary::JsBuffer>()?;
        let copy = buff.grab(|buff| {
            let buff_slice = buff.as_slice();
            // TODO?: use Vec::with_capacity(buff_slice.len()); vec.set_len();
            let mut copy: Vec<u8> = vec![0; buff_slice.len()];
            copy.copy_from_slice(buff_slice);
            copy
        });
        visitor.visit_bytes(&copy)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut buff = self.input.check::<js::binary::JsBuffer>()?;
        let copy = buff.grab(|buff| {
            let buff_slice = buff.as_slice();
            let mut copy: Vec<u8> = vec![0; buff_slice.len()];
            copy.copy_from_slice(buff_slice);
            copy
        });
        visitor.visit_byte_buf(copy)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Variant::Null(_) |
            Variant::Undefined(_) => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Variant::Null(_) |
            Variant::Undefined(_) => visitor.visit_unit(),
            _ => Err(ErrorKind::ExpectingNull)?,
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
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

    fn deserialize_tuple<V>(mut self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(JsArrayAccess::new(&mut self)?)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
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
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use serde::de::IntoDeserializer;
        match self.input.variant() {
            Variant::String(val) => visitor.visit_enum(val.value().into_deserializer()),
            Variant::Object(val) => {
                let prop_names = val.get_own_property_names(self.scope)?;
                let len = prop_names.len();
                if len != 1 {
                    Err(ErrorKind::InvalidKeyType(
                        format!("object key with {} properties", len),
                    ))?
                }
                let key = prop_names.get(self.scope, 0)?;
                let enum_value = val.get(self.scope, key)?;
                visitor.visit_enum(JsEnumAccess::new(self, enum_value)?)
            }
            _ => {
                let m = self.input.to_string(self.scope)?.value();
                Err(ErrorKind::InvalidKeyType(m))?
            }
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.input.variant() {
            Variant::String(val) => visitor.visit_string(val.value()),
            Variant::Number(val) => visitor.visit_f64(val.value()),
            Variant::Object(val) => {
                let prop_names = val.get_own_property_names(self.scope)?;
                let len = prop_names.len();
                if len != 1 {
                    Err(ErrorKind::InvalidKeyType(
                        format!("object key with {} properties", len),
                    ))?
                }
                let key = prop_names.get(self.scope, 0)?;
                let key_str = key.to_string(self.scope)?.value();
                visitor.visit_string(key_str)
            }
            _ => {
                let m = self.input.to_string(self.scope)?.value();
                Err(ErrorKind::InvalidKeyType(m))?
            }
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

#[doc(hidden)]
struct JsArrayAccess<'a, 'de: 'a, S: 'de + Scope<'de>> {
    de: &'a mut Deserializer<'de, S>,
    idx: u32,
    len: u32,
}

#[doc(hidden)]
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

#[doc(hidden)]
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

#[doc(hidden)]
struct JsObjectAccess<'a, 'de: 'a, S: 'de + Scope<'de>> {
    de: &'a mut Deserializer<'de, S>,
    prop_names: Handle<'a, js::JsArray>,
    idx: u32,
    len: u32,
}

#[doc(hidden)]
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

#[doc(hidden)]
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
            return Err(ErrorKind::ArrayIndexOutOfBounds(self.len, self.idx))?;
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

#[doc(hidden)]
struct JsEnumAccess<'a, 'de: 'a, S: 'de + Scope<'de>> {
    de: &'a mut Deserializer<'de, S>,
    value: Handle<'de, js::JsValue>,
}

#[doc(hidden)]
impl<'a, 'de, S: 'de + Scope<'de>> JsEnumAccess<'a, 'de, S> {
    fn new(de: &'a mut Deserializer<'de, S>, value: Handle<'de, js::JsValue>) -> LibResult<Self> {
        Ok(JsEnumAccess { de, value })
    }
}

#[doc(hidden)]
impl<'a, 'de, S: 'de + Scope<'de>> EnumAccess<'de> for JsEnumAccess<'a, 'de, S> {
    type Error = LibError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        let access = self;
        Ok((val, access))
    }
}

#[doc(hidden)]
impl<'a, 'de, S: 'de + Scope<'de>> VariantAccess<'de> for JsEnumAccess<'a, 'de, S> {
    type Error = LibError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        let old = self.de.input;
        self.de.input = self.value;
        let res = seed.deserialize(&mut *self.de);
        self.de.input = old;
        res
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let old = self.de.input;
        self.de.input = self.value;
        let res = self.de.deserialize_seq(visitor);
        self.de.input = old;
        res
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let old = self.de.input;
        self.de.input = self.value;
        let res = self.de.deserialize_map(visitor);
        self.de.input = old;
        res
    }
}
