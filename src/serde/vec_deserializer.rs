use serde::de::{EnumAccess, SeqAccess};

use crate::{
    builder::default_builder::DefaultBuilder,
    error::{parse_error::ParseError, parse_fail::ParseFail, Error},
    lang::builder_tree::TreeItem,
    MunyoDeserializer,
};

use super::{enum_deserializer::EnumDeserializer, item_deserializer::ItemDeserializer};

pub(crate) struct VecDeserializer<'a, 'de: 'a> {
    de: &'a MunyoDeserializer<'de>,
    b: Vec<TreeItem<DefaultBuilder>>,
}

impl<'a, 'de> VecDeserializer<'a, 'de> {
    pub(crate) fn new(de: &'a MunyoDeserializer<'de>, b: Vec<TreeItem<DefaultBuilder>>) -> Self {
        Self { de, b }
    }
}

impl<'de, 'a> SeqAccess<'de> for VecDeserializer<'a, 'de> {
    type Error = ParseFail;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.b.is_empty() {
            return Ok(None);
        }
        let item = self.b.pop().unwrap();
        let mut d = EnumDeserializer::new(self.de, item);
        seed.deserialize(&mut d).map(|a| Some(a))
    }
}
