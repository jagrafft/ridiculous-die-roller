use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Rolled {
    pub timestamp: String,
    pub sides: i32,
    pub rolls: i32,
    pub results: Vec<i32>,
    pub total: i32
}
