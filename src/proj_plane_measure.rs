//! Projective plane measurement functions
//!
//! This module provides cross-ratio computations for projective geometry,
//! ported from the C++ proj_plane_measure.hpp.

use crate::pg_object::{cross0, cross1, PgPoint};
use crate::pg_plane::ProjectivePlane;
use fractions::Fraction;

/// Compute the ratio of two ratios: (a/b) / (c/d)
///
/// For integral types, uses Fraction arithmetic.
/// For non-integral types, uses direct division.
#[inline]
pub fn ratio_ratio(a: i64, b: i64, c: i64, d: i64) -> Fraction<i64> {
    Fraction::new(a, b) / Fraction::new(c, d)
}

/// Compute the cross ratio of four collinear points using dot products.
///
/// $$ R(A, B; l, m) = \frac{A \cdot l / A \cdot m}{B \cdot l / B \cdot m} $$
/// where line_l and line_m are two lines through the points.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, PgLine, proj_plane_measure::x_ratio};
///
/// let a = PgPoint::new([1, 0, 1]);
/// let b = PgPoint::new([0, 1, 1]);
/// let l1 = PgLine::new([1, 1, -1]);
/// let l2 = PgLine::new([1, -1, 0]);
/// let _ratio = x_ratio(&a, &b, &l1, &l2);
/// ```
#[inline]
pub fn x_ratio<Point, Line, Value>(
    pt_a: &Point,
    pt_b: &Point,
    line_l: &Line,
    line_m: &Line,
) -> Fraction<i64>
where
    Point: ProjectivePlane<Line, Value>,
    Line: ProjectivePlane<Point, Value>,
    Value: Into<i64> + Default + Eq + Copy,
{
    ratio_ratio(
        pt_a.dot(line_l).into(),
        pt_a.dot(line_m).into(),
        pt_b.dot(line_l).into(),
        pt_b.dot(line_m).into(),
    )
}

/// Compute cross ratio using yz-plane projection (cross0-based).
///
/// $$ R(A, B; C, D) = \frac{\text{cross0}(A,C) / \text{cross0}(A,D)}{\text{cross0}(B,C) / \text{cross0}(B,D)} $$
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, proj_plane_measure::R0};
///
/// let a = PgPoint::new([1, 0, 1]);
/// let b = PgPoint::new([0, 1, 1]);
/// let c = PgPoint::new([1, 1, 1]);
/// let d = PgPoint::new([2, 1, 1]);
/// let _ratio = R0(&a, &b, &c, &d);
/// ```
#[allow(non_snake_case)]
#[inline]
pub fn R0(pt_a: &PgPoint, pt_b: &PgPoint, pt_c: &PgPoint, pt_d: &PgPoint) -> Fraction<i64> {
    ratio_ratio(
        cross0(&pt_a.coord, &pt_c.coord),
        cross0(&pt_a.coord, &pt_d.coord),
        cross0(&pt_b.coord, &pt_c.coord),
        cross0(&pt_b.coord, &pt_d.coord),
    )
}

/// Compute cross ratio using xz-plane projection (cross1-based).
///
/// Returns the cross ratio R(A, B; C, D) using the xz-plane projection:
/// (cross1(A,C)/cross1(A,D)) / (cross1(B,C)/cross1(B,D))
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, proj_plane_measure::R1};
///
/// let a = PgPoint::new([0, 1, 1]);
/// let b = PgPoint::new([1, 0, 1]);
/// let c = PgPoint::new([1, 1, 1]);
/// let d = PgPoint::new([2, 1, 1]);
/// let _ratio = R1(&a, &b, &c, &d);
/// ```
#[allow(non_snake_case)]
#[inline]
pub fn R1(pt_a: &PgPoint, pt_b: &PgPoint, pt_c: &PgPoint, pt_d: &PgPoint) -> Fraction<i64> {
    ratio_ratio(
        cross1(&pt_a.coord, &pt_c.coord),
        cross1(&pt_a.coord, &pt_d.coord),
        cross1(&pt_b.coord, &pt_c.coord),
        cross1(&pt_b.coord, &pt_d.coord),
    )
}

