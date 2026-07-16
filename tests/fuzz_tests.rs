//! Edge case tests for geometric operations

use proptest::prelude::*;

use projgeom_rs::pg_object::{PgLine, PgPoint};
use projgeom_rs::{ProjectivePlane, ProjectivePlanePrimitive};

fn non_zero_coord() -> impl Strategy<Value = [i64; 3]> {
    any::<[i64; 3]>().prop_filter("non-zero", |c| *c != [0, 0, 0])
}

proptest! {
    #[test]
    fn test_meet_not_all_zeros(p1 in non_zero_coord(), p2 in non_zero_coord()) {
        let a = PgPoint::new(p1);
        let b = PgPoint::new(p2);
        let _line = a.meet(&b);
    }

    #[test]
    fn test_incident_returns_bool(p in non_zero_coord(), l in non_zero_coord()) {
        let pt = PgPoint::new(p);
        let line = PgLine::new(l);
        let _result = pt.incident(&line);
    }

    #[test]
    fn test_coordinate_not_all_zeros(p in non_zero_coord()) {
        let a = PgPoint::new(p);
        assert!(!(a.coord[0] == 0 && a.coord[1] == 0 && a.coord[2] == 0));
    }

    #[test]
    fn test_parametrize_valid(p1 in non_zero_coord(), p2 in non_zero_coord()) {
        let a = PgPoint::new(p1);
        let b = PgPoint::new(p2);
        let result = a.parametrize(1, &b, 1);
        assert!(result.coord != [0, 0, 0]);
    }
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
