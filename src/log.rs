use crate::calculator::num_types::NumType;
use crate::calculator::CalculatorError;
use std::collections::HashMap;
use symbol_type::SymbolType;
use symbol_type::SymbolType::*;

mod built_in;
pub mod symbol_type;

type NumFn = fn(Vec<NumType>) -> Result<NumType, CalculatorError>;

pub struct Log {
    pub history: Vec<(String, String)>,
    pub last_number: NumType,
    vars: HashMap<String, NumType>,
    consts: HashMap<String, NumType>,
    default_functions: HashMap<String, NumFn>,
    commands: HashMap<String, fn(&mut Log) -> String>,
}

impl Default for Log {
    fn default() -> Self {
        Log {
            last_number: NumType::Scalar(1.0),
            history: Vec::<(String, String)>::default(),
            vars: HashMap::<String, NumType>::default(),
            consts: HashMap::<String, NumType>::default(),
            default_functions: HashMap::<String, NumFn>::default(),
            commands: HashMap::<String, fn(&mut Log) -> String>::default(),
        }
    }
}

impl Log {
    pub fn new() -> Self {
        Log {
            consts: built_in::get_constants_hashmap(),
            default_functions: built_in::get_default_functions_hashmap(),
            commands: built_in::get_default_commands_hashmap(),
            ..Default::default()
        }
    }

    pub fn push_results(&mut self, input: &str, output: &str) {
        self.history.push((input.to_owned(), output.to_owned()))
    }

    pub fn add_var(&mut self, name: String, val: &NumType) {
        self.vars.insert(name, val.to_owned());
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn clear_vars(&mut self) {
        self.vars.clear();
    }

    pub fn clear(&mut self) {
        self.vars.clear();
        self.history.clear();
    }

    #[allow(clippy::manual_map)]
    pub fn search_symbol(&self, symbol: &str) -> Option<SymbolType> {
        // Try every base of symbols
        // First try consts
        if let Some(s) = self.consts.get(symbol) {
            Some(Variable(s))
        } else if let Some(s) = self.vars.get(symbol) {
            // Then try vars
            Some(Variable(s))
        } else if let Some(f) = self.default_functions.get(symbol) {
            // Then try built in functions
            Some(DefaultFn(*f))
        } else {
            // Otherwise there is no such symbol
            None
        }
    }

    pub fn search_command(&self, name: &str) -> Option<fn(&mut Log) -> String> {
        self.commands.get(name).copied()
    }
}
