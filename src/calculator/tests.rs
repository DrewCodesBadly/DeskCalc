use std::f64::consts::PI;

use super::*;
use crate::log::Log;
use NumType::*;

// test calc functions

#[test]
fn one_plus_one() {
    let result = calculate("1 + 1", &Log::default());
    assert_eq!(result, " = 2")
}

#[test]
fn two_minus_one() {
    let result = calculate("2 - 1", &Log::default());
    assert_eq!(result, " = 1")
}

#[test]
fn two_plus_neg_1() {
    let result = calculate("2 + -1", &Log::default());
    assert_eq!(result, " = 1")
}

#[test]
fn neg_3() {
    let result = calculate("-3", &Log::default());
    assert_eq!(result, " = -3")
}

#[test]
fn pemdas() {
    let result = calculate("1 + 2 * 3 / 2 + 1", &Log::default());
    assert_eq!(result, " = 5")
}

#[test]
fn parantheses() {
    let result = calculate("3 * (2 + 1)", &Log::default());
    assert_eq!(result, " = 9")
}

#[test]
fn big_numbers() {
    let result = calculate("3.253000 + 1450", &Log::default());
    assert_eq!(result, " = 1453.253")
}

#[test]
fn variable() {
    let mut log = Log::new();
    calculate_assign("#variable = 5 * 2", &mut log);
    let result = calculate("variable + 2", &log);
    assert_eq!(result, " = 12");
}

#[test]
fn consts() {
    let log = Log::new();
    let result = calculate("PI", &log);
    assert_eq!(result, " = ".to_string() + &PI.to_string());
}

#[test]
fn exponents() {
    let log = Log::new();
    let result = calculate("3 ^ 3", &log);
    assert_eq!(result, " = 27");
}

#[test]
fn exponents_order() {
    let log = Log::new();
    let result = calculate("3 + 2^3*4/2 - 1", &log);
    assert_eq!(result, " = 18");
}

#[test]
fn default_funcs() {
    let log = Log::new();
    // Not terminating parantheses after a function should just cause it to accept everything
    // should be 1 + (2 * 1) = 3
    let result = calculate("sin(PI/2) + abs(-2) * sin(PI/2", &log);
    assert_eq!(result, " = 3")
}

#[test]
fn multiple_fn_args() {
    let log = Log::new();
    let result = calculate("log(10, 5+5", &log);
    assert_eq!(result, " = 1")
}

#[test]
fn vector_math() {
    assert_eq!(Vector(vec![1.0, 1.0]) * Scalar(2.0), Vector(vec![2.0, 2.0]));
    assert_eq!(Vector(vec![1.0, 1.0]) + Scalar(2.0), Vector(vec![3.0, 3.0]));
    assert_eq!(Vector(vec![1.0, 1.0]) * Vector(vec![1.0, 2.0]), Vector(vec![1.0, 2.0]));
}
