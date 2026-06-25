//! Euclidean plane measurement functions
//!
//! This module provides quadrance, spread, and cross spread computations
//! for Euclidean geometry, ported from the C++ euclid_plane_measure.hpp.

use crate::pg_object::{dot1, sq, EuclidPoint};
use fractions::Fraction;

/// Compute the squared difference of ratios (integral version).
///
/// Computes ((x1/z1) - (x2/z2))^2 using Fraction arithmetic.
///
/// # Examples
///
/// ```
/// use projgeom_rs::euclid_plane_measure::quad1;
/// use fractions::Fraction;
///
/// let result = quad1(0, 1, 3, 1);
/// assert_eq!(result, Fraction::new(9, 1));
/// ```
#[inline]
pub fn quad1(x1: i64, z1: i64, x2: i64, z2: i64) -> Fraction<i64> {
    let diff = Fraction::new(x1, z1) - Fraction::new(x2, z2);
    diff * diff
}

/// Compute the quadrance (squared distance) between two Euclidean points.
///
/// The quadrance is Q = ((x1/z1) - (x2/z2))^2 + ((y1/z1) - (y2/z2))^2
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, euclid_plane_measure::quadrance};
/// use fractions::Fraction;
///
/// let p1 = EuclidPoint::new([0, 0, 1]);
/// let p2 = EuclidPoint::new([3, 4, 1]);
/// let q = quadrance(&p1, &p2);
/// assert_eq!(q, Fraction::new(25, 1)); // 3^2 + 4^2 = 25
/// ```
#[inline]
pub fn quadrance(a: &EuclidPoint, b: &EuclidPoint) -> Fraction<i64> {
    quad1(a.coord[0], a.coord[2], b.coord[0], b.coord[2])
        + quad1(a.coord[1], a.coord[2], b.coord[1], b.coord[2])
}

/// Compute the base of the spread formula.
///
/// Returns d^2 / (dot1(l1,l1) * dot1(l2,l2)) for integral types.
#[inline]
pub fn sbase(l1: &EuclidPoint, l2: &EuclidPoint, d: i64) -> Fraction<i64> {
    let d_sq = Fraction::new(sq(d), 1);
    let denom = Fraction::new(dot1(&l1.coord, &l2.coord), 1); // placeholder
    d_sq / denom
}

/// Compute the spread (squared sine of angle) between two Euclidean lines.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, euclid_plane_measure::spread};
/// use fractions::Fraction;
///
/// // Lines x=0 and y=0 are perpendicular, so spread = 1
/// let l1 = EuclidPoint::new([1, 0, 0]);
/// let l2 = EuclidPoint::new([0, 1, 0]);
/// let s = spread(&l1, &l2);
/// assert_eq!(s, Fraction::new(1, 1));
/// ```
#[inline]
pub fn spread(l1: &EuclidPoint, l2: &EuclidPoint) -> Fraction<i64> {
    let d = crate::pg_object::cross2_3(&l1.coord, &l2.coord);
    let d_sq = sq(d);
    let denom = dot1(&l1.coord, &l1.coord) * dot1(&l2.coord, &l2.coord);
    Fraction::new(d_sq, denom)
}

/// Compute the cross spread (squared cosine of angle) between two Euclidean lines.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, euclid_plane_measure::cross_s};
/// use fractions::Fraction;
///
/// let l1 = EuclidPoint::new([1, 0, 0]);
/// let l2 = EuclidPoint::new([0, 1, 0]);
/// let cs = cross_s(&l1, &l2);
/// assert_eq!(cs, Fraction::new(0, 1));
/// ```
#[inline]
pub fn cross_s(l1: &EuclidPoint, l2: &EuclidPoint) -> Fraction<i64> {
    let d = dot1(&l1.coord, &l2.coord);
    let d_sq = sq(d);
    let denom = dot1(&l1.coord, &l1.coord) * dot1(&l2.coord, &l2.coord);
    Fraction::new(d_sq, denom)
}

/// Compute the quadrances of all three sides of a triangle.
///
/// Returns [Q(b,c), Q(a,c), Q(a,b)].
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, euclid_plane_measure::tri_quadrance};
///
/// let a = EuclidPoint::new([0, 0, 1]);
/// let b = EuclidPoint::new([3, 0, 1]);
/// let c = EuclidPoint::new([0, 4, 1]);
/// let q = tri_quadrance(&[a, b, c]);
/// assert_eq!(q[0], fractions::Fraction::new(25, 1)); // 3^2 + 4^2 = 25
/// ```
#[inline]
pub fn tri_quadrance(triangle: &[EuclidPoint; 3]) -> [Fraction<i64>; 3] {
    let [a1, a2, a3] = triangle;
    [quadrance(a2, a3), quadrance(a1, a3), quadrance(a1, a2)]
}

/// Compute the spreads of all three angles of a triangle.
///
/// Returns [spread(side1, side2), spread(side1, side3), spread(side2, side3)].
#[inline]
pub fn tri_spread(trilateral: &[EuclidPoint; 3]) -> [Fraction<i64>; 3] {
    let [l1, l2, l3] = trilateral;
    [spread(l2, l3), spread(l1, l3), spread(l1, l2)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pg_object::EuclidPoint;

    #[test]
    fn test_quadrance_basic() {
        let p1 = EuclidPoint::new([0, 0, 1]);
        let p2 = EuclidPoint::new([3, 4, 1]);
        assert_eq!(quadrance(&p1, &p2), Fraction::new(25, 1));
    }

    #[test]
    fn test_quadrance_origin() {
        let p1 = EuclidPoint::new([1, 2, 1]);
        let p2 = EuclidPoint::new([1, 2, 1]);
        assert_eq!(quadrance(&p1, &p2), Fraction::new(0, 1));
    }

    #[test]
    fn test_spread_perpendicular() {
        let l1 = EuclidPoint::new([1, 0, 0]);
        let l2 = EuclidPoint::new([0, 1, 0]);
        let s = spread(&l1, &l2);
        assert_eq!(s, Fraction::new(1, 1));
    }

    #[test]
    fn test_spread_parallel() {
        let l1 = EuclidPoint::new([1, 0, 0]);
        let l2 = EuclidPoint::new([2, 0, 0]);
        let s = spread(&l1, &l2);
        assert_eq!(s, Fraction::new(0, 1));
    }

    #[test]
    fn test_cross_s_perpendicular() {
        let l1 = EuclidPoint::new([1, 0, 0]);
        let l2 = EuclidPoint::new([0, 1, 0]);
        let cs = cross_s(&l1, &l2);
        assert_eq!(cs, Fraction::new(0, 1));
    }

    #[test]
    fn test_cross_s_parallel() {
        let l1 = EuclidPoint::new([1, 0, 0]);
        let l2 = EuclidPoint::new([2, 0, 0]);
        let cs = cross_s(&l1, &l2);
        assert_eq!(cs, Fraction::new(1, 1));
    }

    #[test]
    fn test_tri_quadrance() {
        let a = EuclidPoint::new([0, 0, 1]);
        let b = EuclidPoint::new([3, 0, 1]);
        let c = EuclidPoint::new([0, 4, 1]);
        let q = tri_quadrance(&[a, b, c]);
        assert_eq!(q[0], Fraction::new(25, 1));
    }

    #[test]
    fn test_quad1_zero() {
        let result = quad1(5, 1, 5, 1);
        assert_eq!(result, Fraction::new(0, 1));
    }
}
