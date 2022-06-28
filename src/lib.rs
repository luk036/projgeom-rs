pub mod ck_plane;
// pub mod hyperbolic;
// pub mod elliptic;
pub mod ell_object;
pub mod hyp_object;
pub mod myck_object;
pub mod pg_object;
pub mod pg_plane;

pub use crate::ck_plane::*;
pub use crate::pg_object::{EllLine, EllPoint};
pub use crate::pg_object::{HypLine, HypPoint};
pub use crate::pg_object::{MyCKLine, MyCKPoint};
pub use crate::pg_object::{PgLine, PgPoint};
pub use crate::pg_plane::*;

#[cfg(test)]
mod tests {
    // use crate::pg_plane::{ProjPlanePrim, ProjPlane};
    // use crate::pg_plane::{check_axiom, coincident};
    // use crate::pg_object::*;
    use super::*;

    fn check_pg_plane<P, L>(p: P, q: P)
    where
        P: ProjPlane<L, i128> + std::fmt::Debug,
        L: ProjPlane<P, i128> + std::fmt::Debug,
    {
        let l = p.circ(&q);
        assert_eq!(l, q.circ(&p));
        assert!(l.incident(&p));
        assert!(l.incident(&q));
        let pq = p.plucker(&2, &q, &3);
        assert!(coincident(&p, &q, &pq));

        let h = harm_conj(&p, &q, &pq);
        assert_eq!(harm_conj(&p, &q, &h), pq);
    }

    #[test]
    fn test_pg_point() {
        let p = PgPoint::new([1, 3, 2]);
        let q = PgPoint::new([-2, 1, -1]);
        check_pg_plane(p, q);
    }

    #[test]
    fn test_pg_line() {
        let p = PgLine::new([1, 3, 2]);
        let q = PgLine::new([-2, 1, -1]);
        check_pg_plane(p, q);
    }

    fn check_ck_plane<P, L>(a1: P, a2: P, a3: P)
    where
        P: CKPlane<L, i128> + std::fmt::Debug,
        L: CKPlane<P, i128> + std::fmt::Debug,
    {
        let triangle = [a1, a2, a3];
        let trilateral = tri_dual(&triangle);
        let l1 = &trilateral[0];
        assert!(l1.incident(&triangle[1]));

        let [t1, t2, t3] = tri_altitude(&triangle);
        assert!(is_perpendicular(&t1, l1));
        let o = orthocenter(&triangle);
        assert_eq!(o, t2.circ(&t3));
    }

    #[test]
    fn test_ell_point() {
        let a1 = EllPoint::new([13, 23, 32]);
        let a2 = EllPoint::new([44, -34, 2]);
        let a3 = EllPoint::new([-2, 12, 23]);
        check_ck_plane(a1, a2, a3);
    }

    #[test]
    fn test_ell_line() {
        let a1 = EllLine::new([13, 23, 32]);
        let a2 = EllLine::new([44, -34, 2]);
        let a3 = EllLine::new([-2, 12, 23]);
        check_ck_plane(a1, a2, a3);
    }

    #[test]
    fn test_hyp_point() {
        let a1 = HypPoint::new([13, 23, 32]);
        let a2 = HypPoint::new([44, -34, 2]);
        let a3 = HypPoint::new([-2, 12, 23]);
        check_ck_plane(a1, a2, a3);
    }

    #[test]
    fn test_hyp_line() {
        let a1 = HypLine::new([13, 23, 32]);
        let a2 = HypLine::new([44, -34, 2]);
        let a3 = HypLine::new([-2, 12, 23]);
        check_ck_plane(a1, a2, a3);
    }

    #[test]
    fn test_myck_point() {
        let a1 = MyCKPoint::new([13, 23, 32]);
        let a2 = MyCKPoint::new([44, -34, 2]);
        let a3 = MyCKPoint::new([-2, 12, 23]);
        check_ck_plane(a1, a2, a3);
    }

    #[test]
    fn test_myck_line() {
        let a1 = MyCKLine::new([13, 23, 32]);
        let a2 = MyCKLine::new([44, -34, 2]);
        let a3 = MyCKLine::new([-2, 12, 23]);
        check_ck_plane(a1, a2, a3);
    }
}
