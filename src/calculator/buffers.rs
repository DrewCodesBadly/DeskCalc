use super::num_types::NumType;
use NumType::*;

#[derive(Default)]
pub struct MultiplicationBuffer {
    numbers: Vec<NumType>,
    pub dividing: bool,
}
#[derive(Default)]
pub struct AdditionBuffer {
    numbers: Vec<NumType>,
}

pub trait Collapse {
    fn collapse(&mut self) -> NumType;
}

impl Collapse for MultiplicationBuffer {
    // multiply everything in the buffer together, clear the buffer, and return the result
    fn collapse(&mut self) -> NumType {
        let mut p = Scalar(1.0);
        for n in &self.numbers {
            p = p * n;
        }
        self.numbers.clear();
        p
    }
}

impl Collapse for AdditionBuffer {
    // multiply everything in the buffer together, clear the buffer, and return the result
    fn collapse(&mut self) -> NumType {
        let mut p = Scalar(0.0);
        for n in &self.numbers {
            p = p + n;
        }
        self.numbers.clear();
        p
    }
}

impl AdditionBuffer {
    pub fn push(&mut self, n: NumType) {
        self.numbers.push(n);
    }
}
impl MultiplicationBuffer {
    pub fn push(&mut self, n: NumType) {
        if self.dividing {
            self.numbers.push(Scalar(1.0) / n);
            self.dividing = false;
        } else {
            self.numbers.push(n);
        }
    }
}
