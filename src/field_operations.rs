use std::{backtrace, fmt};
use std::ops::{Add, Mul, Sub, Div, Neg, BitXor};

pub fn xgcd(x: i64, y: i64) -> (i64, i64, i64) {
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

    pub fn primitive_nth_root(&self, n: i64) -> FieldElement {
        assert_eq!(
            self.p,
            Self::BABY_BEAR_P,
            "Unknown field, can't return root of unity"
        );

        // p - 1 = 2^27 * 3 * 5 → max power-of-two root is 2^27
        let max_power_of_two = 1 << 27;

        assert!(
            n > 0 && n <= max_power_of_two && (n & (n - 1)) == 0,
            "n must be a power of two ≤ 2^27"
        );

        // Using generator g = 31
        let generator = self.generator();

        // Compute primitive n-th root directly
        generator ^ ((self.p - 1) / n)
    }

    pub fn sample(&self, byte_array: &[u8]) -> FieldElement {
        let mut acc = 0;

        for &b in byte_array {
            acc = (acc << 8) ^ (b as i64);
        }

        // Reduce modulo p and ensure positive
        let value = acc % self.p;
        let value = if value < 0 { value + self.p } else { value };

        FieldElement::new(value, *self)
    }
}
