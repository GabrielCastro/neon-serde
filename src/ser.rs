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
pub fn to_value<'value, 'shandle, 'scope: 'shandle, V: Serialize + ?Sized, S: Scope<'scope>>(
    value: &'value V,
    scope: &'shandle mut S,
) -> LibResult<Handle<'shandle, js::JsValue>> {
    let serializer = Serializer {
        scope,
        ph: PhantomData,
    };
    let serialized_value = value.serialize(serializer)?;
    Ok(serialized_value)
}

#[doc(hidden)]
pub struct Serializer<'a, 'b: 'a, S: 'a>
where
    S: Scope<'b>,
{
    scope: &'a mut S,
    ph: PhantomData<&'b ()>,
}

#[doc(hidden)]
pub struct ArraySerializer<'a, 'b: 'a, S: 'a>
where
    S: Scope<'b>,
{
    scope: &'a mut S,
    array: Handle<'b, js::JsArray>,
}

#[doc(hidden)]
pub struct TupleVariantSerializer<'a, 'b: 'a, S: 'a>
where
    S: Scope<'b>,
{
    outter_object: Handle<'b, js::JsObject>,
    inner: ArraySerializer<'a, 'b, S>,
}

#[doc(hidden)]
pub struct MapSerializer<'a, 'b: 'a, S: 'a>
where
    S: Scope<'b>,
{
    scope: &'a mut S,
    object: Handle<'b, js::JsObject>,
    key_holder: Handle<'b, js::JsObject>,
}

#[doc(hidden)]
pub struct StructSerializer<'a, 'b: 'a, S: 'a>
where
    S: Scope<'b>,
{
    scope: &'a mut S,
    object: Handle<'b, js::JsObject>,
}

#[doc(hidden)]
pub struct StructVariantSerializer<'a, 'b: 'a, S: 'a>
where
    S: Scope<'b>,
{
    outter_object: Handle<'b, js::JsObject>,
    inner: StructSerializer<'a, 'b, S>,
}

#[doc(hidden)]
impl<'a, 'b, S> ser::Serializer for Serializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

    type SerializeSeq = ArraySerializer<'a, 'b, S>;
    type SerializeTuple = ArraySerializer<'a, 'b, S>;
    type SerializeTupleStruct = ArraySerializer<'a, 'b, S>;
    type SerializeTupleVariant = TupleVariantSerializer<'a, 'b, S>;
    type SerializeMap = MapSerializer<'a, 'b, S>;
    type SerializeStruct = StructSerializer<'a, 'b, S>;
    type SerializeStructVariant = StructVariantSerializer<'a, 'b, S>;


    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsBoolean::new(self.scope, v).upcast())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, cast::f64(v)).upcast())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.scope, v).upcast())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut b = [0; 4];
        let result = v.encode_utf8(&mut b);
        let js_str = js::JsString::new(self.scope, result).ok_or_else(|| {
            ErrorKind::StringTooLongForChar(4)
        })?;
        Ok(js_str.upcast())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let len = v.len();
        let js_str = js::JsString::new(self.scope, v).ok_or_else(|| {
            ErrorKind::StringTooLong(len)
        })?;
        Ok(js_str.upcast())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut buff = js::binary::JsBuffer::new(self.scope, cast::u32(v.len())?)?;
        buff.grab(|mut buff| buff.as_mut_slice().clone_from_slice(v));
        Ok(buff.upcast())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNull::new().upcast())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNull::new().upcast())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNull::new().upcast())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

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
        let value_js = to_value(value, self.scope)?;
        obj.set(variant, value_js)?;

        Ok(obj.upcast())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(ArraySerializer::new(self.scope))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(ArraySerializer::new(self.scope))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(ArraySerializer::new(self.scope))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        TupleVariantSerializer::new(self.scope, variant)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self.scope))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer::new(self.scope))
    }

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
impl<'a, 'b: 'a, S> ArraySerializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    fn new(scope: &'a mut S) -> Self {
        let array = js::JsArray::new(scope, 0);
        ArraySerializer {
            scope,
            array,
        }
    }
}

#[doc(hidden)]
impl<'a, 'b: 'a, S> ser::SerializeSeq for ArraySerializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = to_value(value, self.scope)?;

        let arr: Handle<js::JsArray> = self.array;
        let len = arr.len();
        arr.set(len, value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array.upcast())
    }
}

impl<'a, 'b: 'a, S> ser::SerializeTuple for ArraySerializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

#[doc(hidden)]
impl<'a, 'b: 'a, S> ser::SerializeTupleStruct for ArraySerializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

#[doc(hidden)]
impl<'a, 'b, S> TupleVariantSerializer<'a, 'b, S>
where
    S: Scope<'b>,
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
impl<'a, 'b, S> ser::SerializeTupleVariant for TupleVariantSerializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        use serde::ser::SerializeSeq;
        self.inner.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.outter_object.upcast())
    }
}

#[doc(hidden)]
impl<'a, 'b, S> MapSerializer<'a, 'b, S>
where
    S: Scope<'b>,
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
impl<'a, 'b, S> ser::SerializeMap for MapSerializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = to_value(key, self.scope)?;
        self.key_holder.set("key", key)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key: Handle<js::JsValue> = self.key_holder.get(&mut *self.scope, "key")?;
        let value_obj = to_value(value, self.scope)?;
        self.object.set(key, value_obj)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.object.upcast())
    }
}

#[doc(hidden)]
impl<'a, 'b, S> StructSerializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    fn new(scope: &'a mut S) -> Self {
        let object = js::JsObject::new(scope);
        StructSerializer {
            scope,
            object,
        }
    }
}

#[doc(hidden)]
impl<'a, 'b, S> ser::SerializeStruct for StructSerializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = to_value(value, self.scope)?;
        self.object.set(key, value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.object.upcast())
    }
}

#[doc(hidden)]
impl<'a, 'b, S> StructVariantSerializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    fn new(scope: &'a mut S, key: &'static str) -> LibResult<Self> {
        let inner_object = js::JsObject::new(scope);
        let outter_object = js::JsObject::new(scope);
        outter_object.set(key, inner_object)?;
        Ok(StructVariantSerializer {
            outter_object,
            inner: StructSerializer {
                scope,
                object: inner_object,
            },
        })
    }
}

#[doc(hidden)]
impl<'a, 'b, S> ser::SerializeStructVariant for StructVariantSerializer<'a, 'b, S>
where
    S: Scope<'b>,
{
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

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

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.outter_object.upcast())
    }
}
