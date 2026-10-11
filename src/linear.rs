//! Exact 3x3 linear-algebra helpers over the rationals.
//!
//! These helpers let the conic and transform modules perform exact matrix
//! operations (inverse, adjugate, products) and convert rational homogeneous
//! coordinates to integers, without any floating-point arithmetic.

use fractions::Fraction;

/// A 3x3 matrix of rational numbers.
pub(crate) type Mat3 = [[Fraction<i64>; 3]; 3];

/// A 3-vector of rational numbers.
pub(crate) type Vec3 = [Fraction<i64>; 3];

fn gcd_i64(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm_i64(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd_i64(a, b)).abs() * b.abs()
    }
}

/// Convert exact rational homogeneous coordinates to integers.
///
/// Homogeneous coordinates are defined only up to a non-zero scale, so a triple
/// of fractions is equivalent to the integer triple obtained by clearing
/// denominators (least common multiple) and dividing through by the greatest
/// common divisor. The sign is normalized so that the first non-zero coordinate
/// is positive. Unlike integer division, this conversion is exact.
pub(crate) fn rationalize(values: &[Fraction<i64>; 3]) -> [i64; 3] {
    let mut denom: i64 = 1;
    for value in values {
        denom = lcm_i64(denom, *value.denom());
    }

    let mut ints = [0_i64; 3];
    for (slot, value) in ints.iter_mut().zip(values.iter()) {
        *slot = *value.numer() * (denom / *value.denom());
    }

    let mut divisor: i64 = 0;
    for value in ints {
        divisor = gcd_i64(divisor, value);
    }
    if divisor > 1 {
        for value in &mut ints {
            *value /= divisor;
        }
    }

    for value in ints {
        if value < 0 {
            for slot in &mut ints {
                *slot = -*slot;
            }
            break;
        }
        if value > 0 {
            break;
        }
    }
    ints
}

/// Determinant of a 3x3 matrix.
pub(crate) fn det3(m: &Mat3) -> Fraction<i64> {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// Adjugate (classical adjoint) of a 3x3 matrix.
///
/// Satisfies `m * adjugate3(m) == det3(m) * I`, so it is the numerator of the
/// inverse and is well defined even for singular matrices.
pub(crate) fn adjugate3(m: &Mat3) -> Mat3 {
    [
        [
            m[1][1] * m[2][2] - m[1][2] * m[2][1],
            m[0][2] * m[2][1] - m[0][1] * m[2][2],
            m[0][1] * m[1][2] - m[0][2] * m[1][1],
        ],
        [
            m[1][2] * m[2][0] - m[1][0] * m[2][2],
            m[0][0] * m[2][2] - m[0][2] * m[2][0],
            m[0][2] * m[1][0] - m[0][0] * m[1][2],
        ],
        [
            m[1][0] * m[2][1] - m[1][1] * m[2][0],
            m[0][1] * m[2][0] - m[0][0] * m[2][1],
            m[0][0] * m[1][1] - m[0][1] * m[1][0],
        ],
    ]
}

/// Inverse of a 3x3 matrix (adjugate divided by the determinant).
///
/// # Panics
///
/// Panics if the matrix is singular.
pub(crate) fn inverse3(m: &Mat3) -> Mat3 {
    let det = det3(m);
    if det == Fraction::new(0, 1) {
        panic!("Cannot invert singular matrix");
    }
    let inv_det = Fraction::new(1, 1) / det;
    let adj = adjugate3(m);
    std::array::from_fn(|i| std::array::from_fn(|j| adj[i][j] * inv_det))
}

/// Product of two 3x3 matrices.
pub(crate) fn matmul3(a: &Mat3, b: &Mat3) -> Mat3 {
    std::array::from_fn(|i| {
        std::array::from_fn(|j| a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j])
    })
}

/// Product of a 3x3 matrix and a 3-vector.
pub(crate) fn matvec3(m: &Mat3, v: &Vec3) -> Vec3 {
    std::array::from_fn(|i| m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2])
}
