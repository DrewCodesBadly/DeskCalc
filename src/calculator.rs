mod buffers;
pub mod num_types;

use crate::log::symbol_type::SymbolType::*;
use crate::log::Log;
use buffers::Collapse;
use itertools::Itertools;
use num_types::NumType;
use std::{clone::Clone, fmt::Display, mem};

#[derive(Debug, Clone)]
pub enum CalculatorError {
    UnknownSymbol(String),
    ParseNumberErrror,
    MissingFunctionParameters(String),
    InvalidCommand(String),
    RecursiveVectors,
    ComponentAccessError,
    ComponentDNE,
}

impl Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculatorError::UnknownSymbol(s) => {
                write!(f, "Error: Could not find symbol \"{}\"", s)
            }
            CalculatorError::ParseNumberErrror => write!(f, "Error: Invalid number"),
            CalculatorError::MissingFunctionParameters(s) => {
                write!(
                    f,
                    "Error: Function \"{}\" has missing or invalid parameters",
                    s
                )
            }
            CalculatorError::InvalidCommand(s) => {
                write!(f, "Error: Command \"{}\" not recognized", s)
            }
            CalculatorError::RecursiveVectors => {
                write!(f, "Error: Vectors may not contain other vectors")
            }
            CalculatorError::ComponentAccessError => {
                write!(f, "Error: Cannot access components of a scalar value")
            }
            CalculatorError::ComponentDNE => {
                write!(f, "Error: Component does not exist")
            }
        }
    }
}

// Parses an input string to calculate the output
pub fn calculate(input: &str, log: &Log) -> String {
    // Removes any whitespace at all from the string and make it lowercase
    let mut expression = input.replace(' ', "").to_ascii_lowercase();

    // Return nothing if given nothing
    if expression.is_empty() {
        return String::from("...");
    }

    // check for and remove any variable assignment
    match expression
        .chars()
        .next()
        .expect("Previously checked if string was empty")
    {
        // Find index of '=' (signifies the end of the variable name) or throw an error
        '#' => {
            let var_end = match expression.find('=') {
                Some(i) => i,
                None => return String::from("Error: Missing \"=\" after variable name"),
            };

            // Remove var from expression so as to not confuse parse()
            expression.drain(..=var_end);
        }
        // Handles commands - we won't run them yet though
        '/' => {
            expression.remove(0);
            let name: String = mem::take(&mut expression);
            match log.search_command(&name) {
                Some(_) => return String::from("Enter to run command..."),
                None => return CalculatorError::InvalidCommand(name).to_string(),
            }
        }
        _ => {}
    }

    // Parse and return output
    match parse(expression.chars(), log) {
        Ok(n) => " = ".to_owned() + &n.to_string(),
        Err(e) => e.to_string(),
    }
}

// Like calculate but will actually try to assign the final value to a variable if one is provided
pub fn calculate_assign(input: &str, log: &mut Log) -> String {
    // Remove any whitespace at all from the string
    let mut expression = input.replace(' ', "").to_ascii_lowercase();

    // Return nothing if given nothing
    if expression.is_empty() {
        return String::from("...");
    }

    // Check if we need to assign to a variable
    let assigning_to: Option<String> = match expression
        .chars()
        .next()
        .expect("Previously checked if string was empty")
    {
        '#' => {
            // Find index of '=' (signifies the end of the variable name) or throw an error
            let var_end = match expression.find('=') {
                Some(i) => i - 1, // we remove '#' separately
                None => return String::from("Error: Missing \"=\" after variable name"),
            };

            // Remove var from expression so as to not confuse parse()
            expression.remove(0);
            let var_name: String = expression.drain(..var_end).collect();
            expression.remove(0); // remove '=' separately, not part of var name

            Some(var_name)
        }
        // Handles commands - now, we will run them
        '/' => {
            expression.remove(0);
            let name: String = mem::take(&mut expression);
            match log.search_command(&name) {
                Some(f) => return f(log),
                None => return CalculatorError::InvalidCommand(name).to_string(),
            }
        }
        _ => None,
    };

    let result = parse(expression.chars(), log);

    // tbh not sure this is "correct" but it works
    if let (Ok(n), Some(s)) = (&result, assigning_to) {
        log.add_var(s, n);
    }

    // Parse and return output
    match result {
        Ok(n) => {
            log.last_number = n.clone();
            " = ".to_owned() + &n.to_string()
        }
        Err(e) => e.to_string(),
    }
}

