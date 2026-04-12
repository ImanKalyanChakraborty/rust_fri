use std::fmt;
use std::ops::{Add, Mul, Sub, Div, Neg, BitXor};

fn xgcd(x: i64, y: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (x, y);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        
        // Store old values before updating
        let (next_old_r, next_r) = (r, old_r - quotient * r);
        let (next_old_s, next_s) = (s, old_s - quotient * s);
        let (next_old_t, next_t) = (t, old_t - quotient * t);
        
        old_r = next_old_r;
        r = next_r;
        old_s = next_old_s;
        s = next_s;
        old_t = next_old_t;
        t = next_t;
    }

    (old_s, old_t, old_r)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    pub value: i64,
    pub field: Field,
}

impl FieldElement {
    pub fn new(value: i64, field: Field) -> Self {
        FieldElement { value, field }
    }
    
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
    
    pub fn inverse(&self) -> Self {
        self.field.inverse(self)
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for FieldElement {
    type Output = Self;
    
    fn add(self, right: Self) -> Self {
        self.field.add(&self, &right)
    }
}

impl<'a> Add<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    
    fn add(self, right: &'a FieldElement) -> FieldElement {
        self.field.add(&self, right)
    }
}

impl Sub for FieldElement {
    type Output = Self;
    
    fn sub(self, right: Self) -> Self {
        self.field.subtract(&self, &right)
    }
}

impl Mul for FieldElement {
    type Output = Self;
    
    fn mul(self, right: Self) -> Self {
        self.field.multiply(&self, &right)
    }
}

impl Div for FieldElement {
    type Output = Self;
    
    fn div(self, right: Self) -> Self {
        self.field.divide(&self, &right)
    }
}

impl Neg for FieldElement {
    type Output = Self;
    
    fn neg(self) -> Self {
        self.field.negate(&self)
    }
}

impl BitXor<i64> for FieldElement {
    type Output = Self;
    
    fn bitxor(self, exponent: i64) -> Self {
        let mut acc = FieldElement::new(1, self.field.clone());
        let val = FieldElement::new(self.value, self.field.clone());
        
        for i in (0..64).rev() {
            acc = acc * acc;
            if (exponent >> i) & 1 != 0 {
                acc = acc * val.clone();
            }
        }
        acc
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    pub p: i64,
}

impl Field {
    // Baby Bear field prime: 0x78000001 = 2013265921
    const BABY_BEAR_P: i64 = 0x78000001;
    // Basic Field Operations
    pub fn new(p: i64) -> Self {
        Field { p }
    }
    
    pub fn zero(&self) -> FieldElement {
        FieldElement::new(0, *self)
    }
    
    pub fn one(&self) -> FieldElement {
        FieldElement::new(1, *self)
    }
    
    pub fn multiply(&self, left: &FieldElement, right: &FieldElement) -> FieldElement {
        FieldElement::new((left.value * right.value) % self.p, *self)
    }
    
    pub fn add(&self, left: &FieldElement, right: &FieldElement) -> FieldElement {
        FieldElement::new((left.value + right.value) % self.p, *self)
    }
    
    pub fn subtract(&self, left: &FieldElement, right: &FieldElement) -> FieldElement {
        FieldElement::new((self.p + left.value - right.value) % self.p, *self)
    }
    
    pub fn negate(&self, operand: &FieldElement) -> FieldElement {
        FieldElement::new((self.p - operand.value) % self.p, *self)
    }
    
    pub fn inverse(&self, operand: &FieldElement) -> FieldElement {
        let (a, _, _) = xgcd(operand.value, self.p);
        // Ensure a is positive
        let a = ((a % self.p) + self.p) % self.p;
        FieldElement::new(a, *self)
    }
    
    pub fn divide(&self, left: &FieldElement, right: &FieldElement) -> FieldElement {
        assert!(!right.is_zero(), "divide by zero");
        let (a, _, _) = xgcd(right.value, self.p);
        let a = ((a % self.p) + self.p) % self.p;
        FieldElement::new((left.value * a) % self.p, *self)
    }

