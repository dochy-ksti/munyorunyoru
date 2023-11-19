use std::{collections::BTreeMap, str::FromStr};

use serde::{de::SeqAccess, Deserializer};

use crate::{
    builder::default_builder::{DefaultBuilder, DefaultItem},
    error::parse_fail::ParseFail,
    lang::builder_tree::TreeItem,
    MunyoDeserializer,
};

pub(crate) struct ParamDeserializer<'a, 'de: 'a> {
    pub(crate) de: &'a MunyoDeserializer<'de>,
    pub(crate) params: &'a BTreeMap<String, String>,
    pub(crate) start_index: usize,
    pub(crate) fields: &'static [&'static str],
    pub(crate) field_counter: usize,
}

impl<'a, 'de> ParamDeserializer<'a, 'de> {
    pub(crate) fn new(
        de: &'a MunyoDeserializer<'de>,
        params: &'a BTreeMap<String, String>,
		start_index : usize,
        fields: &'static [&'static str],
    ) -> Self {
        Self {
            de,
            params: params,
            start_index,
            fields,
            field_counter: 0,
        }
    }

    pub(crate) fn err(&self, msg: &str) -> ParseFail {
        ParseFail::msg(self.start_index, msg.to_string())
    }

    pub(crate) fn arg(&mut self) -> Result<String, ParseFail> {
		if let Some(field) = self.fields.get(self.field_counter) {
            self.field_counter += 1;
            if let Some(arg) = self.params.get(*field) {
                Ok(arg.to_string())
            } else{
				Err(self.err(&format!("param {} is not found", *field)))
			}
        } else {
            Err(self.err("this struct has no more fields to deserialize"))
        }
    }

    fn parse<T: FromStr>(&mut self) -> Result3<T, T::Err> {
        match self.arg() {
            Err(e) => Result3::ParseFail(e),
            Ok(arg) => match arg.parse() {
                Ok(r) => Result3::Ok(r),
                Err(e) => Result3::Err(e),
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
    ParseFail(ParseFail),
    Err(E),
}

trait Result3Helper<T, U> {
    fn me(self, de: &ParamDeserializer, f: impl Fn(U) -> String) -> Result<T, ParseFail>;
}

impl<T, U> Result3Helper<T, U> for Result3<T, U> {
    fn me(self, de: &ParamDeserializer, f: impl Fn(U) -> String) -> Result<T, ParseFail> {
        match self {
            Self::Ok(r) => Ok(r),
            Self::ParseFail(e) => Err(e),
            Self::Err(e) => Err(de.err(&f(e))),
        }
    }
}

trait ResultHelper<T> {
    fn me(self, de: &ParamDeserializer) -> Result<T, ParseFail>;
}

impl<T> ResultHelper<T> for Result<T,ParseFail>{
    fn me(self, de: &ParamDeserializer) -> Result<T, ParseFail> {
        self.map_err(|e| de.err(&format!("{}", e.to_string())))
    }
}

impl<'a, 'b, 'de> Deserializer<'de> for &'b mut ParamDeserializer<'a, 'de> {
    type Error = ParseFail;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
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
            _ => Err(self.err("failed to parse bool")),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse::<i8>().me(self, |e| e.to_string())?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse::<i16>().me(self, |e| e.to_string())?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse::<i32>().me(self, |e| e.to_string())?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse::<i64>().me(self, |e| e.to_string())?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse::<u8>().me(self, |e| e.to_string())?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse::<u16>().me(self, |e| e.to_string())?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse::<u32>().me(self, |e| e.to_string())?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse::<u64>().me(self, |e| e.to_string())?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse::<f32>().me(self, |e| e.to_string())?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(self.parse::<f64>().me(self, |e| e.to_string())?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_char(self.parse::<char>().me(self, |e| e.to_string())?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing &str is not supported"))

        //serde default visitor doesn't accept visit_str to deserialize &str
        //visitor.visit_str(&self.args.arg())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.arg()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing byte arrays is not supported"))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing byte buf is not supported"))
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

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing Unit is not supported"))
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing Unit Struct is not supported"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing Tuple Struct is not supported"))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing Tuple is not supported"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing Tuple Struct is not supported"))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing Map is not supported"))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing Map is not supported"))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(self.err("deserializing IgnoredAny is not supported"))
    }
}

impl<'a, 'b, 'de> SeqAccess<'de> for &'b mut ParamDeserializer<'a, 'de> {
    type Error = ParseFail;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut **self).map(|r| Some(r))
    }
}
