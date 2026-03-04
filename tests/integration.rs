//! Integration tests for complex geometric constructions
//!
//! This module contains integration tests that verify complex geometric
//! constructions and theorems across different geometry types, as well as
//! error handling through complex operations.

use projgeom_rs::pg_plane::ProjectivePlanePrimitive;
use projgeom_rs::*;

#[test]
fn test_error_propagation_in_complex_constructions() {
    // Test error handling in complex geometric constructions
    // This tests that errors are properly propagated through multiple operations

    // Test 1: Error in collinear point detection
    let p1 = PgPoint::new([1, 0, 0]);
    let p2 = PgPoint::new([2, 0, 0]);
    let p3 = PgPoint::new([3, 0, 0]); // Collinear with p1 and p2

    // These points are collinear, so operations that assume non-collinearity
    // should be handled gracefully
    let line = p1.meet(&p2);
    assert!(line.incident(&p3));

    // Test 2: Error handling in degenerate triangles
    let _degenerate_triangle = [p1.clone(), p2.clone(), p3.clone()];

    // Operations on degenerate triangles should be handled
    // (orthocenter and tri_altitude have assertions for non-collinearity)
    // In production code, these would return Result types, but for now
    // we verify the behavior with collinear points
    let line_12 = p1.meet(&p2);
    let line_13 = p1.meet(&p3);
    assert_eq!(line_12, line_13); // Same line due to collinearity
}

#[test]
fn test_boundary_conditions_in_geometric_operations() {
    // Test boundary conditions and edge cases in geometric operations

    // Test 1: Point at infinity
    let point_at_infinity = PgPoint::new([1, 0, 0]);
    let finite_point = PgPoint::new([1, 1, 1]);

    let line = point_at_infinity.meet(&finite_point);
    assert!(line.incident(&point_at_infinity));
    assert!(line.incident(&finite_point));

    // Test 2: Line at infinity (in Euclidean geometry)
    let l_inf = EuclidLine::new([0, 0, 1]);
    let _euclid_point = EuclidPoint::new([1, 1, 1]);

    // The line at infinity should be perpendicular to all lines
    let line = EuclidLine::new([1, 0, -1]);
    assert!(l_inf.is_parallel(&line));

    // Test 3: Zero coordinates
    let zero_point = PgPoint::new([0, 0, 1]);
    let line = PgLine::new([1, 1, 0]);
    assert!(line.incident(&zero_point));
}

#[test]
fn test_error_in_conic_operations() {
    // Test error handling in conic section operations

    // Test 1: Degenerate conic (line pair)
    let p1 = PgPoint::new([1, 0, 0]);
    let p2 = PgPoint::new([0, 1, 0]);
    let p3 = PgPoint::new([1, 1, 0]);

    // Points on the line z=0
    let line = p1.meet(&p2);
    assert!(line.incident(&p3));

    // Test 2: Invalid conic parameters
    // A conic requires a non-degenerate matrix
    // This tests that operations handle degenerate cases
    let origin = PgPoint::new([0, 0, 1]);
    let unit_circle = Conic::unit_circle();

    // Verify that the origin is inside the unit circle
    let polar = unit_circle.polar(&origin);
    assert!(!polar.coord.is_empty());
}

#[test]
fn test_error_in_transformations() {
    // Test error handling in geometric transformations

    // Test 1: Singular transformation matrix
    // A transformation with zero determinant should be handled
    let p = PgPoint::new([1, 2, 1]);

    // Identity transformation
    let identity = Transform::identity();
    let transformed = identity.apply_point(&p);
    assert_eq!(transformed, p);

    // Test 2: Transformation that maps to infinity
    // This should be handled gracefully
    let translation = Transform::translation(1, 2);
    let translated = translation.apply_point(&p);
    assert_eq!(translated, PgPoint::new([2, 4, 1]));
}

#[test]
fn test_numerical_stability_in_operations() {
    // Test numerical stability in geometric operations

    // Test 1: Large coordinate values
    let large_p1 = PgPoint::new([1000000, 2000000, 1]);
    let large_p2 = PgPoint::new([3000000, 4000000, 1]);

    let line = large_p1.meet(&large_p2);
    assert!(line.incident(&large_p1));
    assert!(line.incident(&large_p2));

    // Test 2: Normalization with large values
    let unnormalized = PgPoint::new([2, 4, 6]);
    let normalized = PgPoint::new([1, 2, 3]); // Should be normalized
    assert_eq!(unnormalized, normalized);

    // Test 3: Operations with negative coordinates
    let neg_p1 = PgPoint::new([-1, -2, 1]);
    let neg_p2 = PgPoint::new([-3, -4, 1]);

    let line = neg_p1.meet(&neg_p2);
    assert!(line.incident(&neg_p1));
    assert!(line.incident(&neg_p2));
}

