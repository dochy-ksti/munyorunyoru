use std::{collections::BTreeMap, str::FromStr};

use serde::{de::SeqAccess, Deserializer};

use crate::{error::deserialize_error::DeserializeError, MunyoDeserializer};

pub(crate) struct ParamDeserializer<'a, 'de: 'a> {
    //this is currently unused but logically this struct has it, I think.
    _de: &'a MunyoDeserializer<'de>,
    pub(crate) params: &'a BTreeMap<String, String>,
    pub(crate) fields: &'static [&'static str],
    pub(crate) field_counter: usize,
}

fn err(s: &str) -> DeserializeError {
    DeserializeError::Msg(anyhow::anyhow!("{s}"))
}

impl<'a, 'de> ParamDeserializer<'a, 'de> {
    pub(crate) fn new(
        _de: &'a MunyoDeserializer<'de>,
        params: &'a BTreeMap<String, String>,
        fields: &'static [&'static str],
    ) -> Self {
        Self {
            _de,
            params,
            fields,
            field_counter: 0,
        }
    }

    pub(crate) fn arg(&mut self) -> Result<String, DeserializeError> {
        if let Some(field) = self.fields.get(self.field_counter) {
            self.field_counter += 1;
            if let Some(arg) = self.params.get(*field) {
                Ok(arg.to_string())
            } else {
                Err(err(&format!("param {} is not found", *field)))
            }
        } else {
            Err(err("this struct has no more fields to deserialize"))
        }
    }

    fn parse<T: FromStr>(&mut self) -> Result3<T, T::Err> {
        match self.arg() {
            Err(e) => Result3::DeserializeError(e),
            Ok(arg) => match arg.parse() {
                Ok(r) => Result3::Ok(r),
                Err(e) => Result3::Err(arg, e),
            },
        }
    }

    fn has_item(&self) -> bool {
        if let Some(field) = self.fields.get(self.field_counter) {
            self.params.contains_key(*field)
        } else {
            false
        }
    }
}

enum Result3<T, E> {
    Ok(T),
    DeserializeError(DeserializeError),
    Err(String, E),
}

trait Result3Helper<T, U> {
    fn me(self, f: impl Fn(U) -> String) -> Result<T, DeserializeError>;
}

impl<T, U> Result3Helper<T, U> for Result3<T, U> {
    fn me(self, f: impl Fn(U) -> String) -> Result<T, DeserializeError> {
        match self {
            Self::Ok(r) => Ok(r),
            Self::DeserializeError(e) => Err(e),
            Self::Err(arg, e) => Err(err(&format!("{} '{}'", f(e), arg))),
        }
    }
}

impl<'a, 'b, 'de> Deserializer<'de> for &'b mut ParamDeserializer<'a, 'de> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserialize any is not supported"))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let s = self.arg()?;
        match s.as_str() {
            "t" => visitor.visit_bool(true),
            "f" => visitor.visit_bool(false),
            "true" => visitor.visit_bool(true),
            "false" => visitor.visit_bool(false),
            _ => Err(err(&format!("failed to parse bool: input '{s}'"))),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse::<i8>().me(|e| e.to_string())?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse::<i16>().me(|e| e.to_string())?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse::<i32>().me(|e| e.to_string())?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse::<i64>().me(|e| e.to_string())?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse::<u8>().me(|e| e.to_string())?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse::<u16>().me(|e| e.to_string())?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse::<u32>().me(|e| e.to_string())?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse::<u64>().me(|e| e.to_string())?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse::<f32>().me(|e| e.to_string())?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(self.parse::<f64>().me(|e| e.to_string())?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_char(self.parse::<char>().me(|e| e.to_string())?)
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing &str is not supported"))

        //serde default visitor doesn't accept visit_str to deserialize &str
        //visitor.visit_str(&self.args.arg())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.arg()?)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing byte arrays is not supported"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing byte buf is not supported"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.has_item() {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing unit is not supported"))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing unit struct is not supported"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing tuple struct is not supported"))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err(
            "deserializing vec in the parameter position is not supported",
        ))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err(
            "deserializing tuple is not supported in the parameter position",
        ))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing tuple struct is not supported"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing map is not supported"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err(
            "deserializing struct is not supported in the parameter position",
        ))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err(
            "deserializing enum in the parameter position is not supported",
        ))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err(
            "deserializing identifier in the parameter position is not supported",
        ))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing IgnoredAny is not supported"))
    }
}

impl<'a, 'b, 'de> SeqAccess<'de> for &'b mut ParamDeserializer<'a, 'de> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut **self).map(Some)
    }
}
