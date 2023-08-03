// Euclidean Geometry

use crate::ck_plane::{CKPlane, CKPlanePrim};
use crate::pg_object::{EuclidLine, EuclidPoint};
use crate::pg_plane::{coincident, tri_dual, ProjPlane, ProjPlanePrim};
// use crate::pg_object::{plckr, dot};
use crate::pg_object::dot1;

// static I_RE: EuclidPoint = EuclidPoint { coord: [0, 1, 1] };
// static I_IM: EuclidPoint = EuclidPoint { coord: [1, 0, 0] };
static L_INF: EuclidLine = EuclidLine { coord: [0, 0, 1] };

/// This code is implementing the `perp` method for the `CKPlanePrim` trait for the `EuclidPoint`
/// struct. The `perp` method returns a perpendicular line to the given point. In this implementation,
/// it always returns the line `L_INF`, which represents the line at infinity.
impl CKPlanePrim<EuclidLine> for EuclidPoint {
    #[allow(dead_code)]
    fn perp(&self) -> EuclidLine {
        L_INF.clone()
    }
}

/// This code is implementing the `perp` method for the `CKPlanePrim` trait for the `EuclidLine` struct.
/// The `perp` method returns a perpendicular point to the given line. In this implementation, it
/// creates a new `EuclidPoint` with the x and y coordinates of the line and a z coordinate of 0. This
/// represents a point that is perpendicular to the line in the xy-plane.
impl CKPlanePrim<EuclidPoint> for EuclidLine {
    #[allow(dead_code)]
    fn perp(&self) -> EuclidPoint {
        EuclidPoint::new([self.coord[0], self.coord[1], 0])
    }
}

impl CKPlane<EuclidLine, i128> for EuclidPoint {}

impl CKPlane<EuclidPoint, i128> for EuclidLine {}

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
    #[inline]
    pub fn is_parallel(&self, other: &EuclidLine) -> bool {
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
    /// The function checks if two Euclidean lines are perpendicular.
    ///
    /// Arguments:
    ///
    /// * `other`: `other` is a reference to an `EuclidLine` object.
    ///
    /// Returns:
    ///
    /// a boolean value, indicating whether the two lines are perpendicular to each other.
    #[inline]
    pub fn is_perpendicular(&self, other: &EuclidLine) -> bool {
        dot1(&self.coord, &other.coord) == 0
    }

    /// The `altitude` function calculates the perpendicular line from a given point to a line.
    ///
    /// Arguments:
    ///
    /// * `a`: The parameter `a` is of type `EuclidPoint`.
    ///
    /// Returns:
    ///
    /// The `altitude` function returns an `EuclidLine` object.
    /// The `altitude` function calculates the perpendicular line from a given point to a line.
    ///
    /// Arguments:
    ///
    /// * `a`: The parameter `a` is of type `EuclidPoint`.
    ///
    /// Returns:
    ///
    /// The `altitude` function returns an `EuclidLine` object.
    #[inline]
    pub fn altitude(&self, a: &EuclidPoint) -> EuclidLine {
        self.perp().circ(a)
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
    #[inline]
    pub fn midpoint(&self, other: &EuclidPoint) -> EuclidPoint {
        EuclidPoint::plucker(self, other.coord[2], other, self.coord[2])
    }
}

/// The `tri_altitude` function calculates the altitudes of a triangle given its three vertices.
///
/// Arguments:
///
/// * `tri`: The `tri` parameter is an array of `EuclidPoint` structs representing the three vertices of
/// a triangle.
///
/// Returns:
///
/// The function `tri_altitude` returns an array of `EuclidLine` objects, specifically `[t1, t2, t3]`.
#[allow(dead_code)]
pub fn tri_altitude(tri: &[EuclidPoint; 3]) -> [EuclidLine; 3] {
    let [l1, l2, l3] = tri_dual(tri);
    let [a1, a2, a3] = tri;
    assert!(!coincident(a1, a2, a3));
    let t1 = l1.altitude(a1);
    let t2 = l2.altitude(a2);
    let t3 = l3.altitude(a3);
    [t1, t2, t3]
}

/// The `orthocenter` function calculates the orthocenter of a triangle given its three vertices.
///
/// Arguments:
///
/// * `tri`: tri is an array of 3 EuclidPoint objects representing the vertices of a triangle.
///
/// Returns:
///
/// The function `orthocenter` returns an `EuclidPoint` object.
#[allow(dead_code)]
#[inline]
pub fn orthocenter(tri: &[EuclidPoint; 3]) -> EuclidPoint {
    let [a1, a2, a3] = tri;
    assert!(!coincident(a1, a2, a3));
    let t1 = a2.circ(a3).altitude(a1);
    let t2 = a3.circ(a1).altitude(a2);
    t1.circ(&t2)
}
