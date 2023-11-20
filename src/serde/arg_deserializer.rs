use std::str::FromStr;

use serde::Deserializer;

use crate::{
    builder::default_builder::DefaultBuilder,
    error::{deserialize_error::DeserializeError, parse_fail::ParseFail},
    lang::builder_tree::TreeItem,
    MunyoDeserializer,
};

use super::{arguments::Arguments, param_deserializer::ParamDeserializer, vec_access::VecAccess};

pub(crate) struct ArgDeserializer<'a, 'de: 'a> {
    pub(crate) de: &'a MunyoDeserializer<'de>,
    pub(crate) b: TreeItem<DefaultBuilder>,
    args: Arguments,
    children_deserialized: bool,
}

impl<'a, 'de> ArgDeserializer<'a, 'de> {
    pub(crate) fn new(de: &'a MunyoDeserializer<'de>, b: TreeItem<DefaultBuilder>) -> Self {
        let args = Arguments::new(&b.item.content);
        Self {
            de,
            b,
            args,
            children_deserialized: false,
        }
    }

    fn parse<T: FromStr>(&mut self) -> Result<T, T::Err> {
        self.args.arg().parse()
    }
}

trait ResultHelper<T, U> {
    fn me(self, de: &ArgDeserializer, f: impl Fn(U) -> String) -> Result<T, DeserializeError>;
}

impl<T, U> ResultHelper<T, U> for Result<T, U> {
    fn me(self, de: &ArgDeserializer, f: impl Fn(U) -> String) -> Result<T, DeserializeError> {
        self.map_err(|e| err(&f(e)))
    }
}

fn err(msg: &str) -> DeserializeError {
    DeserializeError::msg(msg)
}

impl<'a, 'b, 'de> Deserializer<'de> for &'b mut ArgDeserializer<'a, 'de> {
    type Error = DeserializeError;

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
        let s = self.args.arg();
        match s.as_str() {
            "t" => visitor.visit_bool(true),
            "f" => visitor.visit_bool(false),
            "true" => visitor.visit_bool(true),
            "false" => visitor.visit_bool(false),
            _ => Err(err("failed to parse bool")),
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
        Err(err("deserializing &str is not supported"))

        //serde default visitor doesn't accept visit_str to deserialize &str
        //visitor.visit_str(&self.args.arg())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.args.arg())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing byte arrays is not supported"))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing byte buf is not supported"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err(
            "deserializing Option is not supported in argument position",
        ))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing Unit is not supported"))
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing Unit Struct is not supported"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing Tuple Struct is not supported"))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.children_deserialized {
            Err(err("only one children can be deserialized"))
        } else {
            self.children_deserialized = true;
            let children = std::mem::replace(&mut self.b.children, Vec::new());
            visitor.visit_seq(VecAccess::new(self.de, children))
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(&mut *self)
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
        Err(err("deserializing tuple struct is not supported"))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(err("deserializing map is not supported"))
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
        if self.args.is_empty() == false {
            let rest = self.args.rest();
            return Err(err(&format!(
                "All args must be used. remaining args \"{}\"",
                rest
            )));
        }
        let mut p =
            ParamDeserializer::new(self.de, &self.b.item.params, self.b.start_index, fields);
        visitor.visit_seq(&mut p)
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
        Err(err(
            "deserializing enum is not supported in the argument position",
        ))
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
        // hidden function.
        visitor.visit_string(self.args.rest())
    }
}
