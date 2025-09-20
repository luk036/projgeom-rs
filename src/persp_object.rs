// Perspective Geometry

use crate::pg_object::{PerspLine, PerspPoint};
use crate::pg_plane::{ProjectivePlane, ProjectivePlanePrimitive};
use crate::{CayleyKleinPlane, CayleyKleinPlanePrimitive};
// use crate::pg_object::{plucker_operation, dot};

static I_RE: PerspPoint = PerspPoint { coord: [0, 1, 1] };
static I_IM: PerspPoint = PerspPoint { coord: [1, 0, 0] };
static L_INF: PerspLine = PerspLine { coord: [0, -1, 1] };

impl_cayley_klein_plane!(
    PerspPoint,
    PerspLine,
    |_p: &PerspPoint| L_INF.clone(),
    |l: &PerspLine| {
        let alpha = I_RE.dot(l); // ???
        let beta = I_IM.dot(l); // ???
        PerspPoint::parametrize(&I_RE, alpha, &I_IM, beta)
    }
);

impl PerspLine {
    /// The function checks if two perspective lines are parallel.
    ///
    /// Arguments:
    ///
    /// * `other`: `other` is a reference to an object of type `PerspLine`.
    ///
    /// Returns:
    ///
    /// a boolean value.
    #[inline]
    pub fn is_parallel(&self, other: &PerspLine) -> bool {
        L_INF.dot(&self.meet(other)) == 0
    }
}

impl PerspPoint {
    /// The `midpoint` function calculates the midpoint between two `PerspPoint` objects using the dot
    /// product and Plücker coordinates.
    ///
    /// Arguments:
    ///
    /// * `other`: A reference to another `PerspPoint` object.
    ///
    /// Returns:
    ///
    /// The `midpoint` function returns a `PerspPoint` object.
    #[inline]
    pub fn midpoint(&self, other: &PerspPoint) -> PerspPoint {
        let alpha = L_INF.dot(other);
        let beta = L_INF.dot(self);
        PerspPoint::parametrize(self, alpha, other, beta)
    }
}
