use crate::ck_plane::{CKPlane, CKPlanePrim};
use crate::pg_object::{EllLine, EllPoint};

/// The code is implementing the `CKPlanePrim` trait for the `EllPoint` struct. This means that the
/// `EllPoint` struct is defining behavior for the `perp` method, which is required by the `CKPlanePrim`
/// trait.
/// The code is implementing the `CKPlanePrim` trait for the `EllPoint` struct.
impl CKPlanePrim<EllLine> for EllPoint {
    #[inline]
    fn perp(&self) -> EllLine {
        EllLine::new(self.coord)
    }
}

/// The code is implementing the `CKPlanePrim` trait for the `EllLine` struct. This means that the
/// `EllLine` struct is defining behavior for the `perp` method, which is required by the `CKPlanePrim`
/// trait. The `perp` method in this implementation returns an `EllPoint` and is used to calculate the
/// perpendicular line to the given line in the context of hyperbolic geometry.
/// The code is implementing the `CKPlanePrim` trait for the `EllLine` struct. It defines the `perp`
/// method for `EllLine` that returns an `EllPoint`. The `perp` method is used to calculate the
/// perpendicular line to the given line.
/// Hyperbolic geometry of the line
impl CKPlanePrim<EllPoint> for EllLine {
    #[inline]
    fn perp(&self) -> EllPoint {
        EllPoint::new(self.coord)
    }
}

impl CKPlane<EllLine, i128> for EllPoint {}

impl CKPlane<EllPoint, i128> for EllLine {}
