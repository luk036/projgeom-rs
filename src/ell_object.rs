use crate::ck_plane::CKPlanePrim;
use crate::pg_object::{EllLine, EllPoint};

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
