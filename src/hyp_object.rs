use crate::ck_plane::{CKPlane, CKPlanePrim};
use crate::pg_object::{HypLine, HypPoint};

impl CKPlanePrim<HypLine> for HypPoint {
    #[inline]
    fn perp(&self) -> HypLine {
        HypLine::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}

impl CKPlanePrim<HypPoint> for HypLine {
    #[inline]
    fn perp(&self) -> HypPoint {
        HypPoint::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}

impl CKPlane<HypLine, i128> for HypPoint {}

impl CKPlane<HypPoint, i128> for HypLine {}
