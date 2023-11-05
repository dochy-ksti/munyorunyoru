use serde::{Serialize, Deserialize};

use crate::error::ReadFileError;

#[derive(Serialize, Deserialize)]
pub(crate) struct SampleObj{
	num : usize
}

#[test]
fn des() -> Result<(), ReadFileError>{
	let o = SampleObj{ num : 10 };
	
}