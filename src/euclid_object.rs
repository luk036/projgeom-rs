// Euclidean Geometry

use crate::ck_plane::{CKPlane, CKPlanePrim};
use crate::pg_object::{EuclidLine, EuclidPoint};
use crate::pg_plane::{coincident, tri_dual, ProjPlane, ProjPlanePrim};
// use crate::pg_object::{plckr, dot};
use crate::pg_object::dot1;

// static I_RE: EuclidPoint = EuclidPoint { coord: [0, 1, 1] };
// static I_IM: EuclidPoint = EuclidPoint { coord: [1, 0, 0] };
static L_INF: EuclidLine = EuclidLine { coord: [0, 0, 1] };

impl CKPlanePrim<EuclidLine> for EuclidPoint {
    #[allow(dead_code)]
    fn perp(&self) -> EuclidLine {
        L_INF.clone()
    }
}

impl CKPlanePrim<EuclidPoint> for EuclidLine {
    #[allow(dead_code)]
    fn perp(&self) -> EuclidPoint {
        EuclidPoint::new([self.coord[0], self.coord[1], 0])
    }
}

impl CKPlane<EuclidLine, i128> for EuclidPoint {}

impl CKPlane<EuclidPoint, i128> for EuclidLine {}

impl EuclidLine {
    #[inline]
    pub fn is_parallel(&self, other: &EuclidLine) -> bool {
        self.coord[0] * other.coord[1] == self.coord[1] * other.coord[0]
    }

    #[inline]
    pub fn is_perpendicular(&self, other: &EuclidLine) -> bool {
        dot1(&self.coord, &other.coord) == 0
    }

    #[inline]
    pub fn altitude(&self, a: &EuclidPoint) -> EuclidLine {
        self.perp().circ(a)
    }
}

impl EuclidPoint {
    #[inline]
    pub fn midpoint(&self, other: &EuclidPoint) -> EuclidPoint {
        EuclidPoint::plucker(self, other.coord[2], other, self.coord[2])
    }
}

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

#[allow(dead_code)]
#[inline]
pub fn orthocenter(tri: &[EuclidPoint; 3]) -> EuclidPoint {
    let [a1, a2, a3] = tri;
    assert!(!coincident(a1, a2, a3));
    let t1 = a2.circ(a3).altitude(a1);
    let t2 = a3.circ(a1).altitude(a2);
    t1.circ(&t2)
}
