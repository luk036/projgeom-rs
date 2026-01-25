//! Projective Geometry Example
//!
//! This example demonstrates basic operations in projective geometry,
//! including point-line incidence, line intersection, and parametrization.

use projgeom_rs::pg_plane::ProjectivePlanePrimitive;
use projgeom_rs::{PgLine, PgPoint, ProjectivePlane};

fn main() {
    println!("=== Projective Geometry Example ===\n");

    // Create two points in projective space
    let p1 = PgPoint::new([1, 2, 3]);
    let p2 = PgPoint::new([4, 5, 6]);

    println!("Point p1: {:?}", p1.coord);
    println!("Point p2: {:?}", p2.coord);

    // Find the line through two points (meet operation)
    let line = p1.meet(&p2);
    println!("\nLine through p1 and p2: {:?}", line.coord);

    // Check if points lie on the line
    println!("p1 on line: {}", line.incident(&p1));
    println!("p2 on line: {}", line.incident(&p2));

    // Parametrize a point on the line
    let p3 = p1.parametrize(2, &p2, 3);
    println!("\nParametrized point p3: {:?}", p3.coord);
    println!("p3 on line: {}", line.incident(&p3));

    // Find the intersection of two lines
    let l1 = PgLine::new([1, 0, 0]);
    let l2 = PgLine::new([0, 1, 0]);
    let intersection = l1.meet(&l2);
    println!("\nLine l1: {:?}", l1.coord);
    println!("Line l2: {:?}", l2.coord);
    println!("Intersection: {:?}", intersection.coord);

    // Demonstrate duality
    let point = PgPoint::new([1, 2, 1]);
    let polar = point.aux();
    println!("\nPoint: {:?}", point.coord);
    println!("Polar line: {:?}", polar.coord);
    println!("Point incident with polar: {}", point.incident(&polar));

    // Check collinearity
    let p4 = PgPoint::new([2, 4, 2]);
    println!("\nChecking if p1, p2, p4 are collinear: {}", {
        let line = p1.meet(&p2);
        line.incident(&p4)
    });

    println!("\n=== Example Complete ===");
}
