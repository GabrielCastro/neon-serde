
use serde::ser::{self, Serialize};
use errors::Error;
use errors::ErrorKind;
use errors::Result as LibResult;
use neon::js;
use neon::mem::Handle;
use neon::scope::Scope;
use neon::scope::RootScope;
use neon::js::Object;

pub fn to_value<'value, 'shandle, 'scope, V: Serialize + ?Sized>(
    value: &'value V,
    scope: &'shandle mut RootScope<'scope>
) -> LibResult<Handle<'shandle, js::JsValue>>
{
    let serializer = Serializer(scope);
    let serialized = value.serialize(serializer)?;
    Ok(serialized)
}

pub struct Serializer<'a, 'b: 'a>(&'a mut RootScope<'b>);


#[doc(hidden)]
pub struct ArraySerializer<'a, 'b: 'a>(&'a mut RootScope<'b>, Handle<'b, js::JsArray>);

#[doc(hidden)]
pub struct TupleSerializer<'a, S>(&'a mut S) where S: 'a + Scope<'a>;

#[doc(hidden)]
pub struct TupleStructSerializer<'a, S>(&'a mut S) where S: 'a + Scope<'a>;

#[doc(hidden)]
pub struct TupleVariantSerializer<'a, S>(&'a mut S) where S: 'a + Scope<'a>;

#[doc(hidden)]
pub struct MapSerializer<'a, S>(&'a mut S) where S: 'a + Scope<'a>;

#[doc(hidden)]
pub struct StructSerializer<'a, S>(&'a mut S) where S: 'a + Scope<'a>;

#[doc(hidden)]
pub struct StructVariantSerializer<'a, S>(&'a mut S) where S: 'a + Scope<'a>;


impl<'a, 'b: 'a> ser::Serializer for Serializer<'a, 'b>
{
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

    type SerializeSeq = ArraySerializer<'a, 'b>;
    type SerializeTuple = TupleSerializer<'a, RootScope<'a>>;
    type SerializeTupleStruct = TupleStructSerializer<'a, RootScope<'a>>;
    type SerializeTupleVariant = TupleVariantSerializer<'a, RootScope<'a>>;
    type SerializeMap = MapSerializer<'a, RootScope<'a>>;
    type SerializeStruct = StructSerializer<'a, RootScope<'a>>;
    type SerializeStructVariant = StructVariantSerializer<'a, RootScope<'a>>;


    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsBoolean::new(self.0, v).upcast())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.0, v as f64).upcast())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.0, v as f64).upcast())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.0, v as f64).upcast())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.0, v as f64).upcast())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.0, v as f64).upcast())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.0, v as f64).upcast())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.0, v as f64).upcast())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.0, v as f64).upcast())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.0, v as f64).upcast())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNumber::new(self.0, v as f64).upcast())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut b = [0; 2];
        let result = v.encode_utf8(&mut b);
        let js_str = js::JsString::new(self.0, result).ok_or_else(|| ErrorKind::StringTooLong(2))?;
        Ok(js_str.upcast())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let len = v.len();
        let js_str = js::JsString::new(self.0, v).ok_or_else(|| ErrorKind::StringTooLong(len))?;
        Ok(js_str.upcast())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsUndefined::new().upcast())
    }

    fn serialize_some<T: ? Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNull::new().upcast())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(js::JsNull::new().upcast())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ? Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ? Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(ArraySerializer::new(self.0))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}


impl<'a, 'b: 'a> ArraySerializer<'a, 'b> {
    fn new(scope: &'a mut RootScope<'b>) -> Self {
        let arr = js::JsArray::new(scope, 0);
        ArraySerializer(scope, arr)
    }
}

impl<'a, 'b: 'a> ser::SerializeSeq for ArraySerializer<'a, 'b> {
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

    fn serialize_element<T: ? Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        let value = to_value(value, self.0)?;

        let arr :Handle<js::JsArray> = self.1;
        let len = arr.len();
        arr.set(len, value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.1.upcast())
    }
}

impl<'a, S: 'a + Scope<'a>> ser::SerializeTuple for TupleSerializer<'a, S> {
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;

    fn serialize_element<T: ? Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, S: 'a + Scope<'a>> ser::SerializeTupleStruct for TupleStructSerializer<'a, S> {
    fn serialize_field<T: ? Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;
}

impl<'a, S: 'a + Scope<'a>> ser::SerializeTupleVariant for TupleVariantSerializer<'a, S> {
    fn serialize_field<T: ? Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;
}

impl<'a, S: 'a + Scope<'a>> ser::SerializeMap for MapSerializer<'a, S> {
    fn serialize_key<T: ? Sized>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ? Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;
}

impl<'a, S: 'a + Scope<'a>> ser::SerializeStruct for StructSerializer<'a, S> {
    fn serialize_field<T: ? Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;
}

impl<'a, S: 'a + Scope<'a>> ser::SerializeStructVariant for StructVariantSerializer<'a, S> {
    fn serialize_field<T: ? Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    type Ok = Handle<'a, js::JsValue>;
    type Error = Error;
}
