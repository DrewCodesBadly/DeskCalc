use super::*;

// test calc functions

#[test]
fn one_plus_one() {
    let result = calculate("1 + 1").unwrap();
    assert_eq!(result, "2")
}

#[test]
fn two_minus_one() {
    let result = calculate("2 - 1").unwrap();
    assert_eq!(result, "1")
}

#[test]
fn two_plus_neg_1() {
    let result = calculate("2 + -1").unwrap();
    assert_eq!(result, "1")
}

#[test]
fn neg_3() {
    let result = calculate("-3").unwrap();
    assert_eq!(result, "-3")
}

#[test]
fn pemdas() {
    let result = calculate("1 + 2 * 3 / 2 + 1").unwrap();
    assert_eq!(result, "5")
}

#[test]
fn parantheses() {
    let result = calculate("3 * (2 + 1)").unwrap();
    assert_eq!(result, "9")
}

#[test]
fn big_numbers() {
    let result = calculate("3.253000 + 1450").unwrap();
    assert_eq!(result, "1453.253")
}