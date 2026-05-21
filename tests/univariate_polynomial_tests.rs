use rust_fri::field_operations::{Field, FieldElement};
use rust_fri::polynomials::Polynomial;

// Baby Bear field prime
const BABY_BEAR_P: i64 = 0x78000001;

fn field() -> Field {
    Field::new(BABY_BEAR_P)
}

fn fe(val: i64) -> FieldElement {
    FieldElement::new(val, Field::new(BABY_BEAR_P))
}

fn poly(coeffs: Vec<i64>) -> Polynomial {
    Polynomial::new(coeffs.into_iter().map(fe).collect())
}

// -------------------------------------------------------------------------
// degree()
// -------------------------------------------------------------------------

#[test]
fn test_degree_empty() {
    let p = Polynomial::new(vec![]);
    assert_eq!(p.degree(), -1, "Empty polynomial should have degree -1");
}

#[test]
fn test_degree_all_zeros() {
    let p = poly(vec![0, 0, 0]);
    assert_eq!(p.degree(), -1, "All-zero polynomial should have degree -1");
}

#[test]
fn test_degree_constant() {
    let p = poly(vec![5]);
    assert_eq!(p.degree(), 0, "Constant non-zero polynomial should have degree 0");
}

#[test]
fn test_degree_linear() {
    let p = poly(vec![0, 3]);
    assert_eq!(p.degree(), 1);
}

#[test]
fn test_degree_quadratic() {
    let p = poly(vec![1, 2, 3]);
    assert_eq!(p.degree(), 2);
}

#[test]
fn test_degree_leading_zeros_ignored() {
    // vec![1, 2, 0] represents 1 + 2x + 0x^2 => degree 1
    let p = poly(vec![1, 2, 0]);
    assert_eq!(p.degree(), 1);
}

// -------------------------------------------------------------------------
// neg()
// -------------------------------------------------------------------------

#[test]
fn test_neg_basic() {
    let p = poly(vec![1, 2, 3]);
    let neg_p = p.neg();
    // neg_p + p should be zero
    let sum = p.add(&neg_p);
    assert!(sum.is_zero(), "p + (-p) should be zero");
}

#[test]
fn test_neg_zero_poly() {
    let p = poly(vec![0]);
    let neg_p = p.neg();
    assert!(neg_p.is_zero());
}

#[test]
fn test_neg_double_negation() {
    let p = poly(vec![1, 2, 3]);
    let double_neg = p.neg().neg();
    assert!(p.equals(&double_neg), "--p should equal p");
}

// -------------------------------------------------------------------------
// add()
// -------------------------------------------------------------------------

#[test]
fn test_add_basic() {
    // (1 + 2x) + (3 + 4x) = 4 + 6x
    let a = poly(vec![1, 2]);
    let b = poly(vec![3, 4]);
    let result = a.add(&b);
    let expected = poly(vec![4, 6]);
    assert!(result.equals(&expected));
}

#[test]
fn test_add_different_degrees() {
    // (1) + (0 + 0x + 5x^2) = 1 + 0x + 5x^2
    let a = poly(vec![1]);
    let b = poly(vec![0, 0, 5]);
    let result = a.add(&b);
    let expected = poly(vec![1, 0, 5]);
    assert!(result.equals(&expected));
}

#[test]
fn test_add_zero_polynomial() {
    let a = poly(vec![1, 2, 3]);
    let zero = Polynomial::new(vec![]);
    assert!(a.add(&zero).equals(&a));
    assert!(zero.add(&a).equals(&a));
}

#[test]
fn test_add_cancellation() {
    // p + (-p) = 0
    let p = poly(vec![7, 3, 11]);
    let neg_p = p.neg();
    let result = p.add(&neg_p);
    assert!(result.is_zero());
}

#[test]
fn test_add_operator_overload() {
    let a = poly(vec![1, 2]);
    let b = poly(vec![3, 4]);
    let result = a.clone() + b.clone();
    let expected = a.add(&b);
    assert!(result.equals(&expected));
}

