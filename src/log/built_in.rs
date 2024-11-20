use crate::calculator::num_types::NumType;
use crate::calculator::num_types::NumType::*;
use crate::calculator::CalculatorError;
use std::collections::HashMap;
use std::f64::consts::{E, PI, TAU};

use super::Log;

pub fn get_constants_hashmap() -> HashMap<String, NumType> {
    let mut c: HashMap<String, NumType> = HashMap::new();
    c.insert(String::from("pi"), Scalar(PI));
    c.insert(String::from("e"), Scalar(E));
    c.insert(String::from("tau"), Scalar(TAU));

    c
}

pub fn get_default_functions_hashmap(
) -> HashMap<String, fn(Vec<NumType>) -> Result<NumType, CalculatorError>> {
    let mut f: HashMap<String, fn(Vec<NumType>) -> Result<NumType, CalculatorError>> =
        HashMap::new();

    /*
    Bunch of function additions here
    They are all if let statements which attempt to get the needed parameters from the input vector
    Extra parameters are simply ignored (could maybe be used for overloads?)
    The actual function happens in the body of the if let statement
    Else, it will return a missing parameters error
    */
    f.insert(String::from("sin"), |v| {
        if let Some(Scalar(n)) = v.get(0) {
            Ok(Scalar(n.sin()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "sin",
            )))
        }
    });
    f.insert(String::from("cos"), |v| {
        if let Some(Scalar(n)) = v.get(0) {
            Ok(Scalar(n.cos()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "cos",
            )))
        }
    });
    f.insert(String::from("tan"), |v| {
        if let Some(Scalar(n)) = v.get(0) {
            Ok(Scalar(n.tan()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "tan",
            )))
        }
    });
    f.insert(String::from("asin"), |v| {
        if let Some(Scalar(n)) = v.get(0) {
            Ok(Scalar(n.asin()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "asin",
            )))
        }
    });
    f.insert(String::from("acos"), |v| {
        if let Some(Scalar(n)) = v.get(0) {
            Ok(Scalar(n.acos()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "acos",
            )))
        }
    });
    f.insert(String::from("atan"), |v| {
        if let Some(Scalar(n)) = v.get(0) {
            Ok(Scalar(n.atan()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "atan",
            )))
        }
    });
    f.insert(String::from("abs"), |v| {
        if let Some(n) = v.get(0) {
            match n {
                Scalar(s) => Ok(Scalar(s.abs())),
            }
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "abs",
            )))
        }
    });
    f.insert(String::from("round"), |v| {
        if let Some(n) = v.get(0) {
            match n {
                Scalar(s) => Ok(Scalar(s.round())),
            }
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "round",
            )))
        }
    });
    f.insert(String::from("ceil"), |v| {
        if let Some(n) = v.get(0) {
            match n {
                Scalar(s) => Ok(Scalar(s.ceil())),
            }
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "ceil",
            )))
        }
    });
    f.insert(String::from("floor"), |v| {
        if let Some(n) = v.get(0) {
            match n {
                Scalar(s) => Ok(Scalar(s.floor())),
            }
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "floor",
            )))
        }
    });
    f.insert(String::from("log"), |v| {
        if let Some([Scalar(s), Scalar(b)]) = v.get(0..=1) {
            Ok(Scalar(s.log(*b)))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "log",
            )))
        }
    });
    f.insert(String::from("ln"), |v| {
        if let Some(Scalar(s)) = v.get(0) {
            Ok(Scalar(s.ln()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "ln",
            )))
        }
    });
    f.insert(String::from("sqrt"), |v| {
        if let Some(Scalar(s)) = v.get(0) {
            Ok(Scalar(s.sqrt()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "sqrt",
            )))
        }
    });

    f
}

pub fn get_default_commands_hashmap() -> HashMap<String, fn(&mut Log) -> String> {
    let mut c: HashMap<String, fn(&mut Log) -> String> = HashMap::new();
    c.insert(String::from("clear"), |l| {
        l.clear();
        String::from("All data cleared")
    });
    c.insert(String::from("clearvars"), |l| {
        l.clear_vars();
        String::from("Variable data cleared")
    });
    c.insert(String::from("clearhistory"), |l| {
        l.clear_history();
        String::from("Calculator history cleared")
    });

    c
}
