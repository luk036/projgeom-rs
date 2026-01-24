//! Harmonic Conjugates Example
//!
//! This example demonstrates the computation of harmonic conjugates
//! in projective geometry.

use projgeom_rs::{PgPoint, harm_conj, coincident};
use projgeom_rs::pg_plane::{ProjectivePlane, ProjectivePlanePrimitive};

fn main() {
    println!("=== Harmonic Conjugates Example ===\n");

    // Create three collinear points
    let a = PgPoint::new([1, 0, 0]);
    let b = PgPoint::new([0, 1, 0]);
    let c = PgPoint::new([1, 1, 0]);

    println!("Point a: {:?}", a.coord);
    println!("Point b: {:?}", b.coord);
    println!("Point c: {:?}", c.coord);

    // Check collinearity
    println!("\nPoints are collinear: {}", coincident(&a, &b, &c));

    // Compute harmonic conjugate
    let d = harm_conj(&a, &b, &c);
    println!("\nHarmonic conjugate d: {:?}", d.coord);

    // Verify that d is the harmonic conjugate
    let d_double = harm_conj(&a, &b, &d);
    println!("harm_conj(a, b, d): {:?}", d_double.coord);
    println!("Equals c: {}", d_double == c);

    // Demonstrate the harmonic division property
    println!("\nHarmonic division: (a, b; c, d) = -1");
    println!("This means c and d divide segment ab harmonically");

    // Another example with different points
    let p1 = PgPoint::new([1, 2, 3]);
    let p2 = PgPoint::new([4, 5, 6]);
    let p3 = p1.parametrize(1, &p2, 2);

    println!("\n--- Another Example ---");
    println!("Point p1: {:?}", p1.coord);
    println!("Point p2: {:?}", p2.coord);
    println!("Point p3: {:?}", p3.coord);

    let p4 = harm_conj(&p1, &p2, &p3);
    println!("Harmonic conjugate p4: {:?}", p4.coord);

    let p4_double = harm_conj(&p1, &p2, &p4);
    println!("harm_conj(p1, p2, p4): {:?}", p4_double.coord);
    println!("Equals p3: {}", p4_double == p3);

    println!("\n=== Example Complete ===");
}