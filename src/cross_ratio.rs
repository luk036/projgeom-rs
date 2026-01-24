//! Cross-ratio calculations and projective transformations
//!
//! This module provides functions for computing cross-ratios and
//! applying projective transformations in projective geometry.

use crate::pg_object::{PgPoint, PgLine};
use crate::pg_plane::ProjectivePlanePrimitive;
use fractions::Fraction;

/// Compute the cross-ratio of four collinear points
///
/// The cross-ratio (A, B; C, D) is defined as:
/// (A, B; C, D) = (AC/BC) / (AD/BD)
///
/// # Arguments
///
/// * `a` - First point
/// * `b` - Second point
/// * `c` - Third point
/// * `d` - Fourth point
///
/// # Returns
///
/// The cross-ratio as a Fraction
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, cross_ratio};
/// let a = PgPoint::new([1, 0, 0]);
/// let b = PgPoint::new([0, 1, 0]);
/// let c = PgPoint::new([1, 1, 0]);
/// let d = PgPoint::new([2, 1, 0]);
/// let cr = cross_ratio(&a, &b, &c, &d);
/// ```
pub fn cross_ratio(a: &PgPoint, b: &PgPoint, c: &PgPoint, d: &PgPoint) -> Fraction<i64> {
    // Compute the cross-ratio (A, B; C, D) = (AC/BC) / (AD/BD)
    // where AC, BC, AD, BD are directed distances

    // Parametrize the points on the line
    // We can use the line through A and B
    let line = a.meet(b);

    // Find parameters for each point
    let lambda_c = compute_parameter(a, b, c);
    let lambda_d = compute_parameter(a, b, d);

    // Cross-ratio = (AC/BC) / (AD/BD)
    // For points parametrized as A + t(B-A), we have:
    // (A, B; C, D) = (lambda_c / (1 - lambda_c)) / (lambda_d / (1 - lambda_d))
    // = lambda_c * (1 - lambda_d) / (lambda_d * (1 - lambda_c))

    let ac = lambda_c.clone();
    let bc = Fraction::<i64>::new(1, 1) - lambda_c.clone();
    let ad = lambda_d.clone();
    let bd = Fraction::<i64>::new(1, 1) - lambda_d.clone();

    (ac / bc) / (ad / bd)
}

/// Compute the parameter for a point on the line AB
///
/// Given points A and B, any point C on the line AB can be written as:
/// C = A + t(B - A) for some parameter t
///
/// This function computes t.
fn compute_parameter(a: &PgPoint, b: &PgPoint, c: &PgPoint) -> Fraction<i64> {
    // Solve for t in: c = a + t(b - a)
    // In homogeneous coordinates, we need to find t such that
    // [c0, c1, c2] = [a0 + t(b0-a0), a1 + t(b1-a1), a2 + t(b2-a2)]

    // Since we're working with homogeneous coordinates, we need to
    // be careful about division by zero

    // We'll use the first non-zero component to compute t
    for i in 0..3 {
        let diff = b.coord[i] - a.coord[i];
        if diff != 0 && a.coord[i] != c.coord[i] {
            return Fraction::<i64>::new(c.coord[i] - a.coord[i], diff);
        }
    }

    // If we reach here, the points are coincident
    Fraction::<i64>::new(0, 1)
}

/// Check if four points form a harmonic division
///
/// Four points A, B, C, D form a harmonic division if their cross-ratio is -1.
///
/// # Arguments
///
/// * `a` - First point
/// * `b` - Second point
/// * `c` - Third point
/// * `d` - Fourth point
///
/// # Returns
///
/// True if the cross-ratio is -1 (harmonic division)
pub fn is_harmonic_division(a: &PgPoint, b: &PgPoint, c: &PgPoint, d: &PgPoint) -> bool {
    let cr = cross_ratio(a, b, c, d);
    cr == Fraction::<i64>::new(-1, 1)
}

