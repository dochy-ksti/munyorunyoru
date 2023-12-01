use serde::de::SeqAccess;

use crate::{
    builder::default_builder::DefaultBuilder,
    error::{deserialize_error::DeserializeError, parse_fail::ParseFail},
    lang::builder_tree::TreeItem,
    MunyoDeserializer,
};

use super::enum_deserializer::EnumDeserializer;

pub(crate) struct VecAccess<'a, 'de: 'a> {
    de: &'a MunyoDeserializer<'de>,
    b: Vec<TreeItem<DefaultBuilder>>,
}

impl<'a, 'de> VecAccess<'a, 'de> {
    pub(crate) fn new(
        de: &'a MunyoDeserializer<'de>,
        mut b: Vec<TreeItem<DefaultBuilder>>,
    ) -> Self {
        b.reverse();
        Self { de, b }
    }
}

impl<'de, 'a> SeqAccess<'de> for VecAccess<'a, 'de> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.b.is_empty() {
            return Ok(None);
        }
        let item = self.b.pop().unwrap();
        let start_index = item.start_index;
        let mut d = EnumDeserializer::new(self.de, item);
        let r = seed
            .deserialize(&mut d)
            .map(Some)
            .map_err(|e| map_err(e, start_index));

        d.end().map_err(|e| map_err(e, start_index))?;
        r
    }
}

fn map_err(e: DeserializeError, start_index: usize) -> DeserializeError {
    match e {
        DeserializeError::Fail(e) => DeserializeError::Fail(e),
        DeserializeError::Msg(e) => DeserializeError::Fail(ParseFail::new(start_index, e)),
    }
}
