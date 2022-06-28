use crate::ck_plane::{CKPlane, CKPlanePrim};
use crate::pg_object::{MyCKLine, MyCKPoint};

impl CKPlanePrim<MyCKLine> for MyCKPoint {
    #[inline]
    fn perp(&self) -> MyCKLine {
        MyCKLine::new([-2 * self.coord[0], self.coord[1], -2 * self.coord[2]])
    }
}

impl CKPlanePrim<MyCKPoint> for MyCKLine {
    #[inline]
    fn perp(&self) -> MyCKPoint {
        MyCKPoint::new([-self.coord[0], 2 * self.coord[1], -self.coord[2]])
    }
}

impl CKPlane<MyCKLine, i128> for MyCKPoint {}

impl CKPlane<MyCKPoint, i128> for MyCKLine {}
