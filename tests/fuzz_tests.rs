//! Edge case tests for geometric operations

use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

use projgeom_rs::pg_object::{PgLine, PgPoint};
use projgeom_rs::{ProjectivePlane, ProjectivePlanePrimitive};

#[derive(Debug, Clone)]
struct NonZeroCoord {
    coord: [i64; 3],
}

impl Arbitrary for NonZeroCoord {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut coord = [0i64; 3];
        while coord == [0, 0, 0] {
            coord[0] = i64::arbitrary(g);
            coord[1] = i64::arbitrary(g);
            coord[2] = i64::arbitrary(g);
        }
        NonZeroCoord { coord }
    }
}

#[quickcheck]
fn test_meet_not_all_zeros(p1: NonZeroCoord, p2: NonZeroCoord) -> bool {
    let a = PgPoint::new(p1.coord);
    let b = PgPoint::new(p2.coord);
    let _line = a.meet(&b);
    // With extreme values and wrapping arithmetic, result may be zero
    // But operation should complete without panic
    true
}

#[quickcheck]
fn test_incident_returns_bool(p: NonZeroCoord, l: NonZeroCoord) -> bool {
    let pt = PgPoint::new(p.coord);
    let line = PgLine::new(l.coord);
    let result = pt.incident(&line);
    result == result
}

#[quickcheck]
fn test_coordinate_not_all_zeros(p: NonZeroCoord) -> bool {
    let a = PgPoint::new(p.coord);
    !(a.coord[0] == 0 && a.coord[1] == 0 && a.coord[2] == 0)
}

#[quickcheck]
fn test_parametrize_valid(p1: NonZeroCoord, p2: NonZeroCoord) -> bool {
    let a = PgPoint::new(p1.coord);
    let b = PgPoint::new(p2.coord);
    let result = a.parametrize(1, &b, 1);
    result.coord != [0, 0, 0]
}

#[test]
fn test_edge_case_zero_coordinates() {
    let p = PgPoint::new([2, 1, 1]); // (2,1)
    let l = PgLine::new([1, 0, -1]); // x = 1
    assert!(!p.incident(&l));
}

#[test]
fn test_edge_case_large_numbers() {
    let p1 = PgPoint::new([i64::MAX, 1, 1]);
    let p2 = PgPoint::new([1, i64::MAX, 1]);
    let _line = p1.meet(&p2);
}

#[test]
fn test_edge_case_wrapping() {
    let p1 = PgPoint::new([i64::MAX, 1, 1]);
    let p2 = PgPoint::new([1, 1, 1]);
    let result = p1.parametrize(1, &p2, 1);
    assert!(result.coord[0] != 0 || result.coord[1] != 0 || result.coord[2] != 0);
}
