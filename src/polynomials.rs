use crate::field_operations::{Field, FieldElement};
use std::ops::{Div, Mul, Rem, Sub, BitXor};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub coefficients: Vec<FieldElement>, // Example: vec![1,2,3] => 1 + 2x + 3x^2
}

// Baby Bear field prime: 0x78000001 = 2013265921
const BABY_BEAR_P: i64 = 0x78000001;

impl Polynomial {
    pub fn new(coeffs: Vec<FieldElement>) -> Polynomial {
        Polynomial { coefficients: coeffs }
    }

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

    // Division
    pub fn divide(
        numerator: &Polynomial,
        denominator: &Polynomial,
    ) -> Option<(Polynomial, Polynomial)> {

        // Division by zero polynomial
        if denominator.degree() == -1 {
            return None;
        }

        // If numerator degree < denominator degree
        if numerator.degree() < denominator.degree() {
            return Some((
                Polynomial::new(vec![]),
                numerator.clone(),
            ));
        }

        let field = denominator.coefficients[0].field;

        let mut remainder = numerator.clone();

        let quotient_len =
            (numerator.degree() - denominator.degree() + 1) as usize;

        let mut quotient_coefficients =
            vec![field.zero(); quotient_len];

        while remainder.degree() >= denominator.degree()
            && !remainder.is_zero()
        {
            let coefficient =
                remainder.leading_coefficient().unwrap()
                / denominator.leading_coefficient().unwrap();

            let shift =
                (remainder.degree() - denominator.degree()) as usize;

            // Build monomial: coefficient * x^shift
            let mut monomial_coeffs =
                vec![field.zero(); shift];

            monomial_coeffs.push(coefficient);

            let subtractee =
                Polynomial::new(monomial_coeffs)
                * denominator.clone();

            quotient_coefficients[shift] = coefficient;

            remainder = remainder - subtractee;
        }

        let quotient = Polynomial::new(quotient_coefficients);

        Some((quotient, remainder))
    }
}

// Overloaded Operators
impl Div for Polynomial {
    type Output = Polynomial;

    fn div(self, other: Polynomial) -> Polynomial {
        let (quo, rem) =
            Polynomial::divide(&self, &other)
            .expect("Polynomial division by zero");

        assert!(
            rem.is_zero(),
            "cannot perform polynomial division because remainder is not zero"
        );

        quo
    }
}

impl Rem for Polynomial {
    type Output = Polynomial;

    fn rem(self, other: Polynomial) -> Polynomial {
        let (_, rem) =
            Polynomial::divide(&self, &other)
            .expect("Polynomial division by zero");

        rem
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Polynomial::mul(&self, &rhs)
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Polynomial::sub(&self, &rhs)
    }
}

impl BitXor<i64> for Polynomial {
    type Output = Polynomial;

    fn bitxor(self, exponent: i64) -> Polynomial {

        assert!(exponent >= 0, "Negative exponents are not supported");

        // x^0 = 1
        if exponent == 0 {
            return Polynomial::new(vec![
                self.coefficients[0].field.one()
            ]);
        }

        // 0^n = 0 for n > 0
        if self.is_zero() {
            return Polynomial::new(vec![]);
        }

        let field = self.coefficients[0].field;

        // acc = 1
        let mut acc = Polynomial::new(vec![field.one()]);

        // Copy exponent since we'll mutate it
        let mut exp = exponent;

        // Copy base polynomial
        let mut base = self.clone();

        // Standard binary exponentiation
        while exp > 0 {

            // If current bit is set
            if exp & 1 == 1 {
                acc = acc * base.clone();
            }

            // Square base
            base = base.clone() * base;

            // Shift exponent right
            exp >>= 1;
        }

        acc
    }
}