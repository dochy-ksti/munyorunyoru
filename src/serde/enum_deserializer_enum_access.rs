use serde::{de::{EnumAccess, VariantAccess}, Deserializer};

use crate::{error::parse_fail::ParseFail, serde::item_deserializer::ItemDeserializer};

use super::enum_deserializer::EnumDeserializer;

impl<'a, 'b, 'de> EnumAccess<'de> for &'b mut EnumDeserializer<'a, 'de>{
    type Error = ParseFail;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de> {
        let hoge = seed.deserialize(&mut *self)?;
		Ok((hoge, self))
    }
}

impl<'a, 'b, 'de> VariantAccess<'de> for &'b mut EnumDeserializer<'a, 'de>{
    type Error = ParseFail;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de> 
	{
		seed.deserialize(&mut self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
			visitor.visit_seq(&mut self.de)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(self.err("struct variant is not supported"))
    }
}