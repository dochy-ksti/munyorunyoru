use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Enu {
    It1(usize, String),
}
