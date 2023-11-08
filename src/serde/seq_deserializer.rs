use serde::de::SeqAccess;

use crate::{MunyoDeserializer, error::ReadFileError};

pub(crate) struct SeqDeserializer<'a, 'de:'a>{
	de : &'a mut MunyoDeserializer<'de>
}

impl<'a, 'de> SeqDeserializer<'a, 'de>{
	pub(crate) fn new(de : &'a mut MunyoDeserializer) -> Self{ Self{ de } }
}

impl<'de, 'a> SeqAccess<'de> for SeqDeserializer<'a, 'de>{
    type Error = ReadFileError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de> {
        todo!()
    }
}