//!
//! Serialize a Rust data structure into a `JsValue`
//!

use cast;
use errors::Error;
use errors::ErrorKind;
use errors::Result as LibResult;
use neon::js;
use neon::js::Object;
use neon::mem::Handle;
use neon::scope::Scope;
use neon::vm::Lock;
use serde::ser::{self, Serialize};
use std::marker::PhantomData;

/// Converts a value of type `V` to a `JsValue`
///
/// # Errors
///
/// * `NumberCastError` trying to serialize a `u64` can fail if it overflows in a cast to `f64`
/// * `StringTooLong` if the string exceeds v8's max string size
///
#[inline]
pub fn to_value<'j, S, V>(scope: &mut S, value: &V) -> LibResult<Handle<'j, js::JsValue>>
where
    S: Scope<'j>,
    V: Serialize + ?Sized,
{
    let serializer = Serializer {
        scope,
        ph: PhantomData,
    };
    let serialized_value = value.serialize(serializer)?;
    Ok(serialized_value)
}

#[doc(hidden)]
pub struct Serializer<'a, 'j, S: 'a>
where
    S: Scope<'j>,
{
    scope: &'a mut S,
    ph: PhantomData<&'j ()>,
}

#[doc(hidden)]
pub struct ArraySerializer<'a, 'j, S: 'a>
where
    S: Scope<'j>,
{
    scope: &'a mut S,
    array: Handle<'j, js::JsArray>,
}

#[doc(hidden)]
pub struct TupleVariantSerializer<'a, 'j, S: 'a>
where
    S: Scope<'j>,
{
    outter_object: Handle<'j, js::JsObject>,
    inner: ArraySerializer<'a, 'j, S>,
}

#[doc(hidden)]
pub struct MapSerializer<'a, 'j, S: 'a>
where
    S: Scope<'j>,
{
    scope: &'a mut S,
    object: Handle<'j, js::JsObject>,
    key_holder: Handle<'j, js::JsObject>,
}

#[doc(hidden)]
pub struct StructSerializer<'a, 'j, S: 'a>
where
    S: Scope<'j>,
{
    scope: &'a mut S,
    object: Handle<'j, js::JsObject>,
}

#[doc(hidden)]
pub struct StructVariantSerializer<'a, 'j, S: 'a>
where
    S: Scope<'j>,
{
    outer_object: Handle<'j, js::JsObject>,
    inner: StructSerializer<'a, 'j, S>,
}

#[doc(hidden)]
impl<'a, 'j, S> ser::Serializer for Serializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    type Ok = Handle<'j, js::JsValue>;
    type Error = Error;

    type SerializeSeq = ArraySerializer<'a, 'j, S>;
    type SerializeTuple = ArraySerializer<'a, 'j, S>;
    type SerializeTupleStruct = ArraySerializer<'a, 'j, S>;
    type SerializeTupleVariant = TupleVariantSerializer<'a, 'j, S>;
    type SerializeMap = MapSerializer<'a, 'j, S>;
    type SerializeStruct = StructSerializer<'a, 'j, S>;
    type SerializeStructVariant = StructVariantSerializer<'a, 'j, S>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsBoolean::new(self.scope, v).upcast())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, v).upcast())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut b = [0; 4];
        let result = v.encode_utf8(&mut b);
        let js_str = js::JsString::new(self.scope, result)
            .ok_or_else(|| ErrorKind::StringTooLongForChar(4))?;
        Ok(js_str.upcast())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let len = v.len();
        let js_str = js::JsString::new(self.scope, v).ok_or_else(|| ErrorKind::StringTooLong(len))?;
        Ok(js_str.upcast())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut buff = js::binary::JsBuffer::new(self.scope, cast::u32(v.len())?)?;
        buff.grab(|mut buff| buff.as_mut_slice().clone_from_slice(v));
        Ok(buff.upcast())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNull::new().upcast())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNull::new().upcast())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNull::new().upcast())
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let obj = js::JsObject::new(&mut *self.scope);
        let value_js = to_value(self.scope, value)?;
        obj.set(variant, value_js)?;

        Ok(obj.upcast())
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(ArraySerializer::new(self.scope))
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(ArraySerializer::new(self.scope))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(ArraySerializer::new(self.scope))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        TupleVariantSerializer::new(self.scope, variant)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self.scope))
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer::new(self.scope))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        StructVariantSerializer::new(self.scope, variant)
    }
}

