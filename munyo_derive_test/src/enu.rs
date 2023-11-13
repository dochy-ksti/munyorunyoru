use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Enu {
    It1(usize, String),
}

#[test]
fn test() -> munyo::Result<()> {
    let mut vec = vec![Enu::It1(1, "a".to_string())];
    let s = munyo::to_string(&vec)?;
    let hoge: Vec<Enu> = munyo::from_str(&s)?;
    println!("{:?}", hoge);
    Ok(())
}