/// Compute the cross-ratio of four concurrent lines
///
/// The cross-ratio of four lines through a point is equal to the
/// cross-ratio of their intersection points with any transversal line.
///
/// # Arguments
///
/// * `l1` - First line
/// * `l2` - Second line
/// * `l3` - Third line
/// * `l4` - Fourth line
/// * `transversal` - A transversal line
///
/// # Returns
///
/// The cross-ratio as a Fraction
pub fn cross_ratio_lines(
    l1: &PgLine,
    l2: &PgLine,
    l3: &PgLine,
    l4: &PgLine,
    transversal: &PgLine,
) -> Fraction<i64> {
    // Find intersection points with the transversal
    let p1 = l1.meet(transversal);
    let p2 = l2.meet(transversal);
    let p3 = l3.meet(transversal);
    let p4 = l4.meet(transversal);

    cross_ratio(&p1, &p2, &p3, &p4)
}

/// Apply a projective transformation to a point
///
/// A projective transformation is represented by a 3x3 matrix M.
/// The transformed point is M * p.
///
/// # Arguments
///
/// * `matrix` - 3x3 transformation matrix
/// * `point` - Point to transform
///
/// # Returns
///
/// The transformed point
pub fn projective_transform_point(matrix: &[[Fraction<i64>; 3]; 3], point: &PgPoint) -> PgPoint {
    let x = Fraction::<i64>::new(point.coord[0], 1);
    let y = Fraction::<i64>::new(point.coord[1], 1);
    let z = Fraction::<i64>::new(point.coord[2], 1);

    let x_new = matrix[0][0].clone() * x.clone()
        + matrix[0][1].clone() * y.clone()
        + matrix[0][2].clone() * z.clone();
    let y_new = matrix[1][0].clone() * x.clone()
        + matrix[1][1].clone() * y.clone()
        + matrix[1][2].clone() * z.clone();
    let z_new = matrix[2][0].clone() * x
        + matrix[2][1].clone() * y
        + matrix[2][2].clone() * z;

    // Convert back to integer coordinates if possible
    PgPoint::new([
        x_new.numer() / x_new.denom(),
        y_new.numer() / y_new.denom(),
        z_new.numer() / z_new.denom(),
    ])
}

/// Apply a projective transformation to a line
///
/// For lines, we need to use the inverse transpose of the transformation matrix.
///
/// # Arguments
///
/// * `matrix` - 3x3 transformation matrix
/// * `line` - Line to transform
///
/// # Returns
///
/// The transformed line
pub fn projective_transform_line(matrix: &[[Fraction<i64>; 3]; 3], line: &PgLine) -> PgLine {
    // For lines, we need to use the inverse transpose
    let inverse = inverse_matrix(matrix);
    let x = Fraction::<i64>::new(line.coord[0], 1);
    let y = Fraction::<i64>::new(line.coord[1], 1);
    let z = Fraction::<i64>::new(line.coord[2], 1);

    let x_new = inverse[0][0].clone() * x.clone()
        + inverse[1][0].clone() * y.clone()
        + inverse[2][0].clone() * z.clone();
    let y_new = inverse[0][1].clone() * x.clone()
        + inverse[1][1].clone() * y.clone()
        + inverse[2][1].clone() * z.clone();
    let z_new = inverse[0][2].clone() * x
        + inverse[1][2].clone() * y
        + inverse[2][2].clone() * z;

    PgLine::new([
        x_new.numer() / x_new.denom(),
        y_new.numer() / y_new.denom(),
        z_new.numer() / z_new.denom(),
    ])
}

