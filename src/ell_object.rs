use crate::pg_object::{EllipticLine, EllipticPoint};
use crate::{CayleyKleinPlane, CayleyKleinPlanePrimitive};

impl_cayley_klein_plane!(
    EllipticPoint,
    EllipticLine,
    |p: &EllipticPoint| EllipticLine::new(p.coord),
    |l: &EllipticLine| EllipticPoint::new(l.coord)
);
