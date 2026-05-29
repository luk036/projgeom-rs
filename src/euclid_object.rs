// Euclidean Geometry

use crate::pg_object::{sq, EuclidLine, EuclidPoint};
use crate::pg_plane::{
    coincident, tri_dual, Involution, ProjectivePlane, ProjectivePlanePrimitive,
};
use crate::{CayleyKleinPlane, CayleyKleinPlanePrimitive};
// use crate::pg_object::{plucker_operation, dot_product};
use crate::pg_object::dot1;

/// Line at infinity in Euclidean geometry.
static L_INF: EuclidLine = EuclidLine { coord: [0, 0, 1] };

impl_cayley_klein_plane!(
    EuclidPoint,
    EuclidLine,
    |_p: &EuclidPoint| L_INF.clone(),
    |l: &EuclidLine| EuclidPoint::new([l.coord[0], l.coord[1], 0])
);

impl EuclidLine {
    /// The function checks if two EuclidLine objects are parallel.
    ///
    /// Arguments:
    ///
    /// * `other`: `other` is a reference to an `EuclidLine` object.
    ///
    /// Returns:
    ///
    /// The `is_parallel` function returns a boolean value indicating whether the current `EuclidLine`
    /// object is parallel to the `other` `EuclidLine` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use projgeom_rs::EuclidLine;
    ///
    /// let l1 = EuclidLine::new([1, 0, -1]); // x = 1
    /// let l2 = EuclidLine::new([2, 0, -5]); // x = 2.5 (parallel to l1)
    /// assert!(l1.is_parallel(&l2));
    ///
    /// let l3 = EuclidLine::new([0, 1, -1]); // y = 1 (not parallel to l1)
    /// assert!(!l1.is_parallel(&l3));
    /// ```
    #[inline]
    pub const fn is_parallel(&self, other: &EuclidLine) -> bool {
        self.coord[0] * other.coord[1] == self.coord[1] * other.coord[0]
    }

    /// The function checks if two Euclidean lines are perpendicular.
    ///
    /// Arguments:
    ///
    /// * `other`: `other` is a reference to an `EuclidLine` object.
    ///
    /// Returns:
    ///
    /// a boolean value, indicating whether the two lines are perpendicular to each other.
    ///
    /// # Examples
    ///
    /// ```
    /// use projgeom_rs::EuclidLine;
    ///
    /// let l1 = EuclidLine::new([1, 0, -1]); // x = 1
    /// let l2 = EuclidLine::new([0, 1, -1]); // y = 1
    /// assert!(l1.is_perpendicular(&l2));
    ///
    /// let l3 = EuclidLine::new([1, 1, -1]); // x + y = 1
    /// assert!(!l1.is_perpendicular(&l3));
    /// ```
    #[inline]
    pub const fn is_perpendicular(&self, other: &EuclidLine) -> bool {
        dot1(&self.coord, &other.coord) == 0
    }

    /// The `altitude` function calculates the perpendicular line from a given point to a line.
    ///
    /// Arguments:
    ///
    /// * `pt_a`: The parameter `pt_a` is of type `EuclidPoint`.
    ///
    /// Returns:
    ///
    /// The `altitude` function returns an `EuclidLine` object.
    /// Note: This function uses `perp()` which returns the polar line of the current line,
    /// and then finds the intersection with the given point to create the altitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use projgeom_rs::{EuclidPoint, EuclidLine};
    ///
    /// let p = EuclidPoint::new([1, 2, 1]); // Point (1,2)
    /// let l = EuclidLine::new([1, 0, -1]); // Line x = 1
    /// let alt = l.altitude(&p);
    /// // The altitude from (1,2) to x=1 is the line y=2, which is (0,1,-2) in homogeneous coordinates.
    /// assert_eq!(alt, EuclidLine::new([0, 1, -2]));
    /// ```
    #[inline]
    pub fn altitude(&self, pt_a: &EuclidPoint) -> EuclidLine {
        self.perp().meet(pt_a)
    }
}

impl EuclidPoint {
    /// The `midpoint` function calculates the midpoint between two EuclidPoint objects.
    ///
    /// Arguments:
    ///
    /// * `other`: `other` is a reference to an `EuclidPoint` object.
    ///
    /// Returns:
    ///
    /// The `midpoint` function returns an instance of the `EuclidPoint` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use projgeom_rs::EuclidPoint;
    ///
    /// let p1 = EuclidPoint::new([0, 0, 1]);
    /// let p2 = EuclidPoint::new([2, 4, 1]);
    /// let mid = p1.midpoint(&p2);
    /// // Midpoint of (0,0) and (2,4) is (1,2)
    /// assert_eq!(mid, EuclidPoint::new([1, 2, 1]));
    /// ```
    #[inline]
    pub fn midpoint(&self, other: &EuclidPoint) -> EuclidPoint {
        EuclidPoint::parametrize(self, other.coord[2], other, self.coord[2])
    }
}

