//! Elliptic Geometry
//!
//! This module provides support for elliptic geometry using the Cayley-Klein
//! model. In elliptic geometry, the perpendicular relationship is defined by the
//! polarity induced by the elliptic quadric, where each point is self-dual.

use crate::pg_object::{EllipticLine, EllipticPoint};
use crate::{CayleyKleinPlane, CayleyKleinPlanePrimitive};

impl_cayley_klein_plane!(
    EllipticPoint,
    EllipticLine,
    |p: &EllipticPoint| EllipticLine::new(p.coord),
    |l: &EllipticLine| EllipticPoint::new(l.coord)
);
