//!
//! Deserialize a `JsValue` into a Rust data structure
//!

use errors::Error as LibError;
use errors::ErrorKind;
use errors::Result as LibResult;
use neon::js;
use neon::mem::Handle;
use neon::scope::Scope;
use serde;
use serde::de::Visitor;

use neon::js::Object;
use neon::js::Variant;
use serde::de::{DeserializeOwned, DeserializeSeed, MapAccess, SeqAccess};


/// Deserialize an instance of type `T` from a `Handle<js::JsValue>`
///
/// # Errors
///
/// Can fail for various reasons see `ErrorKind`
///
pub fn from_value<'j, S, T>(scope: &mut S, value: Handle<'j, js::JsValue>) -> LibResult<T>
where
    S: Scope<'j>,
    T: DeserializeOwned + ?Sized,
{
    let mut deserializer: Deserializer<S> = Deserializer::new(scope, value);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

#[doc(hidden)]
pub struct Deserializer<'a, 'j, S: Scope<'j> +'a> {
    scope: &'a mut S,
    input: Handle<'j, js::JsValue>,
}

#[doc(hidden)]
impl<'a, 'j, S: Scope<'j>> Deserializer<'a, 'j, S> {
    fn new(scope: &'a mut S, input: Handle<'j, js::JsValue>) -> Self {
        Deserializer { scope, input }
    }
}

#[doc(hidden)]
impl<'x, 'd, 'a, 'j, S: Scope<'j>> serde::de::Deserializer<'x> for &'d mut Deserializer<'a, 'j, S> {
    type Error = LibError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
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

    forward_to_deserialize_any! {
       <V: Visitor<'x>>
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct tuple tuple_struct
        struct enum identifier ignored_any
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        let input = self.input.check::<js::JsArray>()?;
        visitor.visit_seq(JsArrayAccess::new(self.scope, input)?)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'x>,
    {
        let input = self.input.check::<js::JsObject>()?;
        visitor.visit_map(JsObjectAccess::new(self.scope, input)?)
    }
}

#[doc(hidden)]
struct JsArrayAccess<'a, 'j, S: Scope<'j> + 'a> {
    scope: &'a mut S,
    input: Handle<'j, js::JsArray>,
    idx: u32,
    len: u32,
}

#[doc(hidden)]
impl<'a, 'j, S: Scope<'j>> JsArrayAccess<'a, 'j, S> {
    fn new(scope: &'a mut S, input: Handle<'j, js::JsArray>) -> LibResult<Self> {
        let len = input.len();
        Ok(JsArrayAccess {
            scope,
            input,
            idx: 0,
            len,
        })
    }
}

#[doc(hidden)]
impl<'x, 'a, 'j, S: Scope<'j>> SeqAccess<'x> for JsArrayAccess<'a, 'j, S> {
    type Error = LibError;

    fn next_element_seed<T>(&mut self, seed: T) -> LibResult<Option<T::Value>>
    where
        T: DeserializeSeed<'x>,
    {
        if self.idx >= self.len {
            return Ok(None);
        }
        let v = self.input.get(self.scope, self.idx)?;
        self.idx += 1;

        let mut de = Deserializer::new(self.scope, v);
        return seed.deserialize(&mut de).map(Some)
    }
}

#[doc(hidden)]
struct JsObjectAccess<'a, 'j, S: Scope<'j> + 'a> {
    scope: &'a mut S,
    input: Handle<'j, js::JsObject>,
    prop_names: Handle<'j, js::JsArray>,
    idx: u32,
    len: u32,
}

#[doc(hidden)]
impl<'x, 'a, 'j, S: Scope<'j>> JsObjectAccess<'a, 'j, S> {
    fn new(scope: &'a mut S, input: Handle<'j, js::JsObject>) -> LibResult<Self> {
        let prop_names = input.get_own_property_names(scope)?;
        let len = prop_names.len();

        Ok(JsObjectAccess {
            scope,
            input,
            prop_names,
            idx: 0,
            len,
        })
    }
}

#[doc(hidden)]
impl<'x, 'a, 'j, S: Scope<'j>> MapAccess<'x> for JsObjectAccess<'a, 'j, S> {
    type Error = LibError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'x>,
    {
        if self.idx >= self.len {
            return Ok(None);
        }

        let prop_name = self.prop_names.get(self.scope, self.idx)?;

        let mut de = Deserializer::new(self.scope, prop_name);
        seed.deserialize(&mut de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'x>,
    {
        if self.idx >= self.len {
            return Err(ErrorKind::ArrayIndexOutOfBounds(self.len, self.idx))?;
        }
        let prop_name = self.prop_names.get(self.scope, self.idx)?;
        let value = self.input.get(self.scope, prop_name)?;

        self.idx += 1;
        let mut de = Deserializer::new(self.scope, value);
        let res = seed.deserialize(&mut de)?;
        Ok(res)
    }
}
