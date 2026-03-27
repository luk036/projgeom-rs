//! Hyperbolic Geometry
//!
//! This module provides support for hyperbolic geometry using the Cayley-Klein
//! model. In hyperbolic geometry, the perpendicular relationship is defined by the
//! form x² + y² - z² = 0.

use crate::pg_object::{HyperbolicLine, HyperbolicPoint};
use crate::{CayleyKleinPlane, CayleyKleinPlanePrimitive};

/// Coefficients for perpendicular transformation in hyperbolic geometry.
/// These define the polarity for the hyperbolic metric.
const HYP_PERP_COEFFS: [i64; 3] = [1, 1, -1];

impl_cayley_klein_plane!(
    HyperbolicPoint,
    HyperbolicLine,
    |p: &HyperbolicPoint| {
        HyperbolicLine::new([
            HYP_PERP_COEFFS[0] * p.coord[0],
            HYP_PERP_COEFFS[1] * p.coord[1],
            HYP_PERP_COEFFS[2] * p.coord[2],
        ])
    },
    |l: &HyperbolicLine| {
        HyperbolicPoint::new([
            HYP_PERP_COEFFS[0] * l.coord[0],
            HYP_PERP_COEFFS[1] * l.coord[1],
            HYP_PERP_COEFFS[2] * l.coord[2],
        ])
    }
);
