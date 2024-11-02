mod math;
mod buffers;
mod num_types;

use std::clone::Clone;
use itertools::Itertools;
use num_types::NumType;
use buffers::Collapse;

#[derive(Debug)]
pub enum CalculatorError {
    UnknownSymbol(String),
    MissingVarEquals,
    MissingParantheses,
    ParseNumberErrror
}

impl CalculatorError {
    pub fn as_string(&self) -> String {
        match self {
            CalculatorError::UnknownSymbol(s) => format!("Error: Could not find symbol \"{}\"", s),
            CalculatorError::MissingVarEquals => String::from("Error: Missing \"=\" after variable name"),
            CalculatorError::MissingParantheses => String::from("Error: Missing closing \")\""),
            CalculatorError::ParseNumberErrror => String::from("Error: Invalid number")
        }
    }
}

// Parses an input string to calculate the output
pub fn calculate(input: &str) -> Result<String, CalculatorError> {

    // remove any whitespace at all from the string
    let mut expression = input.replace(' ', "");

    // Return nothing if given nothing
    if expression.is_empty() { return Ok(String::from("")) }

    // check if we are assigning a new calculator variable using #var_name= syntax
    let assigning_to: Option<String> = match expression.chars().next().expect("Previously checked if string was empty") {
        '#' => {
            // Find index of '=' (signifies the end of the variable name) or throw an error
            let var_end = match expression.find('=') {
                Some(i) => i,
                None => return Err(CalculatorError::MissingVarEquals)
            };

            // Remove var from expression so as to not confuse parse()
            expression.remove(0);
            let var_name: String = expression.drain(..var_end).collect();

            Some(var_name)
        }
        _ => None
    };

    let answer = parse(expression.chars())?;

    // TODO: Assign variables if needed
    if assigning_to.is_some() {
        // ...
    }

    Ok(answer.to_string())
}

fn parse<T: Iterator<Item = char> + Clone>(mut input: T) -> Result<NumType, CalculatorError> {

    // Exponents and functions are instantly evaluated, they do not need a buffer
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
                m_buffer.push(
                    parse(input
                        .by_ref()
                        .take_while(|c| *c != ')')
                        .collect::<Vec<char>>()
                        .iter()
                        .cloned()
                    )?
                );

                previous_number = true; // () expressions are treated as a single number after evaluation
            }

            // Check if we are making a vector type, put it into the multiplication buffer
            '[' => {

                previous_number = true;
            }

            // Check for a number, put it into the multiplication buffer
            '0'..='9' | '.' => {
                m_buffer.push(
                    NumType::Scalar(parse_chars_to_f64(c, &mut input)?)
                );

                previous_number = true;
            }

            // Check for operators
            // '*' doesn't really do anything since numbers already go in multiplication buffer
            '*' => {
                previous_number = false;
            }
            // '/' only sets the multiplication buffer to take the reciprocal of the next number
            '/' => {
                m_buffer.dividing = true;
                previous_number = false;
            }
            // '+' collapses the multiplication buffer into the addition buffer
            '+' => {
                a_buffer.push(m_buffer.collapse());
                previous_number = false;
            }
            // '-' pushes -1 to the multiplication buffer and, if the last part was a number, collapses the multiplication buffer
            '-' => {
                // Only collapse the multiplication buffer if the last thing was a number
                // If it was an operator this is unary minus not subtraction
                if previous_number {
                    a_buffer.push(m_buffer.collapse());
                    previous_number = false;
                }

                // Push -1 to the multiplication buffer to reverse the output
                m_buffer.push(NumType::Scalar(-1.0));
            }

            // Else, it must be some sort of symbol
            // Variables go in the multiplication buffer
            // Function outputs go in the multiplication buffer
            _ => {

            }
        }
    };

    a_buffer.push(m_buffer.collapse());
    Ok(a_buffer.collapse())
}

// Turns the character iterator into a float or throws an error
fn parse_chars_to_f64<T: Iterator<Item = char> + Clone>(first: char, iter: &mut T) -> Result<f64, CalculatorError> {
    if let Ok(f) = (
        first.to_string() + &(
            iter
            .take_while_ref(|c| *c == '.' || ('0'..='9').contains(c))
            .collect::<String>()
        )
    )
    .parse()
    {
        Ok(f)
    }

    else
    {
        Err(CalculatorError::ParseNumberErrror)
    }
}

#[cfg(test)]
mod tests;