use crate::pg_object::{EllPoint, EllLine};
use crate::ck_plane::CKPlanePrim;

impl CKPlanePrim<EllLine> for EllPoint {
    fn perp(&self) -> EllLine {
        EllLine::new(self.coord)
    }
}

impl CKPlanePrim<EllPoint> for EllLine {
    fn perp(&self) -> EllPoint {
        EllPoint::new(self.coord)
    }
}