/// The `tri_altitude` function calculates the altitudes of a triangle given its three vertices.
///
/// Arguments:
///
/// * `triangle`: The `triangle` parameter is an array of `EuclidPoint` structs representing the three vertices of
///   a triangle.
///
/// Returns:
///
/// The function `tri_altitude` returns an array of `EuclidLine` objects, specifically `[t_1, t_2, t_3]`.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, EuclidLine, tri_altitude};
///
/// let p1 = EuclidPoint::new([0, 0, 1]);
/// let p2 = EuclidPoint::new([2, 0, 1]);
/// let p3 = EuclidPoint::new([1, 3, 1]);
/// let triangle = [p1, p2, p3];
/// let altitudes = tri_altitude(&triangle);
/// // For p1(0,0), opposite side is line p2p3 (EuclidLine::new(3, -1, -6)). Perpendicular through (0,0) is (1,3,0).
/// assert_eq!(altitudes[0], EuclidLine::new([-1, 3, 0]));
/// // For p2(2,0), opposite side is line p1p3 (EuclidLine::new(3, -1, 0)). Perpendicular through (2,0) is (1,3,-2).
/// assert_eq!(altitudes[1], EuclidLine::new([1, 3, -2]));
/// // For p3(1,3), opposite side is line p1p2 (EuclidLine::new(0, 1, 0)). Perpendicular through (1,3) is (1,0,-1).
/// assert_eq!(altitudes[2], EuclidLine::new([2, 0, -2]));
/// ```
#[allow(dead_code)]
pub fn tri_altitude(triangle: &[EuclidPoint; 3]) -> [EuclidLine; 3] {
    let [l_1, l_2, l_3] = tri_dual(triangle);
    let [a_1, a_2, a_3] = triangle;
    assert!(!coincident(a_1, a_2, a_3));
    let t_1 = l_1.altitude(a_1);
    let t_2 = l_2.altitude(a_2);
    let t_3 = l_3.altitude(a_3);
    [t_1, t_2, t_3]
}

/// Convert a line to its direction vector in the affine plane.
///
/// Extracts the direction components (first two coordinates) of a line,
/// setting z=0 to represent a direction vector.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidLine, EuclidPoint, euclid_object::fB};
///
/// let l = EuclidLine::new([1, 2, -3]);
/// let dir = fB(&l);
/// assert_eq!(dir, EuclidPoint::new([1, 2, 0]));
/// ```
#[allow(non_snake_case)]
#[inline]
pub fn fB(line_l: &EuclidLine) -> EuclidPoint {
    EuclidPoint::new([line_l.coord[0], line_l.coord[1], 0])
}

/// Check if two Euclidean lines are perpendicular (free function).
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidLine, is_perpendicular};
///
/// let l1 = EuclidLine::new([1, 0, -1]); // x = 1
/// let l2 = EuclidLine::new([0, 1, -1]); // y = 1
/// assert!(is_perpendicular(&l1, &l2));
/// ```
#[inline]
pub fn is_perpendicular(line_l: &EuclidLine, line_m: &EuclidLine) -> bool {
    line_l.is_perpendicular(line_m)
}

/// Check if two Euclidean lines are parallel (free function).
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidLine, euclid_object::is_parallel};
///
/// let l1 = EuclidLine::new([1, 0, -1]); // x = 1
/// let l2 = EuclidLine::new([2, 0, -5]); // x = 2.5
/// assert!(is_parallel(&l1, &l2));
/// ```
#[inline]
pub fn is_parallel(line_l: &EuclidLine, line_m: &EuclidLine) -> bool {
    line_l.is_parallel(line_m)
}

/// Compute the midpoint of two Euclidean points (free function).
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, midpoint};
///
/// let a = EuclidPoint::new([0, 0, 1]);
/// let b = EuclidPoint::new([2, 4, 1]);
/// let mid = midpoint(&a, &b);
/// assert_eq!(mid, EuclidPoint::new([1, 2, 1]));
/// ```
#[inline]
pub fn midpoint(a: &EuclidPoint, b: &EuclidPoint) -> EuclidPoint {
    a.midpoint(b)
}

