use serde::{Deserialize, Serialize};

use crate::error::ReadFileError;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum SampleEnum {
    Item1(usize, String),
}

pub(crate) struct SampleObj {
    num: usize,
}

#[test]
fn des() -> Result<(), ReadFileError> {
    let o = vec![SampleEnum::Item1(0, "a".to_string())];
    let mut ser = crate::MunyoSerializer::new();
    o.serialize(&mut ser)?;
	println!("{}", ser.output());
    let mut de = crate::MunyoDeserializer::new(ser.output(), None)?;
    let deserialized: Vec<SampleEnum> = Deserialize::deserialize(&mut de)?;

    assert_eq!(o, deserialized);
    Ok(())
}