fn parse<T: Iterator<Item = char> + Clone>(
    mut input: T,
    log: &Log,
) -> Result<NumType, CalculatorError> {
    // Exponent buffer
    let mut e_buffer = buffers::ExponentBuffer::default();
    // Multiplication/division buffer:
    let mut m_buffer = buffers::MultiplicationBuffer::default();
    // Addition/subtraction buffer:
    let mut a_buffer = buffers::AdditionBuffer::default();

    // Tracks whether or not the last thing the parser found was a number or operator
    // Makes (-) work
    let mut previous_number: bool = false;

    // Loop through characters
    while let Some(c) = input.next() {
        match c {
            // Check for parantheses, meaning we need to recursively call parse()
            // Place the result in the multiplication buffer
            '(' => {
                e_buffer.push(parse(
                    input
                        .by_ref()
                        .take_while(|c| *c != ')')
                        .collect::<Vec<char>>()
                        .iter()
                        .cloned(),
                    log,
                )?);

                previous_number = true; // () expressions are treated as a single number after evaluation
            }

            // Check if we are making a vector type, put it into the exponent buffer
            '[' => {
                e_buffer.push(parse_to_vec(input.by_ref().take_while(|c| *c != ']'), log)?);
                previous_number = true;
            }

            // Check for '.' accessing a vector - if no vec, then assume this is a decimal w/o leading 0
            // jank
            '.' => {
                if previous_number {
                    if let Some(NumType::Vector(v)) = e_buffer.get_back().cloned() {
                        // Remove last element, replace it with the component we're accessing
                        e_buffer.remove_back();
                        // Determine index from characters following '.'
                        let index: usize = match input.next().unwrap_or('x') {
                            'x' => 0,
                            'y' => 1,
                            'z' => 2,
                            c => {
                                let mut s = String::new();
                                s.push(c);
                                input
                                    .take_while_ref(|c| c.is_ascii_digit())
                                    .for_each(|c| s.push(c));
                                s.parse()
                                    .map_err(|_| CalculatorError::ComponentAccessError)?
                            }
                        };
                        e_buffer.push(match v.get(index) {
                            Some(f) => NumType::Scalar(*f),
                            None => return Err(CalculatorError::ComponentDNE),
                        });
                    } else {
                        return Err(CalculatorError::ComponentAccessError);
                    }
                } else {
                    e_buffer.push(NumType::Scalar(parse_chars_to_f64(c, &mut input)?));
                    previous_number = true;
                }
            }

            // Check for a number, put it into the exponent buffer
            '0'..='9' => {
                e_buffer.push(NumType::Scalar(parse_chars_to_f64(c, &mut input)?));
                previous_number = true;
            }

            // Check for operators
            '*' => {
                m_buffer.push(e_buffer.collapse());

                previous_number = false;
            }
            // '/' only sets the multiplication buffer to take the reciprocal of the next number
            '/' => {
                m_buffer.push(e_buffer.collapse());
                m_buffer.dividing = true;

                previous_number = false;
            }
            // '+' collapses the multiplication buffer into the addition buffer
            '+' => {
                m_buffer.push(e_buffer.collapse());
                a_buffer.push(m_buffer.collapse());
                previous_number = false;
            }
            // '-' pushes -1 to the multiplication buffer and, if the last part was a number, collapses the multiplication buffer
            '-' => {
                // Only collapse the multiplication buffer if the last thing was a number
                // If it was an operator this is unary minus not subtraction
                if previous_number {
                    m_buffer.push(e_buffer.collapse());
                    a_buffer.push(m_buffer.collapse());
                    previous_number = false;
                }

                // Push -1 to the multiplication buffer to reverse the output
                m_buffer.push(NumType::Scalar(-1.0));
            }

            // Else, it must be some sort of symbol
            // Symbol names are made up of only ascii alphabetic chars
            // Numbers cannot be used in symbol names or it would not be possible to differentiate symbol then number from one symbol
            'A'..='Z' | 'a'..='z' => {
                let name: String = c.to_string()
                    + &(input
                        .take_while_ref(|c| c.is_ascii_alphabetic())
                        .collect::<String>());
                match log.search_symbol(&name) {
                    Some(Variable(n)) => e_buffer.push(n.clone()),
                    Some(DefaultFn(f)) => e_buffer.push(f(get_function_params(&mut input, log)?)?),
                    Some(UserFn) => {} // TODO: Implement user functions
                    None => return Err(CalculatorError::UnknownSymbol(name)),
                }

                previous_number = true;
            }

            // \ character takes the last answer
            '\\' => e_buffer.push(log.last_number.clone()),

            // otherwise ignore, calc has no clue what to do with this symbol
            _ => {}
        }
    }

    m_buffer.push(e_buffer.collapse());
    a_buffer.push(m_buffer.collapse());
    Ok(a_buffer.collapse())
}

