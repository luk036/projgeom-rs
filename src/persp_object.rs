// Perspective Geometry

use crate::ck_plane::{CKPlane, CKPlanePrim};
use crate::pg_object::{PerspLine, PerspPoint};
use crate::pg_plane::{ProjPlane, ProjPlanePrim};
// use crate::pg_object::{plckr, dot};

static I_RE: PerspPoint = PerspPoint { coord: [0, 1, 1] };
static I_IM: PerspPoint = PerspPoint { coord: [1, 0, 0] };
static L_INF: PerspLine = PerspLine { coord: [0, -1, 1] };

impl CKPlanePrim<PerspLine> for PerspPoint {
    #[inline]
    fn perp(&self) -> PerspLine {
        L_INF.clone()
    }
}

impl CKPlanePrim<PerspPoint> for PerspLine {
    #[inline]
    fn perp(&self) -> PerspPoint {
        let alpha = I_RE.dot(self);
        let beta = I_IM.dot(self);
        I_RE.plucker(&alpha, &I_IM, &beta)
    }
}

impl CKPlane<PerspLine, i128> for PerspPoint {}

impl CKPlane<PerspPoint, i128> for PerspLine {}

impl PerspLine {
    #[inline]
    pub fn is_parallel(&self, other: &PerspLine) -> bool {
        L_INF.dot(&self.circ(other)) == 0
    }
}

impl PerspPoint {
    #[inline]
    pub fn midpoint(&self, other: &PerspPoint) -> PerspPoint {
        let alpha = L_INF.dot(other);
        let beta = L_INF.dot(self);
        self.plucker(&alpha, other, &beta)
    }
}
