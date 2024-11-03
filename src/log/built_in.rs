use crate::calculator::num_types::NumType;
use crate::calculator::num_types::NumType::*;
use std::collections::HashMap;
use std::f64::consts::{E, PI, TAU};

pub fn get_constants_hashmap() -> HashMap<String, NumType> {
    let mut c: HashMap<String, NumType> = HashMap::new();
    c.insert("PI".to_owned(), Scalar(PI));
    c.insert("pi".to_owned(), Scalar(PI));
    c.insert("Pi".to_owned(), Scalar(PI));
    c.insert("e".to_owned(), Scalar(E));
    c.insert("E".to_owned(), Scalar(E));
    c.insert("TAU".to_owned(), Scalar(TAU));
    c.insert("Tau".to_owned(), Scalar(TAU));
    c.insert("tau".to_owned(), Scalar(TAU));

    c
}
