#[macro_use]
pub mod ck_plane;
// pub mod hyperbolic;
// pub mod elliptic;
pub mod ell_object;
pub mod euclid_object;
pub mod hyp_object;
pub mod myck_object;
pub mod persp_object;
pub mod pg_object;
pub mod pg_plane;

pub use crate::ck_plane::*;
pub use crate::pg_object::{EllipticLine, EllipticPoint};
pub use crate::pg_object::{EuclidLine, EuclidPoint};
pub use crate::pg_object::{HyperbolicLine, HyperbolicPoint};
pub use crate::pg_object::{MyCKLine, MyCKPoint};
pub use crate::pg_object::{PerspLine, PerspPoint};
pub use crate::pg_object::{PgLine, PgPoint};
pub use crate::pg_plane::*;

// pub mod fractions;
// pub use crate::fractions::Fraction;