/// Compute the midpoints of all three sides of a triangle.
///
/// Returns [midpoint(a1,a2), midpoint(a2,a3), midpoint(a1,a3)].
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, tri_midpoint};
///
/// let a = EuclidPoint::new([0, 0, 1]);
/// let b = EuclidPoint::new([2, 0, 1]);
/// let c = EuclidPoint::new([0, 2, 1]);
/// let mids = tri_midpoint(&[a, b, c]);
/// // Midpoint of (0,0)-(2,0) is (1,0)
/// assert_eq!(mids[0], EuclidPoint::new([1, 0, 1]));
/// // Midpoint of (2,0)-(0,2) is (1,1)
/// assert_eq!(mids[1], EuclidPoint::new([1, 1, 1]));
/// // Midpoint of (0,0)-(0,2) is (0,1)
/// assert_eq!(mids[2], EuclidPoint::new([0, 1, 1]));
/// ```
#[inline]
pub fn tri_midpoint(triangle: &[EuclidPoint; 3]) -> [EuclidPoint; 3] {
    let [a1, a2, a3] = triangle;
    [midpoint(a1, a2), midpoint(a2, a3), midpoint(a1, a3)]
}

/// Compute a point on the unit circle from trigonometric parameters.
///
/// Creates a point on the unit circle using the parameterization
/// (lambda^2 - mu^2, 2*lambda*mu, lambda^2 + mu^2).
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, uc_point};
///
/// let p = uc_point(1, 0);
/// assert_eq!(p, EuclidPoint::new([1, 0, 1])); // (1,0) on unit circle
///
/// let p2 = uc_point(0, 1);
/// assert_eq!(p2, EuclidPoint::new([-1, 0, 1])); // (-1,0) on unit circle
/// ```
#[inline]
pub fn uc_point(lambda: i64, mu: i64) -> EuclidPoint {
    let lambda_sq = sq(lambda);
    let mu_sq = sq(mu);
    EuclidPoint::new([lambda_sq - mu_sq, 2 * lambda * mu, lambda_sq + mu_sq])
}

/// Archimedes's function: 4*a*b - sq(a + b - c)
#[inline]
pub fn archimedes(a: i64, b: i64, c: i64) -> i64 {
    4 * a * b - sq(a + b - c)
}

/// Cyclic quadrilateral quadrea theorem.
///
/// Returns [line_m, point_p] - the two terms of the cyclic quadrilateral equation.
#[inline]
pub fn cqq(a: i64, b: i64, c: i64, d: i64) -> [i64; 2] {
    let t1 = 4 * a * b;
    let t2 = 4 * c * d;
    let line_m = (t1 + t2) - sq(a + b - c - d);
    let point_p = sq(line_m) - 4 * t1 * t2;
    [line_m, point_p]
}

/// Check Ptolemy's theorem for a cyclic quadrilateral.
///
/// Ptolemy's theorem states that for a cyclic quadrilateral,
/// the product of the diagonals equals the sum of the products
/// of opposite sides.
///
/// Input: [Q12, Q23, Q34, Q14, Q13, Q24] - six side/diagonal quadrances.
///
/// # Examples
///
/// ```
/// use projgeom_rs::Ptolemy;
///
/// // A rectangle with sides 3,4 has diagonals 5
/// // Q12=9, Q23=16, Q34=9, Q14=16, Q13=25, Q24=25
/// assert!(Ptolemy(&[9, 16, 9, 16, 25, 25]));
/// ```
#[allow(non_snake_case)]
#[inline]
pub fn Ptolemy(quad: &[i64; 6]) -> bool {
    let [q12, q23, q34, q14, q13, q24] = *quad;
    archimedes(q12 * q34, q23 * q14, q13 * q24) == 0
}

/// Reflect a point across a line using an involution.
///
/// Creates an `Involution` from the mirror line's direction vector and
/// applies it to the point.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, EuclidLine, reflect_involution};
///
/// let mirror = EuclidLine::new([1, 0, 0]); // y-axis
/// let p = EuclidPoint::new([2, 0, 1]); // Point (2,0)
/// let reflected = reflect_involution(&mirror, &p);
/// // Reflecting (2,0) across y-axis gives (-2,0)
/// assert_eq!(reflected, EuclidPoint::new([-2, 0, 1]));
/// ```
#[inline]
pub fn reflect_involution(mirror: &EuclidLine, pt_p: &EuclidPoint) -> EuclidPoint {
    let dir = fB(mirror);
    let inv = Involution::new(mirror.clone(), dir);
    inv.apply_point(pt_p)
}

