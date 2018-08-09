//!
//! Serialize a Rust data structure into a `JsValue`
//!

use cast;
use errors::Error;
use errors::ErrorKind;
use errors::Result as LibResult;
use neon::prelude::*;
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
pub fn to_value<'j, V>(cx: &mut FunctionContext<'j>, value: &V) -> LibResult<Handle<'j, JsValue>>
where
    V: Serialize + ?Sized,
{
    let serializer = Serializer {
        cx,
        ph: PhantomData,
    };
    let serialized_value = value.serialize(serializer)?;
    Ok(serialized_value)
}

#[doc(hidden)]
pub struct Serializer<'a, 'j: 'a>
{
    cx: &'a mut FunctionContext<'j>,
    ph: PhantomData<&'j ()>,
}

#[doc(hidden)]
pub struct ArraySerializer<'a, 'j: 'a>
{
    cx: &'a mut FunctionContext<'j>,
    array: Handle<'j, JsArray>,
}

#[doc(hidden)]
pub struct TupleVariantSerializer<'a, 'j: 'a>
{
    outter_object: Handle<'j, JsObject>,
    inner: ArraySerializer<'a, 'j>,
}

#[doc(hidden)]
pub struct MapSerializer<'a, 'j: 'a>
{
    cx: &'a mut FunctionContext<'j>,
    object: Handle<'j, JsObject>,
    key_holder: Handle<'j, JsObject>,
}

#[doc(hidden)]
pub struct StructSerializer<'a, 'j: 'a>
{
    cx: &'a mut FunctionContext<'j>,
    object: Handle<'j, JsObject>,
}

#[doc(hidden)]
pub struct StructVariantSerializer<'a, 'j: 'a>
{
    outer_object: Handle<'j, JsObject>,
    inner: StructSerializer<'a, 'j>,
}

#[doc(hidden)]
impl<'a, 'j> ser::Serializer for Serializer<'a, 'j>
{
    type Ok = Handle<'j, JsValue>;
    type Error = Error;