// Turns the character iterator into a float or throws an error
fn parse_chars_to_f64<T: Iterator<Item = char> + Clone>(
    first: char,
    iter: &mut T,
) -> Result<f64, CalculatorError> {
    if let Ok(f) = (first.to_string()
        + &(iter
            .take_while_ref(|c| *c == '.' || c.is_ascii_digit())
            .collect::<String>()))
        .parse()
    {
        Ok(f)
    } else {
        Err(CalculatorError::ParseNumberErrror)
    }
}

// Parses a character iterator of numbers separated by commas to a NumType::Vector
// Vectors may contain vectors. This is a feature because it can be so why not
fn parse_to_vec<T: Iterator<Item = char>>(iter: T, log: &Log) -> Result<NumType, CalculatorError> {
    let mut v = Vec::<f64>::new();
    let mut num_string = String::new();
    for c in iter {
        if c == ',' {
            // Parse string to a number and push it into the vector, then clear
            if let Some(n) = parse(num_string.chars(), log)?.scalar_value() {
                v.push(n);
            } else {
                return Err(CalculatorError::RecursiveVectors);
            };
            num_string.clear()
        } else {
            // keep adding characters to the number
            num_string.push(c);
        }
    }

    // include the last number too
    if let Some(n) = parse(num_string.chars(), log)?.scalar_value() {
        v.push(n);
    } else {
        return Err(CalculatorError::RecursiveVectors);
    };

    Ok(NumType::Vector(v))
}

// TODO: FIX!
fn get_function_params<T: Iterator<Item = char> + Clone>(
    iter: &mut T,
    log: &Log,
) -> Result<Vec<NumType>, CalculatorError> {
    // Vector we will return when finished
    let mut v: Vec<NumType> = Vec::new();

    // Collect the entire function input into a string and separate by commas
    let mut params = iter.take_while(|c| *c != ')');
    while let Some(c) = params.next() {
        let mut s = String::from(c);
        let mut in_vector = false;
        params
            .by_ref()
            .take_while(|c| {
                if *c == '[' {
                    in_vector = true;
                } else if *c == ']' {
                    in_vector = false;
                }
                in_vector || *c != ','
            })
            .for_each(|c| s.push(c));
        v.push(parse(s.chars(), log)?);
    }

    // println!("{:?}", v);
    Ok(v)
}

#[cfg(test)]
mod tests;
