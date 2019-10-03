//!
//! Deserialize a `JsValue` into a Rust data structure
//!

use errors::Error as LibError;
use errors::ErrorKind;
use errors::Result as LibResult;
use neon::prelude::*;
use serde;
use serde::de::Visitor;
use serde::de::{DeserializeOwned, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, Unexpected,
                VariantAccess};

/// Deserialize an instance of type `T` from a `Handle<JsValue>`
///
/// # Errors
///
/// Can fail for various reasons see `ErrorKind`
///
pub fn from_value<'j, C, T>(cx: &mut C, value: Handle<'j, JsValue>) -> LibResult<T>
where
    C: Context<'j>,
    T: DeserializeOwned + ?Sized,
{
    let mut deserializer: Deserializer<C> = Deserializer::new(cx, value);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

pub fn from_value_opt<'j, C, T>(cx: &mut C, value: Option<Handle<'j, JsValue>>) -> LibResult<T>
where
    C: Context<'j>,
    T: DeserializeOwned + ?Sized,
{
    let unwrapped = value.unwrap_or_else(|| JsUndefined::new().upcast());
    from_value(cx, unwrapped)
}

#[doc(hidden)]
pub struct Deserializer<'a, 'j, C: Context<'j> + 'a> {
    cx: &'a mut C,
    input: Handle<'j, JsValue>,
}

#[doc(hidden)]
impl<'a, 'j, C: Context<'j>> Deserializer<'a, 'j, C> {
    fn new(cx: &'a mut C, input: Handle<'j, JsValue>) -> Self {
        Deserializer { cx, input }
    }
}

#[doc(hidden)]
impl<'x, 'd, 'a, 'j, C: Context<'j>> serde::de::Deserializer<'x> for &'d mut Deserializer<'a, 'j, C> {
    type Error = LibError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        if self.input.downcast::<JsNull>().is_ok() || self.input.downcast::<JsUndefined>().is_ok() {
            visitor.visit_unit()
        } else if let Ok(val) = self.input.downcast::<JsBoolean>() {
            visitor.visit_bool(val.value())
        } else if let Ok(val) = self.input.downcast::<JsString>() {
            visitor.visit_string(val.value())
        } else if let Ok(val) = self.input.downcast::<JsNumber>() {
            let v = val.value();
            if v.trunc() == v {
                visitor.visit_i64(v as i64)
            } else {
                visitor.visit_f64(v)
            }
        } else if let Ok(_val) = self.input.downcast::<JsBuffer>() {
            self.deserialize_bytes(visitor)
        } else if let Ok(val) = self.input.downcast::<JsArray>() {
            let mut deserializer = JsArrayAccess::new(self.cx, val);
            visitor.visit_seq(&mut deserializer)
        } else if let Ok(val) = self.input.downcast::<JsObject>() {
            let mut deserializer = JsObjectAccess::new(self.cx, val)?;
            visitor.visit_map(&mut deserializer)
        } else {
            bail!(ErrorKind::NotImplemented(
                "unimplemented Deserializer::Deserializer",
            ));
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        if self.input.downcast::<JsNull>().is_ok() || self.input.downcast::<JsUndefined>().is_ok() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
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
        if let Ok(val) = self.input.downcast::<JsString>() {
            visitor.visit_enum(JsEnumAccess::new(self.cx, val.value(), None))
        } else if let Ok(val) = self.input.downcast::<JsObject>() {
            let prop_names = val.get_own_property_names(self.cx)?;
            let len = prop_names.len();
            if len != 1 {
                Err(ErrorKind::InvalidKeyType(format!(
                    "object key with {} properties",
                    len
                )))?
            }
            let key = prop_names.get(self.cx, 0)?.downcast::<JsString>().or_throw(self.cx)?;
            let enum_value = val.get(self.cx, key)?;
            visitor.visit_enum(JsEnumAccess::new(self.cx, key.value(), Some(enum_value)))
        } else {
            let m = self.input.to_string(self.cx)?.value();
            Err(ErrorKind::InvalidKeyType(m))?
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        let buff = self.input.downcast::<JsBuffer>().or_throw(self.cx)?;
        let copy = self.cx.borrow(&buff, |buff| Vec::from(buff.as_slice()));
        visitor.visit_bytes(&copy)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        let buff = self.input.downcast::<JsBuffer>().or_throw(self.cx)?;
        let copy = self.cx.borrow(&buff, |buff| Vec::from(buff.as_slice()));
        visitor.visit_byte_buf(copy)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'x>,
    {
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
       <V: Visitor<'x>>
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        unit unit_struct seq tuple tuple_struct map struct identifier
        newtype_struct
    }
}

#[doc(hidden)]
struct JsArrayAccess<'a, 'j, C: Context<'j> + 'a> {
    cx: &'a mut C,
    input: Handle<'j, JsArray>,
    idx: u32,
    len: u32,
}

#[doc(hidden)]
impl<'a, 'j, C: Context<'j>> JsArrayAccess<'a, 'j, C> {
    fn new(cx: &'a mut C, input: Handle<'j, JsArray>) -> Self {
        JsArrayAccess {
            cx,
            input,
            idx: 0,
            len: input.len(),
        }
    }
}

