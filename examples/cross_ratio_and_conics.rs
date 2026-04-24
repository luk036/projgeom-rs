//! Cross ratio and conics example
//!
//! This example demonstrates cross-ratio computation and conic sections.

use projgeom_rs::pg_object::PgPoint;
use projgeom_rs::{conic::Conic, cross_ratio, ConicType};

fn main() {
    println!("=== Cross Ratio and Conics Example ===\n");

    let a = PgPoint::new([0, 0, 1]);
    let b = PgPoint::new([2, 0, 1]);
    let c = PgPoint::new([1, 0, 1]);
    let d = PgPoint::new([3, 0, 1]);

    let ratio = cross_ratio(&a, &b, &c, &d);
    println!("Cross ratio (A, B; C, D) = {:?}", ratio);

    println!("\n--- Conic Sections ---\n");

    let circle = Conic::unit_circle();
    println!("Unit circle type: {:?}", circle.conic_type());
    assert_eq!(circle.conic_type(), ConicType::Ellipse);

    let p1 = PgPoint::new([1, 0, 1]);
    let p2 = PgPoint::new([0, 1, 1]);
    let p_on_circle = circle.contains(&p1);
    let p2_on_circle = circle.contains(&p2);
    println!("Point (1,0,1) on circle: {}", p_on_circle);
    println!("Point (0,1,1) on circle: {}", p2_on_circle);

    let polar = circle.polar(&p1);
    println!("Polar of (1,0,1): {:?}", polar.coord);

    println!("\n=== Example Complete ===");
}