// -------------------------------------------------------------------------
// sub()
// -------------------------------------------------------------------------

#[test]
fn test_sub_basic() {
    // (5 + 6x) - (3 + 4x) = 2 + 2x
    let a = poly(vec![5, 6]);
    let b = poly(vec![3, 4]);
    let result = a.sub(&b);
    let expected = poly(vec![2, 2]);
    assert!(result.equals(&expected));
}

#[test]
fn test_sub_self() {
    let p = poly(vec![1, 2, 3]);
    let result = p.sub(&p);
    assert!(result.is_zero(), "p - p should be zero");
}

#[test]
fn test_sub_zero_polynomial() {
    let p = poly(vec![1, 2, 3]);
    let zero = Polynomial::new(vec![]);
    assert!(p.sub(&zero).equals(&p));
}

#[test]
fn test_sub_operator_overload() {
    let a = poly(vec![5, 6]);
    let b = poly(vec![3, 4]);
    let result = a.clone() - b.clone();
    let expected = a.sub(&b);
    assert!(result.equals(&expected));
}

// -------------------------------------------------------------------------
// mul()
// -------------------------------------------------------------------------

#[test]
fn test_mul_basic() {
    // (1 + x) * (1 + x) = 1 + 2x + x^2
    let a = poly(vec![1, 1]);
    let result = a.clone().mul(&a);
    let expected = poly(vec![1, 2, 1]);
    assert!(result.equals(&expected));
}

#[test]
fn test_mul_by_zero() {
    let p = poly(vec![1, 2, 3]);
    let zero = Polynomial::new(vec![]);
    assert!(p.mul(&zero).is_zero());
    assert!(zero.mul(&p).is_zero());
}

#[test]
fn test_mul_by_one() {
    // (1 + 2x + 3x^2) * 1 = (1 + 2x + 3x^2)
    let p = poly(vec![1, 2, 3]);
    let one = poly(vec![1]);
    assert!(p.mul(&one).equals(&p));
}

#[test]
fn test_mul_degree_adds() {
    let a = poly(vec![1, 1]); // degree 1
    let b = poly(vec![1, 0, 1]); // degree 2
    let result = a.mul(&b);
    assert_eq!(result.degree(), 3, "Degree of product should be sum of degrees");
}

#[test]
fn test_mul_commutativity() {
    let a = poly(vec![1, 2]);
    let b = poly(vec![3, 4, 5]);
    assert!(a.mul(&b).equals(&b.mul(&a)));
}

#[test]
fn test_mul_operator_overload() {
    let a = poly(vec![1, 2]);
    let b = poly(vec![3, 4]);
    let result = a.clone() * b.clone();
    let expected = a.mul(&b);
    assert!(result.equals(&expected));
}

// -------------------------------------------------------------------------
// equals() and is_zero()
// -------------------------------------------------------------------------

#[test]
fn test_equals_same() {
    let p = poly(vec![1, 2, 3]);
    assert!(p.equals(&p.clone()));
}

#[test]
fn test_equals_different() {
    let a = poly(vec![1, 2, 3]);
    let b = poly(vec![1, 2, 4]);
    assert!(!a.equals(&b));
}

#[test]
fn test_equals_different_degrees() {
    let a = poly(vec![1, 2]);
    let b = poly(vec![1, 2, 3]);
    assert!(!a.equals(&b));
}

#[test]
fn test_is_zero_empty() {
    let p = Polynomial::new(vec![]);
    assert!(p.is_zero());
}

#[test]
fn test_is_zero_all_zero_coeffs() {
    let p = poly(vec![0, 0]);
    assert!(p.is_zero());
}

#[test]
fn test_is_zero_nonzero() {
    let p = poly(vec![0, 1]);
    assert!(!p.is_zero());
}

// -------------------------------------------------------------------------
// leading_coefficient()
// -------------------------------------------------------------------------

#[test]
fn test_leading_coefficient_zero_poly() {
    let p = Polynomial::new(vec![]);
    assert!(p.leading_coefficient().is_none());
}