    // Generator and Primitive nth root
    pub fn generator(&self) -> FieldElement {
        assert_eq!(self.p, Self::BABY_BEAR_P, 
            "Do not know generator for other fields beyond BabyBear");

        // 31 is a valid primitive root of BabyBear
        FieldElement::new(31, *self)
    }

}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;
    
    // Baby Bear field prime: 0x78000001 = 2013265921
    const BABY_BEAR_P: i64 = 0x78000001;
    
    #[test]
    fn test_baby_bear_field_operations() {
        let field = Field::new(BABY_BEAR_P);
        let a = FieldElement::new(5, field);
        let b = FieldElement::new(7, field);
        
        let sum = a + b;
        assert_eq!(sum.value, 12); // 5 + 7 = 12 mod p
        
        let product = a * b;
        assert_eq!(product.value, 35); // 5 * 7 = 35 mod p (less than p)
        
        let diff = a - b;
        assert_eq!(diff.value, BABY_BEAR_P - 2); // (5 - 7) mod p = p - 2
        
        let quotient = a / b;
        // 7^(-1) mod Baby Bear
        // Since p is prime, we can compute: 7^(-1) mod p
        let inv_7 = 7_i64.mod_inverse(BABY_BEAR_P);
        let expected = (5 * inv_7) % BABY_BEAR_P;
        assert_eq!(quotient.value, expected);
    }
    
    #[test]
    fn test_baby_bear_exponentiation() {
        let field = Field::new(BABY_BEAR_P);
        let a = FieldElement::new(2, field);
        let result = a ^ 3;
        assert_eq!(result.value, 8); // 2^3 = 8 mod Baby Bear
        
        // Test larger exponent
        let a = FieldElement::new(3, field);
        let result = a ^ 10;
        let expected = 3_i64.pow(10) % BABY_BEAR_P;
        assert_eq!(result.value, expected);
    }
    
    #[test]
    fn test_baby_bear_inverse() {
        let field = Field::new(BABY_BEAR_P);
        let a = FieldElement::new(3, field);
        let inv = a.inverse();
        
        // 3 * inv ≡ 1 mod Baby Bear
        assert_eq!((a * inv).value, 1);
        
        // The inverse of 3 mod Baby Bear should be:
        // (p + 1) / 3 = 671088640.666? Let's compute properly
        let expected_inv = 3_i64.mod_inverse(BABY_BEAR_P);
        assert_eq!(inv.value, expected_inv);
    }
    
    #[test]
    fn test_baby_bear_properties() {
        let field = Field::new(BABY_BEAR_P);
        
        // Test additive identity
        let a = FieldElement::new(12345, field);
        let zero = field.zero();
        assert_eq!((a + zero).value, a.value);
        
        // Test multiplicative identity
        let one = field.one();
        assert_eq!((a * one).value, a.value);
        
        // Test additive inverse
        let neg_a = -a;
        assert_eq!((a + neg_a).value, 0);
        
        // Test multiplicative inverse property
        if !a.is_zero() {
            let inv_a = a.inverse();
            assert_eq!((a * inv_a).value, 1);
        }
    }
    
    #[test]
    fn test_baby_bear_boundary_values() {
        let field = Field::new(BABY_BEAR_P);
        
        // Test near modulus boundary
        let a = FieldElement::new(BABY_BEAR_P - 1, field);
        let b = FieldElement::new(1, field);
        
        let sum = a + b;
        assert_eq!(sum.value, 0); // (p-1) + 1 = p ≡ 0
        
        let product = a * b;
        assert_eq!(product.value, BABY_BEAR_P - 1);
        
        // Test large values
        let large = FieldElement::new(1_000_000_000, field);
        let product = large * large;
        let expected = (1_000_000_000_i64 * 1_000_000_000) % BABY_BEAR_P;
        assert_eq!(product.value, expected);
    }
}

// Helper trait for modular inverse (if you don't have one)
#[cfg(test)]
trait ModInverse {
    fn mod_inverse(self, modulus: i64) -> i64;
}

#[cfg(test)]
impl ModInverse for i64 {
    fn mod_inverse(self, modulus: i64) -> i64 {
        let (a, _, _) = xgcd(self, modulus);
        ((a % modulus) + modulus) % modulus
    }
}