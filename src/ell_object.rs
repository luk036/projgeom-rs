use crate::ck_plane::{CKPlane, CKPlanePrim};
use crate::pg_object::{EllLine, EllPoint};

impl CKPlanePrim<EllLine> for EllPoint {
    #[inline]
    fn perp(&self) -> EllLine {
        EllLine::new(self.coord)
    }
}

impl CKPlanePrim<EllPoint> for EllLine {
    #[inline]
    fn perp(&self) -> EllPoint {
        EllPoint::new(self.coord)
    }
}

impl CKPlane<EllLine, i128> for EllPoint {}

impl CKPlane<EllPoint, i128> for EllLine {}
