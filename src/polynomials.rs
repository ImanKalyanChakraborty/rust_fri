use rust_fri::Field;

use crate::field_operations::Field;
pub struct Polynomial {
    pub coefficients: Vec<i64>, // Usage : let p = Polynomial { coefficients: vec![1,2,3]}
}

impl Polynomial {
    pub fn degree (&self) -> i64 {
        if (self.coefficients.is_empty()) {
            -1
        } else {
            let zero = Field::zero(&self);

            if (self.coefficients == Vec[zero] * self.coefficients.len()) {
                -1
            } else {
                let mut max_index : i64 = 0;

                for i in 0..self.coefficients.len() {
                    if self.coefficients[i] != 0 {
                        max_index = i as i64;
                    }
                }
                
                return max_index;
            }
        }
    }

    pub fn neg (&self) -> Polynomial {
        let mut negative_coefficients = &self.coefficients;

        for i in 0..self.coefficients.len() {
            negative_coefficients[i] = -negative_coefficients[i];
        }

        Polynomial { coefficients: negative_coefficients}
    }

    pub fn add (&self, other) -> Polynomial {
        if (self.degree() == -1) {
            other
        } else if ()
    }
}