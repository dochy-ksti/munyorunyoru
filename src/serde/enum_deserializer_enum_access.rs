use serde::de::{EnumAccess, VariantAccess};

use crate::error::deserialize_error::DeserializeError;

use super::enum_deserializer::EnumDeserializer;

impl<'a, 'b, 'de> EnumAccess<'de> for &'b mut EnumDeserializer<'a, 'de> {
    type Error = DeserializeError;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let hoge = seed.deserialize(&mut *self)?;
        Ok((hoge, self))
    }
}

impl<'a, 'b, 'de> VariantAccess<'de> for &'b mut EnumDeserializer<'a, 'de> {
    type Error = DeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(&mut self.de)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeserializeError::msg("struct variant is not supported"))
    }
}
