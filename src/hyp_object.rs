use crate::ck_plane::{CKPlane, CKPlanePrim};
use crate::pg_object::{HypLine, HypPoint};

/// The code block is implementing the hyperbolic geometry of a point in the CKPlanePrim trait for the
/// HypPoint struct.
impl CKPlanePrim<HypLine> for HypPoint {
    #[inline]
    fn perp(&self) -> HypLine {
        HypLine::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}

/// The code block is implementing the hyperbolic geometry of a line in the CKPlanePrim trait for the
/// HypLine struct. It defines a method called `perp` that returns a perpendicular point to the line.
/// The perpendicular point is created by negating the third coordinate of the line's coordinates and
/// creating a new HypPoint with the updated coordinates.
/// The code block is implementing the hyperbolic geometry of a line in the CKPlanePrim trait for the
/// HypLine struct. It defines a method called `perp` that returns a perpendicular point to the line.
/// The perpendicular point is created by negating the third coordinate of the line's coordinates and
/// creating a new HypPoint with the updated coordinates.
/// Hyperbolic geometry of the line
impl CKPlanePrim<HypPoint> for HypLine {
    #[inline]
    fn perp(&self) -> HypPoint {
        HypPoint::new([self.coord[0], self.coord[1], -self.coord[2]])
    }
}

impl CKPlane<HypLine, i128> for HypPoint {}

impl CKPlane<HypPoint, i128> for HypLine {}
