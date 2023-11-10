use serde::ser;

use crate::MunyoSerializer;

use super::serializer::{ResultSHelper, ResultHelper};

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
        self.state.add_param_key(key).me(|| format!("param key failed {key}"))?;
        value.serialize(&mut **self)
        
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
        //self.state.end_param().me(|| "end param failed".to_string())
    }
}
