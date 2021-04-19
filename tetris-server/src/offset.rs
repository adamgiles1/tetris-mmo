use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}
