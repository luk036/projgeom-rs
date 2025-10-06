// Euclidean Geometry

use crate::pg_object::{EuclidLine, EuclidPoint};
use crate::pg_plane::{coincident, tri_dual, ProjectivePlane, ProjectivePlanePrimitive};
use crate::{CayleyKleinPlane, CayleyKleinPlanePrimitive};
// use crate::pg_object::{plucker_operation, dot_product};
use crate::pg_object::dot1;

// static I_RE: EuclidPoint = EuclidPoint { coord: [0, 1, 1] };
// static I_IM: EuclidPoint = EuclidPoint { coord: [1, 0, 0] };
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
    /// The `altitude` function calculates the perpendicular line from a given point to a line.
    ///
    /// Arguments:
    ///
    /// * `pt_a`: The parameter `pt_a` is of type `EuclidPoint`.
    ///
    /// Returns:
    ///
    /// The `altitude` function returns an `EuclidLine` object.
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
///               a triangle.
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
