use std::str::FromStr;

use serde::Deserializer;

use crate::{MunyoDeserializer, lang::builder_tree::TreeItem, builder::default_builder::DefaultBuilder, error::parse_fail::ParseFail};

use super::{arguments::Arguments, item_deserializer::ItemDeserializer};

pub(crate) struct EnumDeserializer<'a, 'de: 'a> {
    pub(crate) de : ItemDeserializer<'a, 'de>
}

impl<'a, 'de> EnumDeserializer<'a, 'de> {
    pub(crate) fn new(de: &'a MunyoDeserializer<'de>, b: TreeItem<DefaultBuilder>) -> Self {
		let de = ItemDeserializer::new(de, b);
        Self { de }
    }

	pub(crate) fn err(&self, msg : &str) -> ParseFail{
		self.de.err(msg)
	}

	fn mes(&self) -> ParseFail{
		self.err("Vec's item must be enum")
	}
	
}

impl<'a, 'b, 'de> Deserializer<'de> for &'b mut EnumDeserializer<'a, 'de>{
    type Error = ParseFail;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> 
	{
		Err(self.mes())
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(self.err("deserializing &str is not supported"))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.err("deserializing byte arrays is not supported"))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.err("deserializing byte buf is not supported"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.err("deserializing unit is not supported"))
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.err("deserializing unit struct is not supported"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.err("deserializing tuple struct is not supported"))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.err("deserializing Tuple is not supported"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.err("deserializing Tuple Struct is not supported"))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.err("deserializing Map is not supported"))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.mes())
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			visitor.visit_string(self.de.b.item.typename.clone())
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			Err(self.err("deserializing ignored any is not supported"))
    }
}