/// The `orthocenter` function calculates the orthocenter of a triangle given its three vertices.
///
/// Arguments:
///
/// * `triangle`: triangle is an array of 3 EuclidPoint objects representing the vertices of a triangle.
///
/// Returns:
///
/// The function `orthocenter` returns an `EuclidPoint` object.
///
/// # Examples
///
/// ```
/// use projgeom_rs::{EuclidPoint, orthocenter};
///
/// let p1 = EuclidPoint::new([0, 0, 1]);
/// let p2 = EuclidPoint::new([2, 0, 1]);
/// let p3 = EuclidPoint::new([1, 3, 1]);
/// let triangle = [p1, p2, p3];
/// let orthocenter_pt = orthocenter(&triangle);
/// // For a triangle with vertices (0,0), (2,0), (1,3), the orthocenter is (1, 1/3)
/// // In homogeneous coordinates, this would be (3, 1, 3)
/// assert_eq!(orthocenter_pt, EuclidPoint::new([3, 1, 3]));
/// ```
#[allow(dead_code)]
#[inline]
pub fn orthocenter(triangle: &[EuclidPoint; 3]) -> EuclidPoint {
    let [a_1, a_2, a_3] = triangle;
    assert!(!coincident(a_1, a_2, a_3));
    let t_1 = a_2.meet(a_3).altitude(a_1);
    let t_2 = a_3.meet(a_1).altitude(a_2);
    t_1.meet(&t_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pg_object::{EuclidLine, EuclidPoint};

    #[test]
    fn test_euclid_parallel() {
        let l1 = EuclidLine::new([1, 0, -1]); // x = 1
        let l2 = EuclidLine::new([2, 0, -5]); // x = 2.5
        assert!(l1.is_parallel(&l2));

        let l3 = EuclidLine::new([0, 1, -1]); // y = 1
        assert!(!l1.is_parallel(&l3));

        let l4 = EuclidLine::new([1, 1, -2]); // x + y = 2
        let l5 = EuclidLine::new([2, 2, -4]); // 2x + 2y = 4
        assert!(l4.is_parallel(&l5));
    }

    #[test]
    fn test_euclid_perpendicular() {
        let l1 = EuclidLine::new([1, 0, -1]); // x = 1
        let l2 = EuclidLine::new([0, 1, -1]); // y = 1
        assert!(l1.is_perpendicular(&l2));

        let l3 = EuclidLine::new([1, 1, -2]); // x + y = 2
        let l4 = EuclidLine::new([1, -1, 0]); // x - y = 0
        assert!(l3.is_perpendicular(&l4));

        assert!(!l1.is_perpendicular(&l3));
    }

    #[test]
    fn test_euclid_altitude() {
        let p = EuclidPoint::new([1, 2, 1]); // Point (1,2)
        let l = EuclidLine::new([1, 0, -1]); // Line x = 1
        let alt = l.altitude(&p);
        // The altitude from (1,2) to x=1 is the line y=2, which is (0,1,-2) in homogeneous coordinates.
        assert_eq!(alt, EuclidLine::new([0, 1, -2]));
    }

    #[test]
    fn test_euclid_midpoint() {
        let p1 = EuclidPoint::new([0, 0, 1]);
        let p2 = EuclidPoint::new([2, 4, 1]);
        let mid = p1.midpoint(&p2);
        assert_eq!(mid, EuclidPoint::new([1, 2, 1]));

        let p3 = EuclidPoint::new([1, 1, 1]);
        let p4 = EuclidPoint::new([5, 5, 1]);
        assert_eq!(p3.midpoint(&p4), EuclidPoint::new([3, 3, 1]));
    }

    #[test]
    fn test_euclid_tri_altitude() {
        let p1 = EuclidPoint::new([0, 0, 1]);
        let p2 = EuclidPoint::new([2, 0, 1]);
        let p3 = EuclidPoint::new([1, 3, 1]);
        let triangle = [p1.clone(), p2.clone(), p3.clone()];
        let altitudes = tri_altitude(&triangle);

        // Check if altitudes pass through vertices
        assert!(altitudes[0].incident(&p1));
        assert!(altitudes[1].incident(&p2));
        assert!(altitudes[2].incident(&p3));

        // Check if altitudes are perpendicular to opposite sides
        let sides = tri_dual(&triangle);
        assert!(altitudes[0].is_perpendicular(&sides[0]));
        assert!(altitudes[1].is_perpendicular(&sides[1]));
        assert!(altitudes[2].is_perpendicular(&sides[2]));
    }

    #[test]
    fn test_euclid_orthocenter() {
        let p1 = EuclidPoint::new([0, 0, 1]);
        let p2 = EuclidPoint::new([2, 0, 1]);
        let p3 = EuclidPoint::new([1, 3, 1]);
        let triangle = [p1, p2, p3];
        let orthocenter_pt = orthocenter(&triangle);

        // Orthocenter for (0,0), (2,0), (1,3) is (1, 1/3) -> (3, 1, 3)
        assert_eq!(orthocenter_pt, EuclidPoint::new([3, 1, 3]));
    }
}
