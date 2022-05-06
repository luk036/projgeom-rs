use crate::ck_plane::CKPlanePrim;
use crate::pg_object::{HypLine, HypPoint};

impl CKPlanePrim<HypLine> for HypPoint {
    fn perp(&self) -> HypLine {
        HypLine::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}

impl CKPlanePrim<HypPoint> for HypLine {
    fn perp(&self) -> HypPoint {
        HypPoint::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}
