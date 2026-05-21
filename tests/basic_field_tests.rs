use rust_fri::field_operations::{Field, FieldElement, xgcd};

const BABY_BEAR_P: i64 = 0x78000001;

// Helper trait for modular inverse
trait ModInverse {
    fn mod_inverse(self, modulus: i64) -> i64;
}

impl ModInverse for i64 {
    fn mod_inverse(self, modulus: i64) -> i64 {
        let (a, _, _) = xgcd(self, modulus);
        ((a % modulus) + modulus) % modulus
    }
}

#[test]
fn test_baby_bear_field_operations() {
    let field = Field::new(BABY_BEAR_P);
    let a = FieldElement::new(5, field);
    let b = FieldElement::new(7, field);

    let sum = a + b;
    assert_eq!(sum.value, 12);

    let product = a * b;
    assert_eq!(product.value, 35);

    let diff = a - b;
    assert_eq!(diff.value, BABY_BEAR_P - 2);

    let quotient = a / b;
    let inv_7 = 7_i64.mod_inverse(BABY_BEAR_P);
    let expected = (5 * inv_7) % BABY_BEAR_P;
    assert_eq!(quotient.value, expected);
}

#[test]
fn test_baby_bear_exponentiation() {
    let field = Field::new(BABY_BEAR_P);
    let a = FieldElement::new(2, field);
    let result = a ^ 3;
    assert_eq!(result.value, 8);

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

    assert_eq!((a * inv).value, 1);

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
    assert_eq!(sum.value, 0);

    let product = a * b;
    assert_eq!(product.value, BABY_BEAR_P - 1);

    // Test large values
    let large = FieldElement::new(1_000_000_000, field);
    let product = large * large;
    let expected = (1_000_000_000_i64 * 1_000_000_000) % BABY_BEAR_P;
    assert_eq!(product.value, expected);
}

#[test]
fn test_generator_and_roots() {
    let field = Field::new(BABY_BEAR_P);

    // Test generator
    let generator = field.generator();
    assert_eq!((generator ^ (BABY_BEAR_P - 1)).value, 1);

    // Test primitive nth roots for various powers of two
    for exp in 1..=10 {
        let n = 1 << exp; // 2, 4, 8, 16, ... 1024
        if n <= 1 << 27 {
            let root = field.primitive_nth_root(n);
            assert_eq!((root ^ n).value, 1, "Failed for n={}", n);

            // For n > 2, root^(n/2) should not be 1
            if n > 2 {
                assert_ne!((root ^ (n / 2)).value, 1, "Failed for n={}", n);
            }
        }
    }
}

#[test]
fn test_sample_single_byte() {
    let field = Field::new(BABY_BEAR_P);

    // Test 1
    let bytes = [0x05];
    let result = field.sample(&bytes);
    assert_eq!(result.value, 5);

    // Test 2
    let bytes = [0xFF];
    let result = field.sample(&bytes);
    assert_eq!(result.value, 0xFF);
}

#[test]
fn test_sample_multiple_bytes() {
    let field = Field::new(BABY_BEAR_P);

    // Two bytes: 0x1234
    let bytes = [0x12, 0x34];
    let elem = field.sample(&bytes);
    assert_eq!(elem.value, 0x1234);

    // Three bytes: 0x123456
    let bytes = [0x12, 0x34, 0x56];
    let elem = field.sample(&bytes);
    assert_eq!(elem.value, 0x123456);

    // Four bytes: 0x12345678
    let bytes = [0x12, 0x34, 0x56, 0x78];
    let elem = field.sample(&bytes);
    assert_eq!(elem.value, 0x12345678);
}