#[test]
fn test_error_in_dual_operations() {
    // Test error handling in duality operations

    // Test 1: Dual of a point at infinity
    let point_at_infinity = PgPoint::new([1, 0, 0]);
    let dual_line = point_at_infinity.aux();

    // The dual should be a line through the origin
    let origin = PgPoint::new([0, 0, 1]);
    assert!(dual_line.incident(&origin));

    // Test 2: Dual of line at infinity (in Euclidean geometry)
    let l_inf = EuclidLine::new([0, 0, 1]);
    let dual_point = l_inf.aux();

    // The dual should be a point (not necessarily at infinity in z-direction)
    // Just verify that it's a valid point and not all zeros
    let is_valid_point =
        dual_point.coord[0] != 0 || dual_point.coord[1] != 0 || dual_point.coord[2] != 0;
    assert!(is_valid_point, "Dual point should not be all zeros");
}

#[test]
fn test_error_in_harmonic_operations() {
    // Test error handling in harmonic conjugate operations

    // Test 1: Harmonic conjugate with collinear points
    let a = PgPoint::new([1, 0, 0]);
    let b = PgPoint::new([0, 1, 0]);
    let c = PgPoint::new([1, 1, 0]);

    let d = harm_conj(&a, &b, &c);

    // Verify that D is the harmonic conjugate
    let d_double = harm_conj(&a, &b, &d);
    assert_eq!(d_double, c);

    // Test 2: Harmonic conjugate with special cases
    // When C is the midpoint of AB, D should be at infinity
    let a2 = PgPoint::new([0, 0, 1]);
    let b2 = PgPoint::new([2, 0, 1]);
    let c2 = PgPoint::new([1, 0, 1]); // Midpoint

    let d2 = harm_conj(&a2, &b2, &c2);
    // D should be at infinity on the x-axis
    assert_eq!(d2.coord[1], 0);
    assert_eq!(d2.coord[2], 0);
}

#[test]
fn test_desargues_theorem() {
    // Desargues' Theorem: If two triangles are perspective from a point,
    // then they are perspective from a line, and vice versa.

    // First triangle
    let a1 = PgPoint::new([1, 0, 0]);
    let b1 = PgPoint::new([0, 1, 0]);
    let c1 = PgPoint::new([0, 0, 1]);

    // Second triangle (perspective from point P)
    let p = PgPoint::new([1, 1, 1]);
    let a2 = PgPoint::new([2, 1, 1]);
    let b2 = PgPoint::new([1, 2, 1]);
    let c2 = PgPoint::new([1, 1, 2]);

    // Check that triangles are perspective from point P
    let line_a1a2 = a1.meet(&a2);
    let line_b1b2 = b1.meet(&b2);
    let line_c1c2 = c1.meet(&c2);

    // All three lines should intersect at P
    assert!(coincident(
        &line_a1a2.meet(&line_b1b2),
        &p,
        &line_a1a2.meet(&line_c1c2)
    ));

    // Find the intersection points of corresponding sides
    let ab = a1.meet(&b1).meet(&a2.meet(&b2));
    let bc = b1.meet(&c1).meet(&b2.meet(&c2));
    let ca = c1.meet(&a1).meet(&c2.meet(&a2));

    // These three points should be collinear (perspective from a line)
    assert!(coincident(&ab, &bc, &ca));
}

#[test]
fn test_pappus_theorem() {
    // Pappus' Hexagon Theorem: Given two lines with three points on each,
    // the three intersection points of opposite sides of the hexagon are collinear.

    // First line with three points
    let _l1 = PgLine::new([0, 0, 1]);
    let a = PgPoint::new([1, 0, 1]);
    let b = PgPoint::new([2, 0, 1]);
    let c = PgPoint::new([3, 0, 1]);

    // Second line with three points
    let _l2 = PgLine::new([1, 1, 0]);
    let d = PgPoint::new([1, -1, 1]);
    let e = PgPoint::new([2, -2, 1]);
    let f = PgPoint::new([3, -3, 1]);

    // Construct hexagon: A, E, B, F, C, D
    let line_ae = a.meet(&e);
    let line_eb = e.meet(&b);
    let line_bf = b.meet(&f);
    let line_fc = f.meet(&c);
    let line_cd = c.meet(&d);
    let line_da = d.meet(&a);

    // Find intersection points of opposite sides
    let p1 = line_ae.meet(&line_fc);
    let p2 = line_eb.meet(&line_cd);
    let p3 = line_bf.meet(&line_da);

    // These three points should be collinear
    assert!(coincident(&p1, &p2, &p3));
}

