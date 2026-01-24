//! Elliptic Geometry Example
//!
//! This example demonstrates operations in elliptic geometry,
//! including perpendicular lines, orthocenters, and altitudes.

use projgeom_rs::{EllipticPoint, EllipticLine, orthocenter, tri_altitude, tri_dual, is_perpendicular};
use projgeom_rs::pg_plane::ProjectivePlanePrimitive;
use projgeom_rs::ck_plane::{CayleyKleinPlane, CayleyKleinPlanePrimitive};

fn main() {
    println!("=== Elliptic Geometry Example ===\n");

    // Create points in elliptic geometry
    let a1 = EllipticPoint::new([1, 0, 0]);
    let a2 = EllipticPoint::new([0, 1, 0]);
    let a3 = EllipticPoint::new([0, 0, 1]);

    println!("Point a1: {:?}", a1.coord);
    println!("Point a2: {:?}", a2.coord);
    println!("Point a3: {:?}", a3.coord);

    // Compute perpendicular line
    let perp_line = a1.perp();
    println!("\nPerpendicular to a1: {:?}", perp_line.coord);
    println!("a1 incident with perp: {}", a1.incident(&perp_line));

    // Compute orthocenter of a triangle
    let triangle = [a1, a2, a3];
    let orthocenter_pt = orthocenter(&triangle);
    println!("\nOrthocenter: {:?}", orthocenter_pt.coord);

    // Compute altitudes
    let altitudes = tri_altitude(&triangle);
    println!("\nAltitudes:");
    for (i, alt) in altitudes.iter().enumerate() {
        println!("  Altitude {}: {:?}", i, alt.coord);
    }

    // Compute trilateral (dual of triangle)
    let trilateral = tri_dual(&triangle);
    println!("\nTrilateral (sides of triangle):");
    for (i, side) in trilateral.iter().enumerate() {
        println!("  Side {}: {:?}", i, side.coord);
    }

    // Check perpendicularity
    println!("\nChecking perpendicularity:");
    for (i, (alt, side)) in altitudes.iter().zip(trilateral.iter()).enumerate() {
        let perp = is_perpendicular(alt, side);
        println!("  Altitude {} ⟂ Side {}: {}", i, i, perp);
    }

    // Demonstrate involution property of perpendicular
    let point = EllipticPoint::new([1, 2, 3]);
    let line = point.perp();
    let double_perp = line.perp();
    println!("\nOriginal point: {:?}", point.coord);
    println!("Perp(perp(point)): {:?}", double_perp.coord);
    println!("Equal: {}", point == double_perp);

    println!("\n=== Example Complete ===");
}
