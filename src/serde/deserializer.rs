use std::path::{Path, PathBuf};

use serde::Deserializer;

use crate::{
    builder::default_builder::{DefaultBuilder, DefaultMetaBuilder},
    error::{deserialize_error::DeserializeError, munyo_error::PathItem},
    lang::{
        builder_tree::{BuilderTree, TreeItem},
        from_str_with_metabuilder::{into_parse_error, parse_text},
    },
    Error,
};

use super::vec_access::VecAccess;

pub struct MunyoDeserializer<'de> {
    pub(crate) path: PathItem,
    pub(crate) text: &'de str,
    tree: BuilderTree<DefaultBuilder>,
}

impl<'de> MunyoDeserializer<'de> {
    pub fn new(text: &'de str) -> Result<Self, Error> {
        Self::inner_new(text, None)
    }
    pub fn with_path<P: AsRef<Path>>(text: &'de str, path: P) -> Result<Self, Error> {
        Self::inner_new(text, Some(path.as_ref().to_path_buf()))
    }

    fn inner_new(text: &'de str, path: Option<PathBuf>) -> Result<Self, Error> {
        let path = PathItem::new(path);
        let tree = parse_text(text, &DefaultMetaBuilder)
            .map_err(|e| Self::into_error(text, DeserializeError::Fail(e), &path))?;

        Ok(Self { text, tree, path })
    }

    pub(crate) fn into_munyo_error(&self, e: DeserializeError) -> Error {
        Self::into_error(self.text, e, &self.path)
    }

    fn into_error(text: &str, e: DeserializeError, path_item: &PathItem) -> Error {
        match e {
            DeserializeError::Fail(e) => {
                let e = into_parse_error(e, text);
                Error::Deserialize(path_item.clone(), e)
            }
            DeserializeError::Msg(e) => Error::Message(e),
        }
    }
}

fn mes() -> Error {
    Error::Message(anyhow::anyhow!("Munyo can only deserialize Vec<enum>"))
}

impl<'de, 'a> Deserializer<'de> for &'a mut MunyoDeserializer<'de> {
    type Error = crate::Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let vec: Vec<TreeItem<DefaultBuilder>> = std::mem::replace(&mut self.tree.root, vec![]);
        match visitor.visit_seq(VecAccess::new(&*self, vec)) {
            Ok(r) => Ok(r),
            Err(e) => Err(self.into_munyo_error(e)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
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
        Err(mes())
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
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
        Err(mes())
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
        Err(mes())
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(mes())
    }
}
