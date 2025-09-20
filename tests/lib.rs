use projgeom_rs::*;
use num_integer::gcd;
use quickcheck_macros::quickcheck;

use fractions::Fraction;

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

fn check_pg_plane<Point, Line>(pt_p: Point, pt_q: Point)
where
    Point: ProjectivePlane<Line, i64> + std::fmt::Debug,
    Line: ProjectivePlane<Point, i64> + std::fmt::Debug,
{
    let ln_l = pt_p.meet(&pt_q);
    assert_eq!(ln_l, pt_q.meet(&pt_p));
    assert!(ln_l.incident(&pt_p));
    assert!(ln_l.incident(&pt_q));
    let pq = Point::parametrize(&pt_p, 2, &pt_q, 3);
    assert!(coincident(&pt_p, &pt_q, &pq));

    let h = harm_conj(&pt_p, &pt_q, &pq);
    assert_eq!(harm_conj(&pt_p, &pt_q, &h), pq);
}

#[test]
fn test_pg_point() {
    let pt_p = PgPoint::new([1, 3, 2]);
    let pt_q = PgPoint::new([-2, 1, -1]);
    check_pg_plane(pt_p, pt_q);
}

#[test]
fn test_pg_line() {
    let pt_p = PgLine::new([1, 3, 2]);
    let pt_q = PgLine::new([-2, 1, -1]);
    check_pg_plane(pt_p, pt_q);
}

fn check_ck_plane<Point, Line>(a_1: Point, a_2: Point, a_3: Point)
where
    Point: CayleyKleinPlane<Line, i64> + std::fmt::Debug,
    Line: CayleyKleinPlane<Point, i64> + std::fmt::Debug,
{
    let triangle = [a_1, a_2, a_3];
    let trilateral = tri_dual(&triangle);
    let l_1 = &trilateral[0];
    assert!(l_1.incident(&triangle[1]));

    let [t_1, t_2, t_3] = tri_altitude(&triangle);
    assert!(is_perpendicular(&t_1, l_1));
    let pt_o = orthocenter(&triangle);
    assert_eq!(pt_o, t_2.meet(&t_3));
}

#[test]
fn test_ell_point() {
    let a_1 = EllipticPoint::new([13, 23, 32]);
    let a_2 = EllipticPoint::new([44, -34, 2]);
    let a_3 = EllipticPoint::new([-2, 12, 23]);
    check_ck_plane(a_1, a_2, a_3);
}

#[test]
fn test_ell_line() {
    let a_1 = EllipticLine::new([13, 23, 32]);
    let a_2 = EllipticLine::new([44, -34, 2]);
    let a_3 = EllipticLine::new([-2, 12, 23]);
    check_ck_plane(a_1, a_2, a_3);
}

#[test]
fn test_hyp_point() {
    let a_1 = HyperbolicPoint::new([13, 23, 32]);
    let a_2 = HyperbolicPoint::new([44, -34, 2]);
    let a_3 = HyperbolicPoint::new([-2, 12, 23]);
    check_ck_plane(a_1, a_2, a_3);
}

#[test]
fn test_hyp_line() {
    let a_1 = HyperbolicLine::new([13, 23, 32]);
    let a_2 = HyperbolicLine::new([44, -34, 2]);
    let a_3 = HyperbolicLine::new([-2, 12, 23]);
    check_ck_plane(a_1, a_2, a_3);
}

#[test]
fn test_myck_point() {
    let a_1 = MyCKPoint::new([13, 23, 32]);
    let a_2 = MyCKPoint::new([44, -34, 2]);
    let a_3 = MyCKPoint::new([-2, 12, 23]);
    check_ck_plane(a_1, a_2, a_3);
}

#[test]
fn test_myck_line() {
    let a_1 = MyCKLine::new([13, 23, 32]);
    let a_2 = MyCKLine::new([44, -34, 2]);
    let a_3 = MyCKLine::new([-2, 12, 23]);
    check_ck_plane(a_1, a_2, a_3);
}

#[test]
fn test_persp_point() {
    let a_1 = PerspPoint::new([13, 23, 32]);
    let a_2 = PerspPoint::new([44, -34, 2]);
    let a_3 = PerspPoint::new([-2, 12, 23]);
    check_ck_plane(a_1, a_2, a_3);
}

// #[test]
// fn test_persp_line() {
//     let a_1 = PerspLine::new([13, 23, 32]);
//     let a_2 = PerspLine::new([44, -34, 2]);
//     let a_3 = PerspLine::new([-2, 12, 23]);
//     check_ck_plane(a_1, a_2, a_3);
// }

#[test]
fn test_euclid_point() {
    let a_1 = EuclidPoint::new([13, 23, 32]);
    let a_2 = EuclidPoint::new([44, -34, 2]);
    let a_3 = EuclidPoint::new([-2, 12, 23]);
    check_ck_plane(a_1, a_2, a_3);
}

#[quickcheck]
fn test_pg_point_q(pz: i32, qz: i32) -> bool {
    let pt_p = PgPoint::new([1, 3, pz.into()]);
    let pt_q = PgPoint::new([-2, 1, qz.into()]);
    pt_p != pt_q
}

#[quickcheck]
fn test_pg_point_q2(pz: i16, qz: i16) -> bool {
    let pt_p = PgPoint::new([10, 30, pz.into()]);
    let pt_q = PgPoint::new([-20, 10, qz.into()]);
    let ln_l = pt_p.meet(&pt_q);
    ln_l == pt_q.meet(&pt_p)
}

#[quickcheck]
fn test_pg_point_q3(pz: i16, qz: i16) -> bool {
    let pt_p = PgPoint::new([10, 30, pz.into()]);
    let pt_q = PgPoint::new([-20, 10, qz.into()]);
    let ln_l = pt_p.meet(&pt_q);
    ln_l.incident(&pt_p) && ln_l.incident(&pt_q)
}
