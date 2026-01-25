#[macro_use]
pub mod ck_plane;
pub mod conic;
pub mod cross_ratio;
// pub mod hyperbolic;
// pub mod elliptic;
pub mod ell_object;
pub mod error;
pub mod euclid_object;
pub mod ffi;
pub mod hyp_object;
pub mod myck_object;
pub mod persp_object;
pub mod pg_object;
pub mod pg_plane;
pub mod predicates;
pub mod transform;
pub mod visualization;

pub use crate::ck_plane::*;
pub use crate::conic::{Conic, ConicType};
pub use crate::cross_ratio::{
    cross_ratio, cross_ratio_lines, is_harmonic_division, projective_transform_line,
    projective_transform_point,
};
pub use crate::error::{GeometryError, Result};
pub use crate::pg_object::{EllipticLine, EllipticPoint};
pub use crate::pg_object::{EuclidLine, EuclidPoint};
pub use crate::pg_object::{HyperbolicLine, HyperbolicPoint};
pub use crate::pg_object::{MyCKLine, MyCKPoint};
pub use crate::pg_object::{PerspLine, PerspPoint};
pub use crate::pg_object::{PgLine, PgPoint};
pub use crate::pg_plane::*;
pub use crate::predicates::*;
pub use crate::transform::*;
pub use crate::visualization::SvgRenderer;

// pub mod fractions;
// pub use crate::fractions::Fraction;