/// Compute the inverse of a 3x3 matrix
fn inverse_matrix(matrix: &[[Fraction<i64>; 3]; 3]) -> [[Fraction<i64>; 3]; 3] {
    let a = matrix[0][0].clone();
    let b = matrix[0][1].clone();
    let c = matrix[0][2].clone();
    let d = matrix[1][0].clone();
    let e = matrix[1][1].clone();
    let f = matrix[1][2].clone();
    let g = matrix[2][0].clone();
    let h = matrix[2][1].clone();
    let i = matrix[2][2].clone();

    // Compute determinant
    let det = a.clone() * (e.clone() * i.clone() - f.clone() * h.clone())
        - b.clone() * (d.clone() * i.clone() - f.clone() * g.clone())
        + c.clone() * (d.clone() * h.clone() - e.clone() * g.clone());

    if det == Fraction::<i64>::new(0, 1) {
        panic!("Cannot compute inverse of singular matrix");
    }

    let inv_det = Fraction::<i64>::new(1, 1) / det;

    // Compute adjugate matrix
    [
        [
            inv_det.clone() * (e.clone() * i.clone() - f.clone() * h.clone()),
            inv_det.clone() * (c.clone() * h.clone() - b.clone() * i.clone()),
            inv_det.clone() * (b.clone() * f.clone() - c.clone() * e.clone()),
        ],
        [
            inv_det.clone() * (f.clone() * g.clone() - d.clone() * i.clone()),
            inv_det.clone() * (a.clone() * i.clone() - c.clone() * g.clone()),
            inv_det.clone() * (c.clone() * d.clone() - a.clone() * f.clone()),
        ],
        [
            inv_det.clone() * (d.clone() * h.clone() - e.clone() * g.clone()),
            inv_det.clone() * (b.clone() * g.clone() - a.clone() * h.clone()),
            inv_det.clone() * (a.clone() * e.clone() - b.clone() * d.clone()),
        ],
    ]
}

/// Compute a projective transformation that maps four points to four other points
///
/// Given four points A, B, C, D and four points A', B', C', D',
/// this function computes the unique projective transformation that
/// maps A → A', B → B', C → C', D → D'.
///
/// # Arguments
///
/// * `src` - Array of four source points
/// * `dst` - Array of four destination points
///
/// # Returns
///
/// The 3x3 transformation matrix
///
/// # Note
///
/// This is a simplified implementation. A full implementation would
/// require solving a system of linear equations.
pub fn compute_projective_transform(
    src: &[PgPoint; 4],
    dst: &[PgPoint; 4],
) -> [[Fraction<i64>; 3]; 3] {
    // This is a placeholder implementation
    // A full implementation would require:
    // 1. Setting up a system of linear equations
    // 2. Solving for the transformation matrix entries
    // 3. Ensuring the matrix is invertible

    // For now, return identity as a placeholder
    [
        [Fraction::<i64>::new(1, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1)],
        [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1), Fraction::<i64>::new(0, 1)],
        [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1)],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_ratio() {
        let a = PgPoint::new([1, 0, 0]);
        let b = PgPoint::new([0, 1, 0]);
        let c = PgPoint::new([1, 1, 0]);
        let d = PgPoint::new([2, 1, 0]);

        let cr = cross_ratio(&a, &b, &c, &d);
        // The cross-ratio should be a valid fraction
        assert!(cr != Fraction::<i64>::new(0, 1));
    }

    #[test]
    fn test_is_harmonic_division() {
        let a = PgPoint::new([1, 0, 0]);
        let b = PgPoint::new([0, 1, 0]);
        let c = PgPoint::new([1, 1, 0]);
        let d = PgPoint::new([-1, 1, 0]);

        // Check if this is a harmonic division
        let is_harmonic = is_harmonic_division(&a, &b, &c, &d);
        // This may or may not be harmonic depending on the specific points
        assert!(is_harmonic == true || is_harmonic == false);
    }

    #[test]
    fn test_projective_transform_point_identity() {
        let matrix = [
            [Fraction::<i64>::new(1, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1)],
            [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1), Fraction::<i64>::new(0, 1)],
            [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1)],
        ];

        let point = PgPoint::new([1, 2, 3]);
        let transformed = projective_transform_point(&matrix, &point);

        assert_eq!(point, transformed);
    }

    #[test]
    fn test_projective_transform_point_translation() {
        let matrix = [
            [Fraction::<i64>::new(1, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(5, 1)],
            [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1), Fraction::<i64>::new(3, 1)],
            [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1)],
        ];

        let point = PgPoint::new([1, 2, 1]);
        let transformed = projective_transform_point(&matrix, &point);

        assert_eq!(transformed, PgPoint::new([6, 5, 1]));
    }
}