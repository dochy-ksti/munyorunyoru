use serde::ser;

use crate::{error::Error, MunyoSerializer};

impl<'a> ser::SerializeStruct for &'a mut MunyoSerializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let mut se = MunyoSerializer::new();
        value.serialize(&mut se)?;
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
