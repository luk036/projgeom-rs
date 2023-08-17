// Euclidean Geometry

use crate::ck_plane::{CayleyKleinPlane, CayleyKleinPlanePrimitive};
use crate::pg_object::{EuclidLine, EuclidPoint};
use crate::pg_plane::{coincident, tri_dual, ProjectivePlane, ProjectivePlanePrimitive};
// use crate::pg_object::{plucker_operation, dot_product};
use crate::pg_object::dot1;

// static I_RE: EuclidPoint = EuclidPoint { coord: [0, 1, 1] };
// static I_IM: EuclidPoint = EuclidPoint { coord: [1, 0, 0] };
static L_INF: EuclidLine = EuclidLine { coord: [0, 0, 1] };

/// This code is implementing the `perp` method for the `CayleyKleinPlanePrimitive` trait for the `EuclidPoint`
/// struct. The `perp` method returns a perpendicular line to the given point. In this implementation,
/// it always returns the line `L_INF`, which represents the line at infinity.
impl CayleyKleinPlanePrimitive<EuclidLine> for EuclidPoint {
    #[allow(dead_code)]
    fn perp(&self) -> EuclidLine {
        L_INF.clone()
    }
}

/// This code is implementing the `perp` method for the `CayleyKleinPlanePrimitive` trait for the `EuclidLine` struct.
/// The `perp` method returns a perpendicular point to the given line. In this implementation, it
/// creates a new `EuclidPoint` with the x and y coordinates of the line and a z coordinate of 0. This
/// represents a point that is perpendicular to the line in the xy-plane.
impl CayleyKleinPlanePrimitive<EuclidPoint> for EuclidLine {
    #[allow(dead_code)]
    fn perp(&self) -> EuclidPoint {
        EuclidPoint::new([self.coord[0], self.coord[1], 0])
    }
}

impl CayleyKleinPlane<EuclidLine, i128> for EuclidPoint {}

impl CayleyKleinPlane<EuclidPoint, i128> for EuclidLine {}

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
/// a triangle.
///
/// Returns:
///
/// The function `tri_altitude` returns an array of `EuclidLine` objects, specifically `[t_1, t_2, t_3]`.
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
#[allow(dead_code)]
#[inline]
pub fn orthocenter(triangle: &[EuclidPoint; 3]) -> EuclidPoint {
    let [a_1, a_2, a_3] = triangle;
    assert!(!coincident(a_1, a_2, a_3));
    let t_1 = a_2.meet(a_3).altitude(a_1);
    let t_2 = a_3.meet(a_1).altitude(a_2);
    t_1.meet(&t_2)
}
