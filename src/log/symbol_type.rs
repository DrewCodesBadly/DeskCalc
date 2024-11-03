use crate::calculator::num_types::NumType;

pub enum SymbolType<'a> {
    Variable(&'a NumType),
    Function,
}
