use serde::ser;

use crate::Error;

use super::serialize_state::{Er, SerializeState};

pub struct MunyoSerializer {
    pub(crate) state: SerializeState,
}

impl MunyoSerializer {
    pub fn new() -> MunyoSerializer {
        MunyoSerializer {
            state: SerializeState::new(),
        }
    }

    pub fn output(&self) -> &str {
        &self.state.output
    }
}

pub(crate) trait ResultHelper {
    fn me<F: Fn() -> String>(self, f: F) -> Result<(), Error>;
}

impl ResultHelper for Result<(), ()> {
    fn me<F: Fn() -> String>(self, f: F) -> Result<(), Error> {
        self.map_err(|_| Error::Serialize(msg(f())))
    }
}

fn msg(s: String) -> anyhow::Error {
    anyhow::Error::msg(s)
}
pub(crate) trait ResultSHelper {
    fn me<F: Fn() -> String>(self, f: F) -> Result<(), Error>;
}

impl ResultSHelper for Result<(), Er> {
    fn me<F: Fn() -> String>(self, f: F) -> Result<(), Error> {
        self.map_err(|e| match e {
            Er::None => Error::Serialize(msg(f())),
            Er::Message(s) => Error::Serialize(msg(s)),
        })
    }
}

fn err(s: &str) -> Error {
    Error::Serialize(msg(s.to_string()))
}

impl<'a> serde::ser::Serializer for &'a mut MunyoSerializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected bool {v}"))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected i8 {v}"))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected i16 {v}"))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected i32 {v}"))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected i64 {v}"))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected u8 {v}"))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected u16 {v}"))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected u32 {v}"))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected u64 {v}"))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected f32 {v}"))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected f64 {v}"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.state
            .add_arg(&v.to_string())
            .me(|| format!("unexpected char {v}"))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.state.add_str(v).me(|| format!("unexpected str {v}"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(err("bytes are not supported"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.state.add_none().me(|| "add none failed".to_string())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(err("unit () is not supported"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(err("unit structs are not supported"))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.state
            .start_line(variant)
            .me(|| format!("unexpected start of line"))?;
        self.state
            .end_line()
            .me(|| format!("unexpected end of line"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.state
            .start_line(variant)
            .me(|| format!("unexpected enum_variant {name} {variant}"))?;
        value.serialize(&mut *self)?;
        self.state.end_line().me(|| format!("failed end_line"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.state
            .start_seq()
            .me(|| format!("unexpected Vec or Array of sorts"))?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(err("tuples are not supported"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(err("tuple structs are not supported"))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.state
            .start_line(variant)
            .me(|| format!("unexpected enum variant {name} {variant}"))?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeMap for &'a mut MunyoSerializer {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut MunyoSerializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeTuple for &'a mut MunyoSerializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut MunyoSerializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
