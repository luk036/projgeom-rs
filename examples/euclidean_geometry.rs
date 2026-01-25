//! Euclidean Geometry Example
//!
//! This example demonstrates operations in Euclidean geometry,
//! including reflections and perpendicular lines.

use projgeom_rs::{EuclidPoint, EuclidLine, reflect, orthocenter, tri_altitude};
use projgeom_rs::pg_plane::ProjectivePlanePrimitive;
use projgeom_rs::ck_plane::CayleyKleinPlanePrimitive;

fn main() {
    println!("=== Euclidean Geometry Example ===\n");

    // Create points in Euclidean geometry
    let p1 = EuclidPoint::new([1, 2, 1]);
    let p2 = EuclidPoint::new([3, 4, 1]);
    let p3 = EuclidPoint::new([5, 1, 1]);

    println!("Point p1: {:?}", p1.coord);
    println!("Point p2: {:?}", p2.coord);
    println!("Point p3: {:?}", p3.coord);

    // Find the line through two points
    let line = p1.meet(&p2);
    println!("\nLine through p1 and p2: {:?}", line.coord);

    // Reflect a point across a line
    let mirror = EuclidLine::new([1, -1, 0]);
    let reflected = reflect(&mirror, &p1);
    println!("\nMirror line: {:?}", mirror.coord);
    println!("Original point: {:?}", p1.coord);
    println!("Reflected point: {:?}", reflected.coord);

    // Check that reflection is an involution
    let reflected_twice = reflect(&mirror, &reflected);
    println!("Reflected twice: {:?}", reflected_twice.coord);
    println!("Equals original: {}", reflected_twice == p1);

    // Create a triangle and compute orthocenter
    let triangle = [p1, p2, p3];
    let orthocenter_pt = orthocenter(&triangle);
    println!("\nOrthocenter: {:?}", orthocenter_pt.coord);

    // Compute altitudes
    let altitudes = tri_altitude(&triangle);
    println!("\nAltitudes:");
    for (i, alt) in altitudes.iter().enumerate() {
        println!("  Altitude {}: {:?}", i, alt.coord);
    }

    // Demonstrate perpendicular
    let point = EuclidPoint::new([1, 1, 1]);
    let perp = point.perp();
    println!("\nPoint: {:?}", point.coord);
    println!("Perpendicular line: {:?}", perp.coord);

    println!("\n=== Example Complete ===");
}