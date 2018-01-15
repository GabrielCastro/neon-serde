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
use neon::js::Value;
use neon::js::Variant;
use neon::vm::Lock;
use serde::de::{DeserializeOwned, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, Unexpected,
                VariantAccess};

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
pub struct Deserializer<'a, 'j, S: Scope<'j> + 'a> {
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
            Variant::Null(_) | Variant::Undefined(_) => visitor.visit_unit(),
            Variant::Boolean(val) => visitor.visit_bool(val.value()),
            Variant::String(val) => visitor.visit_string(val.value()),
            Variant::Integer(val) => visitor.visit_i64(val.value()), // TODO is u32 or i32,
            Variant::Number(val) => visitor.visit_f64(val.value()),
            Variant::Array(val) => {
                let mut deserializer = JsArrayAccess::new(self.scope, val);
                visitor.visit_seq(&mut deserializer)
            }
            Variant::Object(val) => {
                let mut deserializer = JsObjectAccess::new(self.scope, val)?;
                visitor.visit_map(&mut deserializer)
            }
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

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        match self.input.variant() {
            Variant::Null(_) | Variant::Undefined(_) => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        match self.input.variant() {
            Variant::String(val) => {
                visitor.visit_enum(JsEnumAccess::new(self.scope, val.value(), None))
            }
            Variant::Object(val) => {
                let prop_names = val.get_own_property_names(self.scope)?;
                let len = prop_names.len();
                if len != 1 {
                    Err(ErrorKind::InvalidKeyType(format!(
                        "object key with {} properties",
                        len
                    )))?
                }
                let key = prop_names.get(self.scope, 0)?.check::<js::JsString>()?;
                let enum_value = val.get(self.scope, key)?;
                visitor.visit_enum(JsEnumAccess::new(self.scope, key.value(), Some(enum_value)))
            }
            _ => {
                let m = self.input.to_string(self.scope)?.value();
                Err(ErrorKind::InvalidKeyType(m))?
            }
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        let mut buff = self.input.check::<js::binary::JsBuffer>()?;
        let copy = buff.grab(|buff| Vec::from(buff.as_slice()));
        visitor.visit_bytes(&copy)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        let mut buff = self.input.check::<js::binary::JsBuffer>()?;
        let copy = buff.grab(|buff| Vec::from(buff.as_slice()));
        visitor.visit_byte_buf(copy)
    }

    forward_to_deserialize_any! {
       <V: Visitor<'x>>
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        unit unit_struct seq tuple tuple_struct map struct identifier
        newtype_struct ignored_any
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
    fn new(scope: &'a mut S, input: Handle<'j, js::JsArray>) -> Self {
        JsArrayAccess {
            scope,
            input,
            idx: 0,
            len: input.len(),
        }
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
        return seed.deserialize(&mut de).map(Some);
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

#[doc(hidden)]
struct JsEnumAccess<'a, 'j, S: Scope<'j> + 'a> {
    scope: &'a mut S,
    variant: String,
    value: Option<Handle<'j, js::JsValue>>,
}

#[doc(hidden)]
impl<'a, 'j, S: Scope<'j>> JsEnumAccess<'a, 'j, S> {
    fn new(scope: &'a mut S, key: String, value: Option<Handle<'j, js::JsValue>>) -> Self {
        JsEnumAccess {
            scope,
            variant: key,
            value,
        }
    }
}

#[doc(hidden)]
impl<'x, 'a, 'j, S: Scope<'j>> EnumAccess<'x> for JsEnumAccess<'a, 'j, S> {
    type Error = LibError;
    type Variant = JsVariantAccess<'a, 'j, S>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'x>,
    {
        use serde::de::IntoDeserializer;
        let variant = self.variant.into_deserializer();
        let variant_access = JsVariantAccess::new(self.scope, self.value);
        seed.deserialize(variant).map(|v| (v, variant_access))
    }
}

#[doc(hidden)]
struct JsVariantAccess<'a, 'j, S: Scope<'j> + 'a> {
    scope: &'a mut S,
    value: Option<Handle<'j, js::JsValue>>,
}

#[doc(hidden)]
impl<'a, 'j, S: Scope<'j>> JsVariantAccess<'a, 'j, S> {
    fn new(scope: &'a mut S, value: Option<Handle<'j, js::JsValue>>) -> Self {
        JsVariantAccess { scope, value }
    }
}

#[doc(hidden)]
impl<'x, 'a, 'j, S: Scope<'j>> VariantAccess<'x> for JsVariantAccess<'a, 'j, S> {
    type Error = LibError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Some(val) => {
                let mut deserializer = Deserializer::new(self.scope, val);
                serde::de::Deserialize::deserialize(&mut deserializer)
            }
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'x>,
    {
        match self.value {
            Some(val) => {
                let mut deserializer = Deserializer::new(self.scope, val);
                seed.deserialize(&mut deserializer)
            }
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        match self.value.map(|v| v.variant()) {
            Some(Variant::Array(val)) => {
                let mut deserializer = JsArrayAccess::new(self.scope, val);
                visitor.visit_seq(&mut deserializer)
            }
            Some(_) => Err(serde::de::Error::invalid_type(
                Unexpected::Other("JsValue"),
                &"tuple variant",
            )),
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        match self.value.map(|v| v.variant()) {
            Some(Variant::Object(val)) => {
                let mut deserializer = JsObjectAccess::new(self.scope, val)?;
                visitor.visit_map(&mut deserializer)
            }
            Some(_) => Err(serde::de::Error::invalid_type(
                Unexpected::Other("JsValue"),
                &"struct variant",
            )),
            _ => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}
