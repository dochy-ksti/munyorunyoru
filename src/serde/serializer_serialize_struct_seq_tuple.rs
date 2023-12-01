use serde::ser;

use crate::MunyoSerializer;

use super::serializer::{ResultHelper, ResultSHelper};

impl<'a> ser::SerializeStruct for &'a mut MunyoSerializer {
    type Ok = ();

    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.state
            .add_param_key(key)
            .me(|| format!("param key failed {key}"))?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
        //self.state.end_param().me(|| "end param failed".to_string())
    }
}
impl<'a> ser::SerializeSeq for &'a mut MunyoSerializer {
    type Ok = ();

    type Error = crate::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        T::serialize(value, &mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.state
            .end_seq()
            .me(|| "unexpected end of seq".to_string())?;
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut MunyoSerializer {
    type Ok = ();

    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.state
            .end_line()
            .me(|| "unexpected end of tuple".to_string())?;
        Ok(())
    }
}
impl<'a> ser::SerializeTuple for &'a mut MunyoSerializer {
    type Ok = ();

    type Error = crate::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)?;
		Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}