#[doc(hidden)]
impl<'x, 'a, 'j, C: Context<'j>> SeqAccess<'x> for JsArrayAccess<'a, 'j, C> {
    type Error = LibError;

    fn next_element_seed<T>(&mut self, seed: T) -> LibResult<Option<T::Value>>
    where
        T: DeserializeSeed<'x>,
    {
        if self.idx >= self.len {
            return Ok(None);
        }
        let v = self.input.get(self.cx, self.idx)?;
        self.idx += 1;

        let mut de = Deserializer::new(self.cx, v);
        seed.deserialize(&mut de).map(Some)
    }
}

#[doc(hidden)]
struct JsObjectAccess<'a, 'j, C: Context<'j> + 'a> {
    cx: &'a mut C,
    input: Handle<'j, JsObject>,
    prop_names: Handle<'j, JsArray>,
    idx: u32,
    len: u32,
}

#[doc(hidden)]
impl<'x, 'a, 'j, C: Context<'j>> JsObjectAccess<'a, 'j, C> {
    fn new(cx: &'a mut C, input: Handle<'j, JsObject>) -> LibResult<Self> {
        let prop_names = input.get_own_property_names(cx)?;
        let len = prop_names.len();

        Ok(JsObjectAccess {
            cx,
            input,
            prop_names,
            idx: 0,
            len,
        })
    }
}

#[doc(hidden)]
impl<'x, 'a, 'j, C: Context<'j>> MapAccess<'x> for JsObjectAccess<'a, 'j, C> {
    type Error = LibError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'x>,
    {
        if self.idx >= self.len {
            return Ok(None);
        }

        let prop_name = self.prop_names.get(self.cx, self.idx)?;

        let mut de = Deserializer::new(self.cx, prop_name);
        seed.deserialize(&mut de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'x>,
    {
        if self.idx >= self.len {
            return Err(ErrorKind::ArrayIndexOutOfBounds(self.len, self.idx))?;
        }
        let prop_name = self.prop_names.get(self.cx, self.idx)?;
        let value = self.input.get(self.cx, prop_name)?;

        self.idx += 1;
        let mut de = Deserializer::new(self.cx, value);
        let res = seed.deserialize(&mut de)?;
        Ok(res)
    }
}

#[doc(hidden)]
struct JsEnumAccess<'a, 'j, C: Context<'j> + 'a> {
    cx: &'a mut C,
    variant: String,
    value: Option<Handle<'j, JsValue>>,
}

#[doc(hidden)]
impl<'a, 'j, C: Context<'j>> JsEnumAccess<'a, 'j, C> {
    fn new(cx: &'a mut C, key: String, value: Option<Handle<'j, JsValue>>) -> Self {
        JsEnumAccess {
            cx,
            variant: key,
            value,
        }
    }
}

#[doc(hidden)]
impl<'x, 'a, 'j, C: Context<'j>> EnumAccess<'x> for JsEnumAccess<'a, 'j, C> {
    type Error = LibError;
    type Variant = JsVariantAccess<'a, 'j, C>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'x>,
    {
        use serde::de::IntoDeserializer;
        let variant = self.variant.into_deserializer();
        let variant_access = JsVariantAccess::new(self.cx, self.value);
        seed.deserialize(variant).map(|v| (v, variant_access))
    }
}

#[doc(hidden)]
struct JsVariantAccess<'a, 'j, C: Context<'j> + 'a> {
    cx: &'a mut C,
    value: Option<Handle<'j, JsValue>>,
}

#[doc(hidden)]
impl<'a, 'j, C: Context<'j>> JsVariantAccess<'a, 'j, C> {
    fn new(cx: &'a mut C, value: Option<Handle<'j, JsValue>>) -> Self {
        JsVariantAccess { cx, value }
    }
}

#[doc(hidden)]
impl<'x, 'a, 'j, C: Context<'j>> VariantAccess<'x> for JsVariantAccess<'a, 'j, C> {
    type Error = LibError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Some(val) => {
                let mut deserializer = Deserializer::new(self.cx, val);
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
                let mut deserializer = Deserializer::new(self.cx, val);
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
        match self.value {
            Some(handle) => {
                if let Ok(val) = handle.downcast::<JsArray>() {
                    let mut deserializer = JsArrayAccess::new(self.cx, val);
                    visitor.visit_seq(&mut deserializer)
                } else {
                    Err(serde::de::Error::invalid_type(
                        Unexpected::Other("JsValue"),
                        &"tuple variant",
                    ))
                }
            },
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
        match self.value {
            Some(handle) => {
                if let Ok(val) = handle.downcast::<JsObject>() {
                    let mut deserializer = JsObjectAccess::new(self.cx, val)?;
                    visitor.visit_map(&mut deserializer)
                } else {
                    Err(serde::de::Error::invalid_type(
                        Unexpected::Other("JsValue"),
                        &"struct variant",
                    ))
                }
            },
            _ => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}
