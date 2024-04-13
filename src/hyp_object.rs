use crate::ck_plane::{CayleyKleinPlane, CayleyKleinPlanePrimitive};
use crate::pg_object::{HyperbolicLine, HyperbolicPoint};

/// The code block is implementing the hyperbolic geometry of a point in the CayleyKleinPlanePrimitive trait for the
/// HyperbolicPoint struct.
impl CayleyKleinPlanePrimitive<HyperbolicLine> for HyperbolicPoint {
    #[inline]
    fn perp(&self) -> HyperbolicLine {
        HyperbolicLine::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}

/// The code block is implementing the hyperbolic geometry of a line in the CayleyKleinPlanePrimitive trait for the
/// HyperbolicLine struct. It defines a method called `perp` that returns a perpendicular point to the line.
/// The perpendicular point is created by negating the third coordinate of the line's coordinates and
/// creating a new HyperbolicPoint with the updated coordinates.
/// The code block is implementing the hyperbolic geometry of a line in the CayleyKleinPlanePrimitive trait for the
/// HyperbolicLine struct. It defines a method called `perp` that returns a perpendicular point to the line.
/// The perpendicular point is created by negating the third coordinate of the line's coordinates and
/// creating a new HyperbolicPoint with the updated coordinates.
/// Hyperbolic geometry of the line
impl CayleyKleinPlanePrimitive<HyperbolicPoint> for HyperbolicLine {
    #[inline]
    fn perp(&self) -> HyperbolicPoint {
        HyperbolicPoint::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}

impl CayleyKleinPlane<HyperbolicLine, i64> for HyperbolicPoint {}

impl CayleyKleinPlane<HyperbolicPoint, i64> for HyperbolicLine {}
