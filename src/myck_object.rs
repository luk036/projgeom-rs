use crate::pg_object::{MyCKLine, MyCKPoint};
use crate::{CayleyKleinPlane, CayleyKleinPlanePrimitive};

const MYCK_POINT_PERP_COEFFS: [i64; 3] = [-2, 1, -2];
const MYCK_LINE_PERP_COEFFS: [i64; 3] = [-1, 2, -1];

impl_cayley_klein_plane!(
    MyCKPoint,
    MyCKLine,
    |p: &MyCKPoint| {
        MyCKLine::new([
            MYCK_POINT_PERP_COEFFS[0] * p.coord[0],
            MYCK_POINT_PERP_COEFFS[1] * p.coord[1],
            MYCK_POINT_PERP_COEFFS[2] * p.coord[2],
        ])
    },
    |l: &MyCKLine| {
        MyCKPoint::new([
            MYCK_LINE_PERP_COEFFS[0] * l.coord[0],
            MYCK_LINE_PERP_COEFFS[1] * l.coord[1],
            MYCK_LINE_PERP_COEFFS[2] * l.coord[2],
        ])
    }
);
