use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg},
};

// enum containing different types of numbers the calculator may handle
#[derive(Clone, Debug)]
pub enum NumType {
    Scalar(f64),
}

use NumType::*;

// implement math for NumType
impl Mul for NumType {
    type Output = NumType;
    fn mul(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l * r),
        };
    }
}

impl Mul<&NumType> for NumType {
    type Output = NumType;
    fn mul(self, rhs: &Self) -> Self::Output {
        return match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l * r),
        };
    }
}

impl Div for NumType {
    type Output = NumType;
    fn div(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l / r),
        };
    }
}

impl Add for NumType {
    type Output = NumType;
    fn add(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l + r),
        };
    }
}

impl Add<&NumType> for NumType {
    type Output = NumType;
    fn add(self, rhs: &Self) -> Self::Output {
        return match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l + r),
        };
    }
}

impl Neg for NumType {
    type Output = NumType;
    fn neg(self) -> Self::Output {
        match self {
            Scalar(s) => Scalar(-s),
        }
    }
}

// String representations
impl Display for NumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scalar(s) => write!(f, "{}", s.to_string()),
        }
    }
}

// Exponents and more
impl NumType {
    pub fn pow(self, n: &NumType) -> NumType {
        match (self, n) {
            (Scalar(b), Scalar(e)) => Scalar(b.powf(*e)),
        }
    }
}