#[test]
fn test_leading_coefficient_constant() {
    let p = poly(vec![7]);
    assert_eq!(p.leading_coefficient(), Some(fe(7)));
}

#[test]
fn test_leading_coefficient_higher_degree() {
    // 1 + 2x + 3x^2 => leading coeff is 3
    let p = poly(vec![1, 2, 3]);
    assert_eq!(p.leading_coefficient(), Some(fe(3)));
}

#[test]
fn test_leading_coefficient_trailing_zero() {
    // 1 + 2x + 0x^2 => degree is 1, leading coeff is 2
    let p = poly(vec![1, 2, 0]);
    assert_eq!(p.leading_coefficient(), Some(fe(2)));
}

// -------------------------------------------------------------------------
// divide()
// -------------------------------------------------------------------------

#[test]
fn test_divide_by_zero_returns_none() {
    let p = poly(vec![1, 2, 3]);
    let zero = Polynomial::new(vec![]);
    assert!(Polynomial::divide(&p, &zero).is_none());
}

#[test]
fn test_divide_numerator_smaller_degree() {
    // deg(num) < deg(den) => quotient = 0, remainder = num
    let num = poly(vec![1, 2]);
    let den = poly(vec![1, 2, 3]);
    let (q, r) = Polynomial::divide(&num, &den).unwrap();
    assert!(q.is_zero());
    assert!(r.equals(&num));
}

#[test]
fn test_divide_exact() {
    // (1 + x)^2 = 1 + 2x + x^2; divide by (1 + x) => (1 + x)
    let dividend = poly(vec![1, 2, 1]);
    let divisor = poly(vec![1, 1]);
    let (q, r) = Polynomial::divide(&dividend, &divisor).unwrap();
    assert!(r.is_zero(), "Remainder should be zero for exact division");
    // Verify: q * divisor == dividend
    let reconstructed = q.mul(&divisor);
    assert!(reconstructed.equals(&dividend));
}

#[test]
fn test_divide_with_remainder() {
    // x^2 divided by (x + 1)
    // x^2 = (x - 1)(x + 1) + 1  => quotient = x-1, remainder = 1
    let dividend = poly(vec![0, 0, 1]); // x^2
    let divisor = poly(vec![1, 1]);     // 1 + x
    let (q, r) = Polynomial::divide(&dividend, &divisor).unwrap();
    // Verify: q * divisor + r == dividend
    let reconstructed = q.mul(&divisor).add(&r);
    assert!(reconstructed.equals(&dividend));
}

#[test]
fn test_div_operator_exact() {
    let dividend = poly(vec![1, 2, 1]);
    let divisor = poly(vec![1, 1]);
    let q = dividend.clone() / divisor.clone();
    let expected = poly(vec![1, 1]);
    assert!(q.equals(&expected));
}

#[test]
fn test_rem_operator() {
    let dividend = poly(vec![1, 2, 1]);
    let divisor = poly(vec![1, 1]);
    let r = dividend % divisor;
    assert!(r.is_zero());
}

// -------------------------------------------------------------------------
// evaluate()
// -------------------------------------------------------------------------

#[test]
fn test_evaluate_constant() {
    // p(x) = 5; p(7) = 5
    let p = poly(vec![5]);
    assert_eq!(p.evaluate(fe(7)), fe(5));
}

#[test]
fn test_evaluate_linear() {
    // p(x) = 2 + 3x; p(4) = 2 + 12 = 14
    let p = poly(vec![2, 3]);
    assert_eq!(p.evaluate(fe(4)), fe(14));
}

#[test]
fn test_evaluate_quadratic() {
    // p(x) = 1 + 2x + x^2; p(3) = 1 + 6 + 9 = 16
    let p = poly(vec![1, 2, 1]);
    assert_eq!(p.evaluate(fe(3)), fe(16));
}

#[test]
fn test_evaluate_at_zero() {
    // p(0) should equal the constant term
    let p = poly(vec![42, 7, 3]);
    assert_eq!(p.evaluate(fe(0)), fe(42));
}

