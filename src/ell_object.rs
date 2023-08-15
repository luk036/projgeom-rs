use crate::ck_plane::{CayleyKleinPlane, CayleyKleinPlanePrimitive};
use crate::pg_object::{EllipticLine, EllipticPoint};

/// The code is implementing the `CayleyKleinPlanePrimitive` trait for the `EllipticPoint` struct. This means that the
/// `EllipticPoint` struct is defining behavior for the `perp` method, which is required by the `CayleyKleinPlanePrimitive`
/// trait.
/// The code is implementing the `CayleyKleinPlanePrimitive` trait for the `EllipticPoint` struct.
impl CayleyKleinPlanePrimitive<EllipticLine> for EllipticPoint {
    #[inline]
    fn perp(&self) -> EllipticLine {
        EllipticLine::new(self.coord)
    }
}

/// The code is implementing the `CayleyKleinPlanePrimitive` trait for the `EllipticLine` struct. This means that the
/// `EllipticLine` struct is defining behavior for the `perp` method, which is required by the `CayleyKleinPlanePrimitive`
/// trait. The `perp` method in this implementation returns an `EllipticPoint` and is used to calculate the
/// perpendicular line to the given line in the context of hyperbolic geometry.
/// The code is implementing the `CayleyKleinPlanePrimitive` trait for the `EllipticLine` struct. It defines the `perp`
/// method for `EllipticLine` that returns an `EllipticPoint`. The `perp` method is used to calculate the
/// perpendicular line to the given line.
/// Hyperbolic geometry of the line
impl CayleyKleinPlanePrimitive<EllipticPoint> for EllipticLine {
    #[inline]
    fn perp(&self) -> EllipticPoint {
        EllipticPoint::new(self.coord)
    }
}

impl CayleyKleinPlane<EllipticLine, i128> for EllipticPoint {}

impl CayleyKleinPlane<EllipticPoint, i128> for EllipticLine {}
