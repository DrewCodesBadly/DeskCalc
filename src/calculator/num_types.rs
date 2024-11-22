use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg},
};

// enum containing different types of numbers the calculator may handle
#[derive(Clone, Debug, PartialEq)]
pub enum NumType {
    Scalar(f64),
    Vector(Vec<f64>),
}

use NumType::*;

// implement math for NumType
impl Mul for NumType {
    type Output = NumType;
    fn mul(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l * r),
            (Vector(v), Scalar(n)) => Vector(v.iter().map(|f| f * n).collect()),
            (Scalar(n), Vector(v)) => Vector(v.iter().map(|f| f * n).collect()),
            // Vector/Vector behavior multiplies each component by the next vector's corresponding component
            // If there is no corresponding component it uses 1 instead
            (Vector(v), Vector(v2)) => {
                let mut i = v2.iter();
                Vector(v.iter().map(|f| f * i.next().unwrap_or(&1.0)).collect())
            }
        };
    }
}

impl Mul<&NumType> for NumType {
    type Output = NumType;
    fn mul(self, rhs: &Self) -> Self::Output {
        return match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l * r),
            (Vector(v), Scalar(n)) => Vector(v.iter().map(|f| f * n).collect()),
            (Scalar(n), Vector(v)) => Vector(v.iter().map(|f| f * n).collect()),
            // Vector/Vector behavior multiplies each component by the next vector's corresponding component
            // If there is no corresponding component it uses 1 instead
            (Vector(v), Vector(v2)) => {
                let mut i = v2.iter();
                Vector(v.iter().map(|f| f * i.next().unwrap_or(&1.0)).collect())
            }
        };
    }
}

impl Div for NumType {
    type Output = NumType;
    fn div(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l / r),
            (Vector(v), Scalar(n)) => Vector(v.iter().map(|f| f / n).collect()),
            (Scalar(n), Vector(v)) => Vector(v.iter().map(|f| f / n).collect()),
            // Vector/Vector behavior divides each component by the next vector's corresponding component
            // If there is no corresponding component it uses 1 instead
            (Vector(v), Vector(v2)) => {
                let mut i = v2.iter();
                Vector(v.iter().map(|f| f / i.next().unwrap_or(&1.0)).collect())
            }
        };
    }
}

impl Add for NumType {
    type Output = NumType;
    fn add(self, rhs: Self) -> Self::Output {
        return match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l + r),
            (Vector(v), Scalar(n)) => Vector(v.iter().map(|f| f + n).collect()),
            (Scalar(n), Vector(v)) => Vector(v.iter().map(|f| f + n).collect()),
            // Vector/Vector behavior adds each component by the next vector's corresponding component
            // If there is no corresponding component it uses 1 instead
            (Vector(v), Vector(v2)) => {
                let mut i = v2.iter();
                Vector(v.iter().map(|f| f + i.next().unwrap_or(&1.0)).collect())
            }
        };
    }
}

impl Add<&NumType> for NumType {
    type Output = NumType;
    fn add(self, rhs: &Self) -> Self::Output {
        return match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l + r),
            (Vector(v), Scalar(n)) => Vector(v.iter().map(|f| f + n).collect()),
            (Scalar(n), Vector(v)) => Vector(v.iter().map(|f| f + n).collect()),
            // Vector/Vector behavior divides each component by the next vector's corresponding component
            // If there is no corresponding component it uses 1 instead
            (Vector(v), Vector(v2)) => {
                let mut i = v2.iter();
                Vector(v.iter().map(|f| f + i.next().unwrap_or(&1.0)).collect())
            }
        };
    }
}

impl Neg for NumType {
    type Output = NumType;
    fn neg(self) -> Self::Output {
        match self {
            Scalar(s) => Scalar(-s),
            Vector(v) => Vector(v.iter().map(|f| -f).collect()),
        }
    }
}

// String representations
impl Display for NumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scalar(s) => write!(f, "{}", s.to_string()),
            Vector(v) => {
                write!(
                    f,
                    "[{}]",
                    v.iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}

// Exponents and more
impl NumType {
    pub fn pow(self, rhs: &NumType) -> NumType {
        match (self, rhs) {
            (Scalar(l), Scalar(r)) => Scalar(l.powf(*r)),
            (Vector(v), Scalar(n)) => Vector(v.iter().map(|f| f.powf(*n)).collect()),
            (Scalar(n), Vector(v)) => Vector(v.iter().map(|f| f.powf(n)).collect()),
            // Vector/Vector behavior divides each component by the next vector's corresponding component
            // If there is no corresponding component it uses 1 instead
            (Vector(v), Vector(v2)) => {
                let mut i = v2.iter();
                Vector(
                    v.iter()
                        .map(|f| f.powf(*i.next().unwrap_or(&1.0)))
                        .collect(),
                )
            }
        }
    }

    pub fn scalar_value(self) -> Option<f64> {
        return match self {
            Scalar(s) => Some(s),
            Vector(_) => None,
        };
    }
}
