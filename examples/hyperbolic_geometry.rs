//! Hyperbolic Geometry Example
//!
//! This example demonstrates operations in hyperbolic geometry,
//! including perpendicular lines and triangle properties.

use projgeom_rs::{HyperbolicPoint, HyperbolicLine, CayleyKleinPlane, orthocenter, tri_altitude};
use projgeom_rs::pg_plane::ProjectivePlanePrimitive;
use projgeom_rs::ck_plane::CayleyKleinPlanePrimitive;

fn main() {
    println!("=== Hyperbolic Geometry Example ===\n");

    // Create points in hyperbolic geometry
    // In hyperbolic geometry, we use points with z > 0
    let p1 = HyperbolicPoint::new([1, 0, 1]);
    let p2 = HyperbolicPoint::new([0, 1, 1]);
    let p3 = HyperbolicPoint::new([1, 1, 2]);

    println!("Point p1: {:?}", p1.coord);
    println!("Point p2: {:?}", p2.coord);
    println!("Point p3: {:?}", p3.coord);

    // Find the line through two points
    let line = p1.meet(&p2);
    println!("\nLine through p1 and p2: {:?}", line.coord);
    println!("p1 on line: {}", line.incident(&p1));
    println!("p2 on line: {}", line.incident(&p2));

    // Compute perpendicular
    let perp = p1.perp();
    println!("\nPerpendicular to p1: {:?}", perp.coord);

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

    // Demonstrate involution property
    let point = HyperbolicPoint::new([2, 1, 1]);
    let line = point.perp();
    let double_perp = line.perp();
    println!("\nOriginal point: {:?}", point.coord);
    println!("Perp(perp(point)): {:?}", double_perp.coord);
    println!("Equal: {}", point == double_perp);

    println!("\n=== Example Complete ===");
}