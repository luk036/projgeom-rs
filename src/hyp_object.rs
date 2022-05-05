use crate::pg_object::{HypPoint, HypLine};
use crate::ck_plane::CKPlanePrim;

impl CKPlanePrim<HypLine> for HypPoint {
    // type Dual: ProjPlanePrim;
    fn perp(&self) -> HypLine {
        HypLine::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}

impl CKPlanePrim<HypPoint> for HypLine {
    // type Dual: ProjPlanePrim;
    fn perp(&self) -> HypPoint {
        HypPoint::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}