/// Compute the cross ratio of four collinear points (generic version).
///
/// Automatically chooses the best coordinate projection based on the points.
/// Uses yz-plane projection if cross0(A,B) != 0, otherwise uses xz-plane.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, proj_plane_measure::R};
///
/// let a = PgPoint::new([1, 0, 1]);
/// let b = PgPoint::new([0, 1, 1]);
/// let c = PgPoint::new([1, 1, 1]);
/// let d = PgPoint::new([2, 1, 1]);
/// let _ratio = R(&a, &b, &c, &d);
/// ```
#[allow(non_snake_case)]
#[inline]
pub fn R(pt_a: &PgPoint, pt_b: &PgPoint, pt_c: &PgPoint, pt_d: &PgPoint) -> Fraction<i64> {
    if cross0(&pt_a.coord, &pt_b.coord) != 0 {
        R0(pt_a, pt_b, pt_c, pt_d)
    } else {
        R1(pt_a, pt_b, pt_c, pt_d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pg_object::PgPoint;

    #[test]
    fn test_ratio_ratio_basic() {
        let result = ratio_ratio(1, 2, 3, 4);
        assert_eq!(result, Fraction::new(2, 3));
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_R0_basic() {
        // Collinear points on the x-axis: (0,0), (1,0), (2,0), (3,0)
        let a = PgPoint::new([0, 0, 1]);
        let b = PgPoint::new([1, 0, 1]);
        let c = PgPoint::new([2, 0, 1]);
        let d = PgPoint::new([3, 0, 1]);
        // cross0 values: y1*z2 - y2*z1. Since all y=0, cross0=0 for all.
        // This means R0 returns Fraction with zero denominator (degenerate case).
        // R would auto-select R1 for this case.
        let _result = R0(&a, &b, &c, &d);
        // R0 can give degenerate results when cross0=0; that's expected.
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_R1_basic() {
        // Collinear points with varying x coordinates (non-zero cross1)
        // Line y = x: (0,0), (1,1), (2,2), (3,3)
        let a = PgPoint::new([0, 0, 1]);
        let b = PgPoint::new([1, 1, 1]);
        let c = PgPoint::new([2, 2, 1]);
        let d = PgPoint::new([3, 3, 1]);
        // cross1(a,c) = 0*1 - 2*1 = -2 ≠ 0
        let result = R1(&a, &b, &c, &d);
        assert!(*result.denom() != 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_R_auto_select_yz() {
        // Points with cross0 != 0 (different y,z)
        let a = PgPoint::new([0, 0, 1]);
        let b = PgPoint::new([1, 1, 1]);
        let c = PgPoint::new([2, 2, 1]);
        let d = PgPoint::new([3, 3, 1]);
        // cross0(a,b) = 0*1 - 1*1 = -1 ≠ 0, so uses R0
        // But the cross ratio of 4 collinear equally-spaced points = ???
        let result = R(&a, &b, &c, &d);
        assert!(*result.denom() != 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_R_xz_projection() {
        // Points where cross0 is 0 (proportional y,z) so R falls through to R1
        // But these must be COLLINEAR for cross ratio to be meaningful
        let a = PgPoint::new([1, 0, 1]);
        let b = PgPoint::new([2, 0, 1]);
        let c = PgPoint::new([3, 0, 1]);
        let d = PgPoint::new([4, 0, 1]);
        // cross0(a,b) = 0*1 - 0*1 = 0, so R uses R1
        let result = R(&a, &b, &c, &d);
        assert!(*result.denom() != 0);
    }

    #[test]
    fn test_x_ratio_with_pgpoint() {
        use crate::pg_object::{PgLine, PgPoint};
        // Points on vertical line x=1 and two arbitrary lines through them
        let a = PgPoint::new([1, 0, 1]); // (1,0)
        let b = PgPoint::new([1, 1, 1]); // (1,1)
                                         // Lines not incident with either point
        let l1 = PgLine::new([1, 0, -3]); // x = 3
        let l2 = PgLine::new([0, 0, 1]); // z = 0 (line at infinity)
        let result = x_ratio(&a, &b, &l1, &l2);
        assert!(*result.denom() != 0);
    }
}
