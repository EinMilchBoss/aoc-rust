use std::collections::HashMap;

use crate::instruction::{RegisterId, Word};

#[derive(Debug, Clone, PartialEq)]
pub struct Registers(HashMap<RegisterId, Word>);

impl Registers {
    pub fn new() -> Self {
        Self(HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0)]))
    }
}
