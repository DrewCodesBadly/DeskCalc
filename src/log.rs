use crate::calculator::num_types::NumType;
use std::collections::HashMap;
use symbol_type::SymbolType;
use symbol_type::SymbolType::*;

mod built_in;
pub mod symbol_type;

#[derive(Default)]
pub struct Log {
    pub commands: Vec<(String, String)>,
    vars: HashMap<String, NumType>,
    consts: HashMap<String, NumType>,
}

impl Log {
    pub fn new() -> Self {
        Log {
            consts: built_in::get_constants_hashmap(),
            ..Default::default()
        }
    }

    pub fn push_results(&mut self, input: &str, output: &str) {
        self.commands.push((input.to_owned(), output.to_owned()))
    }

    pub fn add_var(&mut self, name: String, val: &NumType) {
        self.vars.insert(name, val.to_owned());
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    pub fn clear_vars(&mut self) {
        self.vars.clear();
    }

    pub fn clear(&mut self) {
        self.vars.clear();
        self.commands.clear();
    }

    pub fn search_symbol(&self, symbol: &str) -> Option<SymbolType> {
        // Try every base of symbols
        // First try vars
        if let Some(s) = self.vars.get(symbol) {
            Some(Variable(s))
        } else if let Some(s) = self.consts.get(symbol) {
            // Then try constants
            Some(Variable(s))
        } else {
            None
        }
    }
}
