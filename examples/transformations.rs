//! Projective transformations example
//!
//! Demonstrates projective transformations on points and lines.

use fractions::Fraction;
use projgeom_rs::cross_ratio::projective_transform_point;
use projgeom_rs::pg_object::PgPoint;

fn main() {
    println!("=== Projective Transformations Example ===\n");

    let point = PgPoint::new([1, 2, 1]);
    println!("Original point: {:?}", point.coord);

    let identity = [
        [
            Fraction::<i64>::new(1, 1),
            Fraction::<i64>::new(0, 1),
            Fraction::<i64>::new(0, 1),
        ],
        [
            Fraction::<i64>::new(0, 1),
            Fraction::<i64>::new(1, 1),
            Fraction::<i64>::new(0, 1),
        ],
        [
            Fraction::<i64>::new(0, 1),
            Fraction::<i64>::new(0, 1),
            Fraction::<i64>::new(1, 1),
        ],
    ];

    let transformed = projective_transform_point(&identity, &point);
    println!("After identity transform: {:?}", transformed.coord);

    let scale_2 = [
        [
            Fraction::<i64>::new(2, 1),
            Fraction::<i64>::new(0, 1),
            Fraction::<i64>::new(0, 1),
        ],
        [
            Fraction::<i64>::new(0, 1),
            Fraction::<i64>::new(2, 1),
            Fraction::<i64>::new(0, 1),
        ],
        [
            Fraction::<i64>::new(0, 1),
            Fraction::<i64>::new(0, 1),
            Fraction::<i64>::new(1, 1),
        ],
    ];

    let scaled = projective_transform_point(&scale_2, &point);
    println!("After scale(2) transform: {:?}", scaled.coord);

    println!("\n=== Example Complete ===");
}