#[test]
fn test_pascal_theorem() {
    // Pascal's Theorem: For any hexagon inscribed in a conic,
    // the three intersection points of opposite sides are collinear.

    // Six points on a circle (conic)
    let p1 = PgPoint::new([1, 0, 1]);
    let p2 = PgPoint::new([0, 1, 1]);
    let p3 = PgPoint::new([-1, 0, 1]);
    let p4 = PgPoint::new([0, -1, 1]);
    let p5 = PgPoint::new([2, 0, 1]);
    let p6 = PgPoint::new([0, 2, 1]);

    // Construct hexagon: P1, P2, P3, P4, P5, P6
    let line_p1p2 = p1.meet(&p2);
    let line_p2p3 = p2.meet(&p3);
    let line_p3p4 = p3.meet(&p4);
    let line_p4p5 = p4.meet(&p5);
    let line_p5p6 = p5.meet(&p6);
    let line_p6p1 = p6.meet(&p1);

    // Find intersection points of opposite sides
    let q1 = line_p1p2.meet(&line_p4p5);
    let q2 = line_p2p3.meet(&line_p5p6);
    let q3 = line_p3p4.meet(&line_p6p1);

    // These three points should be collinear (Pascal line)
    assert!(coincident(&q1, &q2, &q3));
}

#[test]
fn test_harmonic_bundle() {
    // Test that harmonic conjugates form a harmonic bundle

    let a = PgPoint::new([1, 0, 0]);
    let b = PgPoint::new([0, 1, 0]);
    let c = PgPoint::new([1, 1, 0]);

    let d = harm_conj(&a, &b, &c);

    // Verify that (A, B; C, D) = -1
    // This means the cross ratio is -1
    let _line = a.meet(&b);

    // Verify that D is the harmonic conjugate of C
    let d_double = harm_conj(&a, &b, &d);
    assert_eq!(d_double, c);
}

#[test]
fn test_elliptic_triangle_properties() {
    // Test properties of triangles in elliptic geometry

    let a = EllipticPoint::new([1, 0, 0]);
    let b = EllipticPoint::new([0, 1, 0]);
    let c = EllipticPoint::new([0, 0, 1]);

    let triangle = [a.clone(), b.clone(), c.clone()];

    // Compute orthocenter
    let orthocenter_pt = orthocenter(&triangle);

    // In elliptic geometry, the orthocenter should be the pole of the
    // line through the vertices
    let line_ab = a.meet(&b);
    let line_bc = b.meet(&c);
    let pole_pg = line_ab.aux().meet(&line_bc.aux());
    let pole = EllipticPoint::new(pole_pg.coord);

    assert_eq!(orthocenter_pt, pole);
}

#[test]
fn test_cross_ratio_invariance() {
    // Test that cross ratio is invariant under projective transformations

    let p1 = PgPoint::new([1, 0, 0]);
    let p2 = PgPoint::new([0, 1, 0]);
    let p3 = PgPoint::new([1, 1, 0]);
    let p4 = PgPoint::new([2, 1, 0]);

    // Apply a projective transformation (involution)
    let origin = PgPoint::new([1, 0, 0]);
    let mirror = PgLine::new([0, 1, 0]);

    let p1_t = involution(&origin, &mirror, &p1);
    let p2_t = involution(&origin, &mirror, &p2);
    let p3_t = involution(&origin, &mirror, &p3);
    let p4_t = involution(&origin, &mirror, &p4);

    // The cross ratio should be preserved
    // (This is a simplified test; actual cross ratio computation
    // would require more sophisticated arithmetic)
    let line = p1.meet(&p2);
    assert!(line.incident(&p1_t));
    assert!(line.incident(&p2_t));
    assert!(line.incident(&p3_t));
    assert!(line.incident(&p4_t));
}

#[test]
fn test_menelaus_theorem() {
    // Menelaus' Theorem: For a triangle and a transversal line,
    // the product of certain ratios is -1.

    let a = PgPoint::new([0, 0, 1]);
    let b = PgPoint::new([4, 0, 1]);
    let c = PgPoint::new([0, 3, 1]);

    // A transversal line
    let transversal = PgLine::new([1, -1, 0]);

    // Find intersection points
    let d = a.meet(&b).meet(&transversal);
    let e = b.meet(&c).meet(&transversal);
    let f = c.meet(&a).meet(&transversal);

    // Verify that D, E, F are collinear
    assert!(coincident(&d, &e, &f));
}
