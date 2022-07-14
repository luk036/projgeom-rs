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
pub use crate::pg_object::{EllLine, EllPoint};
pub use crate::pg_object::{EuclidLine, EuclidPoint};
pub use crate::pg_object::{HypLine, HypPoint};
pub use crate::pg_object::{MyCKLine, MyCKPoint};
pub use crate::pg_object::{PerspLine, PerspPoint};
pub use crate::pg_object::{PgLine, PgPoint};
pub use crate::pg_plane::*;

pub mod fractions;
pub use crate::fractions::Fraction;

#[cfg(test)]
mod tests {
    use num_integer::gcd;
    // use crate::pg_plane::{ProjPlanePrim, ProjPlane};
    // use crate::pg_plane::{check_axiom, coincident};
    // use crate::pg_object::*;
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn it_works() {
        let result = gcd(4, -6);
        assert_eq!(result, 2);

        let f = Fraction::new(30, -40);
        assert_eq!(f, Fraction::new(-3, 4));

        let h = Fraction::from(30);
        assert_eq!(h, Fraction::new(30, 1));

        let g = Fraction::<i32>::default();
        assert_eq!(g, Fraction::new(0, 1));
    }

    #[test]
    fn test_cross() {
        let f = Fraction::new(30, 40);
        let h = Fraction::from(3);
        let result = Fraction::cross(&f, &h);
        assert_eq!(result, -9);
        assert_eq!(h, 3);
        // assert_eq!(result, 30);
    }

    #[test]
    fn test_ordering() {
        let f = Fraction::new(3, 4);
        assert!(f != 1i32);
        assert!(1i32 != f);
        assert!(f < 1i32);
        assert!(1i32 > f);
        // assert_eq!(result, 30);
    }

    #[test]
    fn test_mul_div_assign() {
        let mut f = Fraction::new(3, 4);
        let g = Fraction::new(5, 6);
        f *= g;
        assert_eq!(f, Fraction::new(5, 8));
        f /= g;
        assert_eq!(f, Fraction::new(3, 4));
        f *= 2;
        assert_eq!(f, Fraction::new(3, 2));
        f /= 2;
        assert_eq!(f, Fraction::new(3, 4));
        f /= 0;
        assert_eq!(f, Fraction::new(1, 0));
        assert_eq!(-g, Fraction::new(-5, 6));
    }

    #[test]
    fn test_add_sub_assign() {
        let mut f = Fraction::new(3, 4);
        let g = Fraction::new(5, 6);
        f -= g;
        assert_eq!(f, Fraction::new(-1, 12));
        f += g;
        assert_eq!(f, Fraction::new(3, 4));
        f -= 2;
        assert_eq!(f, Fraction::new(-5, 4));
        f += 2;
        assert_eq!(f, Fraction::new(3, 4));
    }

    #[test]
    fn test_mul_div() {
        let f = Fraction::new(3, 4);
        let g = Fraction::new(5, 6);
        assert_eq!(f * g, Fraction::new(5, 8));
        assert_eq!(f / g, Fraction::new(9, 10));
        assert_eq!(f * 2, Fraction::new(3, 2));
        assert_eq!(f / 2, Fraction::new(3, 8));
    }

    #[test]
    fn test_add_sub() {
        let f = Fraction::new(3, 4);
        let g = Fraction::new(5, 6);
        assert_eq!(f - g, Fraction::new(-1, 12));
        assert_eq!(f + g, Fraction::new(19, 12));
        assert_eq!(f - 2, Fraction::new(-5, 4));
        assert_eq!(f + 2, Fraction::new(11, 4));
    }

    #[test]
    fn test_special() {
        let zero = Fraction::new(0, 1);
        let infp = Fraction::new(1, 0);
        let infn = Fraction::new(-1, 0);
        let pos = Fraction::new(1, 40);
        let neg = Fraction::new(-1, 2);
        assert!(infn < neg);
        assert!(neg < zero);
        assert!(zero < pos);
        assert!(pos < infp);
        assert!(infn < infp);

        let nan = Fraction::new(0, 0);
        assert_eq!(infn * neg, infp);
        assert_eq!(infn * pos, infn);
        assert_eq!(infp * zero, nan);
        assert_eq!(infn * zero, nan);
        assert_eq!(infp / infp, nan);
        assert_eq!(infp + infp, infp);
        assert_eq!(infp - infp, nan);
        assert_eq!(infp - pos, infp);
        assert_eq!(infn + pos, infn);
    }

    fn check_pg_plane<P, L>(p: P, q: P)
    where
        P: ProjPlane<L, i128> + std::fmt::Debug,
        L: ProjPlane<P, i128> + std::fmt::Debug,
    {
        let l = p.circ(&q);
        assert_eq!(l, q.circ(&p));
        assert!(l.incident(&p));
        assert!(l.incident(&q));
        let pq = P::plucker(&p, &2, &q, &3);
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

    #[test]
    fn test_persp_point() {
        let a1 = PerspPoint::new([13, 23, 32]);
        let a2 = PerspPoint::new([44, -34, 2]);
        let a3 = PerspPoint::new([-2, 12, 23]);
        check_ck_plane(a1, a2, a3);
    }

    // #[test]
    // fn test_persp_line() {
    //     let a1 = PerspLine::new([13, 23, 32]);
    //     let a2 = PerspLine::new([44, -34, 2]);
    //     let a3 = PerspLine::new([-2, 12, 23]);
    //     check_ck_plane(a1, a2, a3);
    // }

    #[test]
    fn test_euclid_point() {
        let a1 = EuclidPoint::new([13, 23, 32]);
        let a2 = EuclidPoint::new([44, -34, 2]);
        let a3 = EuclidPoint::new([-2, 12, 23]);
        check_ck_plane(a1, a2, a3);
    }

    #[quickcheck]
    fn test_pg_point_q(pz: i64, qz: i64) -> bool {
        let p = PgPoint::new([1, 3, pz.into()]);
        let q = PgPoint::new([-2, 1, qz.into()]);
        p != q
    }

    #[quickcheck]
    fn test_pg_point_q2(pz: i32, qz: i32) -> bool {
        let p = PgPoint::new([100000003, 30000001, pz.into()]);
        let q = PgPoint::new([-200000004, 100000005, qz.into()]);
        let l = p.circ(&q);
        l == q.circ(&p)
    }

    #[quickcheck]
    fn test_pg_point_q3(pz: i32, qz: i32) -> bool {
        let p = PgPoint::new([100000003, 30000001, pz.into()]);
        let q = PgPoint::new([-200000004, 100000005, qz.into()]);
        let l = p.circ(&q);
        l.incident(&p) && l.incident(&q) 
    }
}
