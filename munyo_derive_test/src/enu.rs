use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Enu {
    It1(usize, String),
}

#[test]
fn test() -> Result<(),anyhow::Error>{
	let mut vec = vec![Enu::It1(1, "a".to_string())];
	let s = munyo::to_string(&vec)?;
	munyo::from_str(&s, None)

}