// -------------------------------------------------------------------------
// evaluate_domain()
// -------------------------------------------------------------------------

#[test]
fn test_evaluate_domain_basic() {
    // p(x) = 1 + x; evaluate at [0, 1, 2] => [1, 2, 3]
    let p = poly(vec![1, 1]);
    let domain = vec![fe(0), fe(1), fe(2)];
    let results = p.evaluate_domain(&domain);
    assert_eq!(results, vec![fe(1), fe(2), fe(3)]);
}

#[test]
fn test_evaluate_domain_length_matches() {
    let p = poly(vec![1, 2, 3]);
    let domain = vec![fe(0), fe(1), fe(2), fe(3)];
    let results = p.evaluate_domain(&domain);
    assert_eq!(results.len(), domain.len());
}

// -------------------------------------------------------------------------
// interpolate_domain()
// -------------------------------------------------------------------------

#[test]
fn test_interpolate_single_point() {
    // Single point (2, 5) => constant polynomial 5
    let domain = vec![fe(2)];
    let values = vec![fe(5)];
    let p = Polynomial::interpolate_domain(&domain, &values);
    assert_eq!(p.evaluate(fe(2)), fe(5));
}

#[test]
fn test_interpolate_two_points() {
    // Points (0, 1) and (1, 3) => p(x) = 1 + 2x
    let domain = vec![fe(0), fe(1)];
    let values = vec![fe(1), fe(3)];
    let p = Polynomial::interpolate_domain(&domain, &values);
    assert_eq!(p.evaluate(fe(0)), fe(1));
    assert_eq!(p.evaluate(fe(1)), fe(3));
    assert_eq!(p.degree(), 1);
}

#[test]
fn test_interpolate_recovers_polynomial() {
    // Evaluate a known polynomial on a domain, then interpolate back
    let original = poly(vec![3, 1, 4]); // 3 + x + 4x^2
    let domain = vec![fe(1), fe(2), fe(5)];
    let values = original.evaluate_domain(&domain);
    let recovered = Polynomial::interpolate_domain(&domain, &values);
    assert!(recovered.equals(&original));
}

#[test]
fn test_interpolate_roundtrip_degree() {
    let original = poly(vec![1, 0, 2, 7]); // degree 3
    let domain = vec![fe(1), fe(2), fe(3), fe(4)];
    let values = original.evaluate_domain(&domain);
    let recovered = Polynomial::interpolate_domain(&domain, &values);
    assert_eq!(recovered.degree(), original.degree());
}

// -------------------------------------------------------------------------
// zerofier_domain()
// -------------------------------------------------------------------------

#[test]
fn test_zerofier_vanishes_on_domain() {
    let domain = vec![fe(1), fe(2), fe(3)];
    let z = Polynomial::zerofier_domain(&domain);
    for &d in &domain {
        assert_eq!(z.evaluate(d), fe(0), "Zerofier must evaluate to 0 on all domain points");
    }
}

#[test]
fn test_zerofier_degree_equals_domain_size() {
    let domain = vec![fe(1), fe(2), fe(3), fe(4)];
    let z = Polynomial::zerofier_domain(&domain);
    assert_eq!(z.degree(), domain.len() as i64);
}

#[test]
fn test_zerofier_nonzero_outside_domain() {
    let domain = vec![fe(1), fe(2), fe(3)];
    let z = Polynomial::zerofier_domain(&domain);
    // x=5 is not in the domain, so zerofier should be nonzero there
    assert_ne!(z.evaluate(fe(5)), fe(0));
}

// -------------------------------------------------------------------------
// scale()
// -------------------------------------------------------------------------

#[test]
fn test_scale_by_one() {
    let p = poly(vec![1, 2, 3]);
    let scaled = p.scale(fe(1));
    assert!(scaled.equals(&p), "Scaling by 1 should leave polynomial unchanged");
}

