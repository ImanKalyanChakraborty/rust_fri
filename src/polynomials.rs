use crate::field_operations::{Field, FieldElement};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub coefficients: Vec<FieldElement>, // Example: vec![1,2,3] => 1 + 2x + 3x^2
}

// Baby Bear field prime: 0x78000001 = 2013265921
const BABY_BEAR_P: i64 = 0x78000001;

impl Polynomial {

    // Returns degree of polynomial
    // Zero polynomial => -1
    pub fn degree(&self) -> i64 {

        if self.coefficients.is_empty() {
            return -1;
        }

        let zero = Field {p: BABY_BEAR_P}.zero();

        // Check if all coefficients are zero
        if self.coefficients.iter().all(|&x| x == zero) {
            return -1;
        }

        let mut max_index: i64 = 0;

        for i in 0..self.coefficients.len() {
            if self.coefficients[i] != zero {
                max_index = i as i64;
            }
        }

        max_index
    }

    // Negation of polynomial
    pub fn neg(&self) -> Polynomial {

        let mut negative_coefficients = self.coefficients.clone();

        for i in 0..negative_coefficients.len() {
            negative_coefficients[i] = -negative_coefficients[i].clone();
        }

        Polynomial {
            coefficients: negative_coefficients,
        }
    }

    // Addition of two polynomials
    pub fn add(&self, other: &Polynomial) -> Polynomial {

        if self.degree() == -1 {
            return other.clone();
        }

        if other.degree() == -1 {
            return self.clone();
        }

        let zero = Field { p: BABY_BEAR_P }.zero();

        let max_len = std::cmp::max(
            self.coefficients.len(),
            other.coefficients.len(),
        );

        let mut coeffs = vec![zero; max_len];

        // Add self coefficients
        for i in 0..self.coefficients.len() {
            coeffs[i] = coeffs[i].clone() + self.coefficients[i].clone();
        }

        // Add other coefficients
        for i in 0..other.coefficients.len() {
            coeffs[i] = coeffs[i].clone() + other.coefficients[i].clone();
        }

        Polynomial {
            coefficients: coeffs,
        }
    }

    // Subtraction
    pub fn sub(&self, other: &Polynomial) -> Polynomial {
        self.add(&other.neg())
    }

    // Multiplication
    pub fn mul(&self, other: &Polynomial) -> Polynomial {

        if self.coefficients.is_empty() || other.coefficients.is_empty() {
            return Polynomial {
                coefficients: vec![],
            };
        }

        let zero = Field {p: BABY_BEAR_P}.zero();

        let mut buf = vec![
            zero;
            self.coefficients.len() + other.coefficients.len() - 1
        ];

        for i in 0..self.coefficients.len() {

            if self.coefficients[i] == zero {
                continue;
            }

            for j in 0..other.coefficients.len() {
                buf[i + j] = buf[i + j].clone() + self.coefficients[i].clone() * other.coefficients[j].clone();
            }
        }

        Polynomial {
            coefficients: buf,
        }
    }

    // Equality check
    pub fn equals(&self, other: &Polynomial) -> bool {

        if self.degree() != other.degree() {
            return false;
        }

        if self.degree() == -1 {
            return true;
        }

        for i in 0..self.coefficients.len() {
            if self.coefficients[i] != other.coefficients[i] {
                return false;
            }
        }

        true
    }

    // Check if polynomial is zero
    pub fn is_zero(&self) -> bool {
        self.degree() == -1
    }

    // Leading coefficient
    pub fn leading_coefficient(&self) -> Option<FieldElement> {

        let deg = self.degree();

        if deg == -1 {
            return None;
        }

        Some(self.coefficients[deg as usize].clone())
    }
}