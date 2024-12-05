use crate::calculator::num_types::NumType;
use crate::calculator::num_types::NumType::*;
use crate::calculator::CalculatorError;
use std::collections::HashMap;
use std::f64::consts::{E, PI, TAU};

use super::Log;

type NumFn = fn(Vec<NumType>) -> Result<NumType, CalculatorError>;

pub fn get_constants_hashmap() -> HashMap<String, NumType> {
    let mut c: HashMap<String, NumType> = HashMap::new();
    c.insert(String::from("pi"), Scalar(PI));
    c.insert(String::from("e"), Scalar(E));
    c.insert(String::from("tau"), Scalar(TAU));

    c
}

pub fn get_default_functions_hashmap() -> HashMap<String, NumFn> {
    let mut f: HashMap<String, NumFn> = HashMap::new();

    /*
    Bunch of function additions here
    They are all if let statements which attempt to get the needed parameters from the input vector
    Extra parameters are simply ignored (could maybe be used for overloads?)
    The actual function happens in the body of the if let statement
    Else, it will return a missing parameters error
    */
    f.insert(String::from("sin"), |v| {
        if let Some(Scalar(n)) = v.first() {
            Ok(Scalar(n.sin()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "sin",
            )))
        }
    });
    f.insert(String::from("cos"), |v| {
        if let Some(Scalar(n)) = v.first() {
            Ok(Scalar(n.cos()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "cos",
            )))
        }
    });
    f.insert(String::from("tan"), |v| {
        if let Some(Scalar(n)) = v.first() {
            Ok(Scalar(n.tan()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "tan",
            )))
        }
    });
    f.insert(String::from("asin"), |v| {
        if let Some(Scalar(n)) = v.first() {
            Ok(Scalar(n.asin()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "asin",
            )))
        }
    });
    f.insert(String::from("acos"), |v| {
        if let Some(Scalar(n)) = v.first() {
            Ok(Scalar(n.acos()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "acos",
            )))
        }
    });
    f.insert(String::from("atan"), |v| {
        if let Some(Scalar(n)) = v.first() {
            Ok(Scalar(n.atan()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "atan",
            )))
        }
    });
    f.insert(String::from("abs"), |v| {
        if let Some(n) = v.first() {
            match n {
                Scalar(s) => Ok(Scalar(s.abs())),
                Vector(v) => Ok(Vector(v.iter().map(|f| f.abs()).collect())),
            }
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "abs",
            )))
        }
    });
    f.insert(String::from("round"), |v| {
        if let Some(n) = v.first() {
            match n {
                Scalar(s) => Ok(Scalar(s.round())),
                Vector(v) => Ok(Vector(v.iter().map(|f| f.round()).collect())),
            }
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "round",
            )))
        }
    });
    f.insert(String::from("ceil"), |v| {
        if let Some(n) = v.first() {
            match n {
                Scalar(s) => Ok(Scalar(s.ceil())),
                Vector(v) => Ok(Vector(v.iter().map(|f| f.ceil()).collect())),
            }
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "ceil",
            )))
        }
    });
    f.insert(String::from("floor"), |v| {
        if let Some(n) = v.first() {
            match n {
                Scalar(s) => Ok(Scalar(s.floor())),
                Vector(v) => Ok(Vector(v.iter().map(|f| f.floor()).collect())),
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
        if let Some(Scalar(s)) = v.first() {
            Ok(Scalar(s.ln()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "ln",
            )))
        }
    });
    f.insert(String::from("sqrt"), |v| {
        if let Some(Scalar(s)) = v.first() {
            Ok(Scalar(s.sqrt()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "sqrt",
            )))
        }
    });
    f.insert(String::from("rad"), |v| {
        if let Some(Scalar(s)) = v.first() {
            Ok(Scalar(s.to_radians()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "rad",
            )))
        }
    });
    f.insert(String::from("deg"), |v| {
        if let Some(Scalar(s)) = v.first() {
            Ok(Scalar(s.to_degrees()))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "deg",
            )))
        }
    });

    // Vector functions start here
    f.insert(String::from("mag"), |v| {
        if let Some(Vector(v)) = v.first() {
            Ok(Scalar(
                v.iter().fold(0.0, |acc, n| acc + n.powf(2.0)).sqrt(),
            ))
        } else {
            Err(CalculatorError::MissingFunctionParameters(String::from(
                "mag",
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