#[test]
fn test_scale_by_factor() {
    // p(x) = 1 + 2x + 3x^2; scale(2) => p(2x) = 1 + 4x + 12x^2
    let p = poly(vec![1, 2, 3]);
    let scaled = p.scale(fe(2));
    // Coefficient at degree k becomes factor^k * original_coeff[k]
    assert_eq!(scaled.evaluate(fe(0)), fe(1));
    // At x=1: 1 + 4*1 + 12*1 = 17
    assert_eq!(scaled.evaluate(fe(1)), fe(17));
}

#[test]
fn test_scale_preserves_degree() {
    let p = poly(vec![1, 2, 3]);
    let scaled = p.scale(fe(5));
    assert_eq!(scaled.degree(), p.degree());
}

// -------------------------------------------------------------------------
// test_collinearity()
// -------------------------------------------------------------------------

#[test]
fn test_collinearity_two_points() {
    // Any two distinct points are always collinear (define a line)
    let points = vec![(fe(0), fe(1)), (fe(1), fe(3))];
    assert!(Polynomial::test_collinearity(&points));
}

#[test]
fn test_collinearity_three_collinear_points() {
    // Points on the line y = 2x + 1: (0,1), (1,3), (2,5)
    let points = vec![(fe(0), fe(1)), (fe(1), fe(3)), (fe(2), fe(5))];
    assert!(Polynomial::test_collinearity(&points));
}

#[test]
fn test_collinearity_three_non_collinear_points() {
    // Points (0,0), (1,1), (2,3) are not collinear
    let points = vec![(fe(0), fe(0)), (fe(1), fe(1)), (fe(2), fe(3))];
    assert!(!Polynomial::test_collinearity(&points));
}

// -------------------------------------------------------------------------
// bitxor (^) exponentiation operator
// -------------------------------------------------------------------------

#[test]
fn test_pow_zero_exponent() {
    // p^0 = 1 (constant polynomial)
    let p = poly(vec![1, 2, 3]);
    let result = p ^ 0;
    let one = poly(vec![1]);
    assert!(result.equals(&one));
}

#[test]
fn test_pow_one_exponent() {
    let p = poly(vec![1, 2, 3]);
    let result = p.clone() ^ 1;
    assert!(result.equals(&p));
}

#[test]
fn test_pow_two() {
    // (1 + x)^2 = 1 + 2x + x^2
    let p = poly(vec![1, 1]);
    let result = p ^ 2;
    let expected = poly(vec![1, 2, 1]);
    assert!(result.equals(&expected));
}

#[test]
fn test_pow_three() {
    // (1 + x)^3 = 1 + 3x + 3x^2 + x^3
    let p = poly(vec![1, 1]);
    let result = p ^ 3;
    let expected = poly(vec![1, 3, 3, 1]);
    assert!(result.equals(&expected));
}

#[test]
fn test_pow_zero_polynomial() {
    // 0^n = 0 for n > 0
    let zero = Polynomial::new(vec![]);
    let result = zero ^ 5;
    assert!(result.is_zero());
}

#[test]
fn test_pow_degree_scales() {
    // deg(p^n) = deg(p) * n
    let p = poly(vec![1, 1]); // degree 1
    let result = p ^ 4;
    assert_eq!(result.degree(), 4);
}

// -------------------------------------------------------------------------
// Field arithmetic edge cases (wrap-around in Baby Bear)
// -------------------------------------------------------------------------

#[test]
fn test_add_field_wraparound() {
    // Adding two large field elements should wrap around modulo P
    let a = poly(vec![BABY_BEAR_P - 1]);
    let b = poly(vec![1]);
    let result = a.add(&b);
    assert!(result.is_zero(), "P-1 + 1 should wrap to 0 in Baby Bear field");
}

#[test]
fn test_mul_field_wraparound() {
    // (P-1) * (P-1) ≡ 1 (mod P) since -1 * -1 = 1
    let a = poly(vec![BABY_BEAR_P - 1]);
    let b = poly(vec![BABY_BEAR_P - 1]);
    let result = a.mul(&b);
    let expected = poly(vec![1]);
    assert!(result.equals(&expected));
}