#[doc(hidden)]
impl<'a, 'j, S> ArraySerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    #[inline]
    fn new(scope: &'a mut S) -> Self {
        let array = js::JsArray::new(scope, 0);
        ArraySerializer { scope, array }
    }
}

#[doc(hidden)]
impl<'a, 'j, S> ser::SerializeSeq for ArraySerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    type Ok = Handle<'j, js::JsValue>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = to_value(self.scope, value)?;

        let arr: Handle<'j, js::JsArray> = self.array;
        let len = arr.len();
        arr.set(len, value)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array.upcast())
    }
}

impl<'a, 'j, S> ser::SerializeTuple for ArraySerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    type Ok = Handle<'j, js::JsValue>;
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

#[doc(hidden)]
impl<'a, 'j, S> ser::SerializeTupleStruct for ArraySerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    type Ok = Handle<'j, js::JsValue>;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

#[doc(hidden)]
impl<'a, 'j, S> TupleVariantSerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    fn new(scope: &'a mut S, key: &'static str) -> LibResult<Self> {
        let inner_array = js::JsArray::new(scope, 0);
        let outter_object = js::JsObject::new(scope);
        outter_object.set(key, inner_array)?;
        Ok(TupleVariantSerializer {
            outter_object,
            inner: ArraySerializer {
                scope,
                array: inner_array,
            },
        })
    }
}

#[doc(hidden)]
impl<'a, 'j, S> ser::SerializeTupleVariant for TupleVariantSerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    type Ok = Handle<'j, js::JsValue>;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        use serde::ser::SerializeSeq;
        self.inner.serialize_element(value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.outter_object.upcast())
    }
}

#[doc(hidden)]
impl<'a, 'j, S> MapSerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    fn new(scope: &'a mut S) -> Self {
        let object = js::JsObject::new(scope);
        let key_holder = js::JsObject::new(scope);
        MapSerializer {
            scope,
            object,
            key_holder,
        }
    }
}

#[doc(hidden)]
impl<'a, 'j, S> ser::SerializeMap for MapSerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    type Ok = Handle<'j, js::JsValue>;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = to_value(self.scope, key)?;
        self.key_holder.set("key", key)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key: Handle<'j, js::JsValue> = self.key_holder.get(&mut *self.scope, "key")?;
        let value_obj = to_value(self.scope, value)?;
        self.object.set(key, value_obj)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.object.upcast())
    }
}

#[doc(hidden)]
impl<'a, 'j, S> StructSerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    #[inline]
    fn new(scope: &'a mut S) -> Self {
        let object = js::JsObject::new(scope);
        StructSerializer { scope, object }
    }
}

#[doc(hidden)]
impl<'a, 'j, S> ser::SerializeStruct for StructSerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    type Ok = Handle<'j, js::JsValue>;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = to_value(self.scope, value)?;
        self.object.set(key, value)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.object.upcast())
    }
}

#[doc(hidden)]
impl<'a, 'j, S> StructVariantSerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    fn new(scope: &'a mut S, key: &'static str) -> LibResult<Self> {
        let inner_object = js::JsObject::new(scope);
        let outter_object = js::JsObject::new(scope);
        outter_object.set(key, inner_object)?;
        Ok(StructVariantSerializer {
            outer_object: outter_object,
            inner: StructSerializer {
                scope,
                object: inner_object,
            },
        })
    }
}

#[doc(hidden)]
impl<'a, 'j, S> ser::SerializeStructVariant for StructVariantSerializer<'a, 'j, S>
where
    S: Scope<'j>,
{
    type Ok = Handle<'j, js::JsValue>;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        use serde::ser::SerializeStruct;
        self.inner.serialize_field(key, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.outer_object.upcast())
    }
}
