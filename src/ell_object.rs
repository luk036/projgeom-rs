use crate::ck_plane::{CKPlanePrim, CKPlane};
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

impl CKPlane<EllLine, i64> for EllPoint {
}

impl CKPlane<EllPoint, i64> for EllLine {
}
