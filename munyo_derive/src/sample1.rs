use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum Enu{
	It1(usize, String)
}