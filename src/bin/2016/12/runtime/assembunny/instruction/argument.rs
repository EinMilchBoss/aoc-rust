use super::{register_id::RegisterId, Word};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Argument {
    Literal(Word),
    Reference(RegisterId),
}
