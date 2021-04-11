use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}