    type SerializeSeq = ArraySerializer<'a, 'j>;
    type SerializeTuple = ArraySerializer<'a, 'j>;
    type SerializeTupleStruct = ArraySerializer<'a, 'j>;
    type SerializeTupleVariant = TupleVariantSerializer<'a, 'j>;
    type SerializeMap = MapSerializer<'a, 'j>;
    type SerializeStruct = StructSerializer<'a, 'j>;
    type SerializeStructVariant = StructVariantSerializer<'a, 'j>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(JsBoolean::new(self.cx, v).upcast())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(JsNumber::new(self.cx, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(JsNumber::new(self.cx, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(JsNumber::new(self.cx, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(JsNumber::new(self.cx, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(JsNumber::new(self.cx, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(JsNumber::new(self.cx, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(JsNumber::new(self.cx, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(JsNumber::new(self.cx, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(JsNumber::new(self.cx, cast::f64(v)).upcast())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(JsNumber::new(self.cx, v).upcast())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut b = [0; 4];
        let result = v.encode_utf8(&mut b);
        let js_str = JsString::try_new(self.cx, result)
            .map_err(|_| ErrorKind::StringTooLongForChar(4))?;
        Ok(js_str.upcast())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let len = v.len();
        let js_str = JsString::try_new(self.cx, v).map_err(|_| ErrorKind::StringTooLong(len))?;
        Ok(js_str.upcast())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut buff = JsBuffer::new(self.cx, cast::u32(v.len())?)?;
        self.cx.borrow_mut(&mut buff, |buff| buff.as_mut_slice().clone_from_slice(v));
        Ok(buff.upcast())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsNull::new().upcast())
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
        Ok(JsNull::new().upcast())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(JsNull::new().upcast())
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
        let obj = JsObject::new(&mut *self.cx);
        let value_js = to_value(self.cx, value)?;
        obj.set(self.cx, variant, value_js)?;

        Ok(obj.upcast())
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(ArraySerializer::new(self.cx))
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(ArraySerializer::new(self.cx))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(ArraySerializer::new(self.cx))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        TupleVariantSerializer::new(self.cx, variant)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self.cx))
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer::new(self.cx))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        StructVariantSerializer::new(self.cx, variant)
    }
}

#[doc(hidden)]
impl<'a, 'j> ArraySerializer<'a, 'j>
{
    #[inline]
    fn new(cx: &'a mut FunctionContext<'j>) -> Self {
        let array = JsArray::new(cx, 0);
        ArraySerializer { cx, array }
    }
}

#[doc(hidden)]
impl<'a, 'j> ser::SerializeSeq for ArraySerializer<'a, 'j>
{
    type Ok = Handle<'j, JsValue>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = to_value(self.cx, value)?;

        let arr: Handle<'j, JsArray> = self.array;
        let len = arr.len();
        arr.set(self.cx, len, value)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array.upcast())
    }
}

impl<'a, 'j> ser::SerializeTuple for ArraySerializer<'a, 'j>
{
    type Ok = Handle<'j, JsValue>;
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
impl<'a, 'j> ser::SerializeTupleStruct for ArraySerializer<'a, 'j>
{
    type Ok = Handle<'j, JsValue>;
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
impl<'a, 'j> TupleVariantSerializer<'a, 'j>
{
    fn new(cx: &'a mut FunctionContext<'j>, key: &'static str) -> LibResult<Self> {
        let inner_array = JsArray::new(cx, 0);
        let outter_object = JsObject::new(cx);
        outter_object.set(cx, key, inner_array)?;
        Ok(TupleVariantSerializer {
            outter_object,
            inner: ArraySerializer {
                cx,
                array: inner_array,
            },
        })
    }
}

#[doc(hidden)]
impl<'a, 'j> ser::SerializeTupleVariant for TupleVariantSerializer<'a, 'j>
{
    type Ok = Handle<'j, JsValue>;
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
impl<'a, 'j> MapSerializer<'a, 'j>
{
    fn new(cx: &'a mut FunctionContext<'j>) -> Self {
        let object = JsObject::new(cx);
        let key_holder = JsObject::new(cx);
        MapSerializer {
            cx,
            object,
            key_holder,
        }
    }
}

#[doc(hidden)]
impl<'a, 'j> ser::SerializeMap for MapSerializer<'a, 'j>
{
    type Ok = Handle<'j, JsValue>;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = to_value(self.cx, key)?;
        self.key_holder.set(self.cx, "key", key)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key: Handle<'j, JsValue> = self.key_holder.get(&mut *self.cx, "key")?;
        let value_obj = to_value(self.cx, value)?;
        self.object.set(self.cx, key, value_obj)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.object.upcast())
    }
}

#[doc(hidden)]
impl<'a, 'j> StructSerializer<'a, 'j>
{
    #[inline]
    fn new(cx: &'a mut FunctionContext<'j>) -> Self {
        let object = JsObject::new(cx);
        StructSerializer { cx, object }
    }
}

#[doc(hidden)]
impl<'a, 'j> ser::SerializeStruct for StructSerializer<'a, 'j>
{
    type Ok = Handle<'j, JsValue>;
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
        let value = to_value(self.cx, value)?;
        self.object.set(self.cx, key, value)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.object.upcast())
    }
}

#[doc(hidden)]
impl<'a, 'j> StructVariantSerializer<'a, 'j>
{
    fn new(cx: &'a mut FunctionContext<'j>, key: &'static str) -> LibResult<Self> {
        let inner_object = JsObject::new(cx);
        let outter_object = JsObject::new(cx);
        outter_object.set(cx, key, inner_object)?;
        Ok(StructVariantSerializer {
            outer_object: outter_object,
            inner: StructSerializer {
                cx,
                object: inner_object,
            },
        })
    }
}

#[doc(hidden)]
impl<'a, 'j> ser::SerializeStructVariant for StructVariantSerializer<'a, 'j>
{
    type Ok = Handle<'j, JsValue>;
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
