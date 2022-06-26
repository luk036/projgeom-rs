pub mod ck_plane;
// pub mod hyperbolic;
// pub mod elliptic;
pub mod ell_object;
pub mod hyp_object;
pub mod pg_object;
pub mod pg_plane;

pub use crate::ck_plane::*;
pub use crate::pg_plane::*;
pub use crate::pg_object::{PgPoint, PgLine};

#[cfg(test)]
mod tests {
    // use crate::pg_plane::{ProjPlanePrim, ProjPlane};
    // use crate::pg_plane::{check_axiom, coincident};
    // use crate::pg_object::*;
    use super::*;

    #[test]
    fn test_pg_point() {
        let p = PgPoint::new([1, 3, 2]);
        let q = PgPoint::new([-2, 1, -1]);

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
    fn test_pg_line() {
        let p = PgLine::new([1, 3, 2]);
        let q = PgLine::new([-2, 1, -1]);

        let l = p.circ(&q);
        assert_eq!(l, q.circ(&p));
        assert!(l.incident(&p));
        assert!(l.incident(&q));

        let pq = p.plucker(&2, &q, &3);
        assert!(coincident(&p, &q, &pq));

        let h = harm_conj(&p, &q, &pq);
        assert_eq!(harm_conj(&p, &q, &h), pq);
    }
}
