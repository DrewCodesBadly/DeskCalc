use crate::calculator::{num_types::NumType, CalculatorError};

pub enum SymbolType<'a> {
    Variable(&'a NumType),
    DefaultFn(fn(Vec<NumType>) -> Result<NumType, CalculatorError>),
    UserFn,
}
