use crate::ck_plane::{CayleyKleinPlane, CayleyKleinPlanePrimitive};
use crate::pg_object::{MyCKLine, MyCKPoint};

impl CayleyKleinPlanePrimitive<MyCKLine> for MyCKPoint {
    #[inline]
    fn perp(&self) -> MyCKLine {
        MyCKLine::new([-2 * self.coord[0], self.coord[1], -2 * self.coord[2]])
    }
}

impl CayleyKleinPlanePrimitive<MyCKPoint> for MyCKLine {
    #[inline]
    fn perp(&self) -> MyCKPoint {
        MyCKPoint::new([-self.coord[0], 2 * self.coord[1], -self.coord[2]])
    }
}

impl CayleyKleinPlane<MyCKLine, i64> for MyCKPoint {}

impl CayleyKleinPlane<MyCKPoint, i64> for MyCKLine {}
