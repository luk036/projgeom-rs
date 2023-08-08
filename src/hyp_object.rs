use crate::ck_plane::{CayleyKleinPlane, CayleyKleinPlanePrimitive};
use crate::pg_object::{HypLine, HypPoint};

/// The code block is implementing the hyperbolic geometry of a point in the CayleyKleinPlanePrimitive trait for the
/// HypPoint struct.
impl CayleyKleinPlanePrimitive<HypLine> for HypPoint {
    #[inline]
    fn perp(&self) -> HypLine {
        HypLine::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}

/// The code block is implementing the hyperbolic geometry of a line in the CayleyKleinPlanePrimitive trait for the
/// HypLine struct. It defines a method called `perp` that returns a perpendicular point to the line.
/// The perpendicular point is created by negating the third coordinate of the line's coordinates and
/// creating a new HypPoint with the updated coordinates.
/// The code block is implementing the hyperbolic geometry of a line in the CayleyKleinPlanePrimitive trait for the
/// HypLine struct. It defines a method called `perp` that returns a perpendicular point to the line.
/// The perpendicular point is created by negating the third coordinate of the line's coordinates and
/// creating a new HypPoint with the updated coordinates.
/// Hyperbolic geometry of the line
impl CayleyKleinPlanePrimitive<HypPoint> for HypLine {
    #[inline]
    fn perp(&self) -> HypPoint {
        HypPoint::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}

impl CayleyKleinPlane<HypLine, i128> for HypPoint {}

impl CayleyKleinPlane<HypPoint, i128> for HypLine {}
