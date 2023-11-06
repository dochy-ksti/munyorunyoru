use serde::{Serialize, Deserialize};

use crate::error::ReadFileError;

#[derive(Serialize, Deserialize)]
pub(crate) struct SampleObj{
	num : usize
}

#[test]
fn des() -> Result<(), ReadFileError>{
	let o = SampleObj{ num : 10 };
	let mut ser = crate::MunyoSerializer::new();
	o.serialize(&mut ser)?;
	let de = crate::MunyoDeserializer::from_str(ser.output());
	let deserialized = SampleObj::deserialize(de)?;

	assert_eq!(o.num, deserialized.num);
	Ok(())
}