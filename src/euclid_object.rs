// Euclidean Geometry

use crate::ck_plane::{CKPlane, CKPlanePrim};
use crate::pg_object::{EuclidLine, EuclidPoint};
use crate::pg_plane::{ProjPlane, ProjPlanePrim};
// use crate::pg_object::{plckr, dot};
use crate::pg_object::{cross2, dot1};

// static I_RE: EuclidPoint = EuclidPoint { coord: [0, 1, 1] };
// static I_IM: EuclidPoint = EuclidPoint { coord: [1, 0, 0] };
static L_INF: EuclidLine = EuclidLine { coord: [0, 0, 1] };

impl CKPlanePrim<EuclidLine> for EuclidPoint {
    #[inline]
    fn perp(&self) -> EuclidLine {
        L_INF.clone()
    }
}

impl CKPlanePrim<EuclidPoint> for EuclidLine {
    #[inline]
    fn perp(&self) -> EuclidPoint {
        EuclidPoint::new([self.coord[0], self.coord[1], 0])
    }
}

impl CKPlane<EuclidLine, i128> for EuclidPoint {}

impl CKPlane<EuclidPoint, i128> for EuclidLine {}

impl EuclidLine {
    #[inline]
    pub fn is_parallel(&self, other: &EuclidLine) -> bool {
        cross2(&self.coord, &other.coord) == 0
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
        self.plucker(&other.coord[2], other, &self.coord[2])
    }
}
