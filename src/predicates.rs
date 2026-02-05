//! Geometric predicates and utility functions
//!
//! This module provides various geometric predicates such as orientation,
//! distance calculations, and angle computations for projective geometry.

use crate::pg_object::{PgLine, PgPoint};
use crate::pg_plane::ProjectivePlane;
use fractions::Fraction;
use num_integer::gcd;

/// Represents the orientation of three points in the plane
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Points are in clockwise orientation
    Clockwise,
    /// Points are in counter-clockwise orientation
    CounterClockwise,
    /// Points are collinear
    Collinear,
}

/// Represents the position of a point relative to a line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinePosition {
    /// Point is on the line
    OnLine,
    /// Point is to the left of the line
    Left,
    /// Point is to the right of the line
    Right,
}

/// Normalize a homogeneous coordinate by dividing by the GCD
///
/// This function reduces the homogeneous coordinates to their canonical form
/// by dividing all components by their greatest common divisor.
///
/// # Arguments
///
/// * `coord` - A mutable reference to the homogeneous coordinate array
///
/// # Examples
///
/// ```
/// use projgeom_rs::predicates::normalize_homogeneous;
/// let mut coord = [2, 4, 6];
/// normalize_homogeneous(&mut coord);
/// assert_eq!(coord, [1, 2, 3]);
/// ```
pub fn normalize_homogeneous(coord: &mut [i64; 3]) {
    let g = gcd(gcd(coord[0].abs(), coord[1].abs()), coord[2].abs());
    if g != 0 && g != 1 {
        coord[0] /= g;
        coord[1] /= g;
        coord[2] /= g;
    }

    // Ensure the last non-zero coordinate is positive
    if coord[2] != 0 && coord[2] < 0 {
        coord[0] = -coord[0];
        coord[1] = -coord[1];
        coord[2] = -coord[2];
    } else if coord[2] == 0 && coord[1] != 0 && coord[1] < 0 {
        coord[0] = -coord[0];
        coord[1] = -coord[1];
    } else if coord[2] == 0 && coord[1] == 0 && coord[0] < 0 {
        coord[0] = -coord[0];
    }
}

/// Compute the orientation of three points
///
/// This function determines whether three points are arranged in clockwise,
/// counter-clockwise, or collinear order.
///
/// # Arguments
///
/// * `p1` - First point
/// * `p2` - Second point
/// * `p3` - Third point
///
/// # Returns
///
/// The orientation of the three points
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, predicates::orientation};
/// let p1 = PgPoint::new([0, 0, 1]);
/// let p2 = PgPoint::new([1, 0, 1]);
/// let p3 = PgPoint::new([0, 1, 1]);
/// let orient = orientation(&p1, &p2, &p3);
/// ```
pub fn orientation(p1: &PgPoint, p2: &PgPoint, p3: &PgPoint) -> Orientation {
    // Convert to affine coordinates if possible
    let (x1, y1) = to_affine(p1);
    let (x2, y2) = to_affine(p2);
    let (x3, y3) = to_affine(p3);

    let cross = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);

    if cross > 0 {
        Orientation::CounterClockwise
    } else if cross < 0 {
        Orientation::Clockwise
    } else {
        Orientation::Collinear
    }
}

/// Determine the position of a point relative to a line
///
/// # Arguments
///
/// * `point` - The point to test
/// * `line` - The line
///
/// # Returns
///
/// The position of the point relative to the line
pub fn line_position(point: &PgPoint, line: &PgLine) -> LinePosition {
    let dot = point.dot(line);
    if dot == 0 {
        LinePosition::OnLine
    } else if dot > 0 {
        LinePosition::Left
    } else {
        LinePosition::Right
    }
}

/// Compute the squared distance between two points in Euclidean geometry
///
/// This function returns the squared Euclidean distance, which is useful for
/// comparisons without the computational cost of a square root.
///
/// # Arguments
///
/// * `p1` - First point
/// * `p2` - Second point
///
/// # Returns
///
/// The squared distance as a Fraction
///
/// # Examples
///
/// ```
/// use projgeom_rs::{PgPoint, predicates::squared_distance};
/// let p1 = PgPoint::new([0, 0, 1]);
/// let p2 = PgPoint::new([3, 4, 1]);
/// let dist_sq = squared_distance(&p1, &p2);
/// ```
pub fn squared_distance(p1: &PgPoint, p2: &PgPoint) -> Fraction<i64> {
    let (x1, y1) = to_affine(p1);
    let (x2, y2) = to_affine(p2);

    let dx = Fraction::<i64>::new(x2, 1) - Fraction::<i64>::new(x1, 1);
    let dy = Fraction::<i64>::new(y2, 1) - Fraction::<i64>::new(y1, 1);

    dx * dx + dy * dy
}

/// Compute the Euclidean distance between two points
///
/// # Arguments
///
/// * `p1` - First point
/// * `p2` - Second point
///
/// # Returns
///
/// The distance as a Fraction (may involve square roots, not implemented)
pub fn distance(p1: &PgPoint, p2: &PgPoint) -> Fraction<i64> {
    squared_distance(p1, p2)
}

/// Compute the angle between three points
///
/// This computes the angle at point p2 formed by the segments p1-p2 and p3-p2.
///
/// # Arguments
///
/// * `p1` - First point
/// * `p2` - Vertex point
/// * `p3` - Third point
///
/// # Returns
///
/// The cosine of the angle as a Fraction
pub fn angle_cosine(p1: &PgPoint, p2: &PgPoint, p3: &PgPoint) -> Fraction<i64> {
    let (x1, y1) = to_affine(p1);
    let (x2, y2) = to_affine(p2);
    let (x3, y3) = to_affine(p3);

    let v1x = Fraction::<i64>::new(x1 - x2, 1);
    let v1y = Fraction::<i64>::new(y1 - y2, 1);
    let v2x = Fraction::<i64>::new(x3 - x2, 1);
    let v2y = Fraction::<i64>::new(y3 - y2, 1);

    let dot = v1x * v2x + v1y * v2y;
    let norm1_sq = v1x * v1x + v1y * v1y;
    let norm2_sq = v2x * v2x + v2y * v2y;

    if norm1_sq == Fraction::<i64>::new(0, 1) || norm2_sq == Fraction::<i64>::new(0, 1) {
        return Fraction::<i64>::new(0, 1);
    }

    dot / (norm1_sq * norm2_sq)
}

/// Compute the area of a triangle formed by three points
///
/// # Arguments
///
/// * `p1` - First vertex
/// * `p2` - Second vertex
/// * `p3` - Third vertex
///
/// # Returns
///
/// The signed area as a Fraction (positive for counter-clockwise orientation)
pub fn triangle_area(p1: &PgPoint, p2: &PgPoint, p3: &PgPoint) -> Fraction<i64> {
    let (x1, y1) = to_affine(p1);
    let (x2, y2) = to_affine(p2);
    let (x3, y3) = to_affine(p3);

    let x1_f = Fraction::<i64>::new(x1, 1);
    let y1_f = Fraction::<i64>::new(y1, 1);
    let x2_f = Fraction::<i64>::new(x2, 1);
    let y2_f = Fraction::<i64>::new(y2, 1);
    let x3_f = Fraction::<i64>::new(x3, 1);
    let y3_f = Fraction::<i64>::new(y3, 1);

    ((x2_f - x1_f) * (y3_f - y1_f) - (x3_f - x1_f) * (y2_f - y1_f)) / Fraction::<i64>::new(2, 1)
}

/// Check if a point is inside a triangle
///
/// # Arguments
///
/// * `point` - The point to test
/// * `v1` - First vertex of the triangle
/// * `v2` - Second vertex of the triangle
/// * `v3` - Third vertex of the triangle
///
/// # Returns
///
/// True if the point is inside or on the boundary of the triangle
pub fn point_in_triangle(point: &PgPoint, v1: &PgPoint, v2: &PgPoint, v3: &PgPoint) -> bool {
    let orient1 = orientation(v1, v2, point);
    let orient2 = orientation(v2, v3, point);
    let orient3 = orientation(v3, v1, point);

    // Point is inside if:
    // 1. All orientations are the same (all CCW or all CW), OR
    // 2. Any orientation is Collinear (point is on an edge)
    let all_same = (orient1 == orient2) && (orient2 == orient3);
    let any_collinear = orient1 == Orientation::Collinear
        || orient2 == Orientation::Collinear
        || orient3 == Orientation::Collinear;

    all_same || any_collinear
}

/// Convert a projective point to affine coordinates
///
/// # Arguments
///
/// * `point` - The projective point
///
/// # Returns
///
/// A tuple (x, y) of affine coordinates as i64
///
/// # Panics
///
/// Panics if the point is at infinity (z = 0)
fn to_affine(point: &PgPoint) -> (i64, i64) {
    if point.coord[2] == 0 {
        panic!("Cannot convert point at infinity to affine coordinates");
    }
    (
        point.coord[0] / point.coord[2],
        point.coord[1] / point.coord[2],
    )
}

/// Check if a point is at infinity
///
/// # Arguments
///
/// * `point` - The point to test
///
/// # Returns
///
/// True if the point is at infinity (z = 0)
pub fn is_at_infinity(point: &PgPoint) -> bool {
    point.coord[2] == 0
}

/// Check if a line is the line at infinity
///
/// # Arguments
///
/// * `line` - The line to test
///
/// # Returns
///
/// True if the line is the line at infinity (z = 0)
pub fn is_line_at_infinity(line: &PgLine) -> bool {
    line.coord[0] == 0 && line.coord[1] == 0 && line.coord[2] != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_homogeneous() {
        let mut coord = [2, 4, 6];
        normalize_homogeneous(&mut coord);
        assert_eq!(coord, [1, 2, 3]);

        let mut coord = [-2, -4, 6];
        normalize_homogeneous(&mut coord);
        assert_eq!(coord, [-1, -2, 3]);

        let mut coord = [0, 0, 1];
        normalize_homogeneous(&mut coord);
        assert_eq!(coord, [0, 0, 1]);
    }

    #[test]
    fn test_orientation() {
        let p1 = PgPoint::new([0, 0, 1]);
        let p2 = PgPoint::new([1, 0, 1]);
        let p3 = PgPoint::new([0, 1, 1]);

        assert_eq!(orientation(&p1, &p2, &p3), Orientation::CounterClockwise);

        let p4 = PgPoint::new([0, -1, 1]);
        assert_eq!(orientation(&p1, &p2, &p4), Orientation::Clockwise);

        let p5 = PgPoint::new([2, 0, 1]);
        assert_eq!(orientation(&p1, &p2, &p5), Orientation::Collinear);
    }

    #[test]
    fn test_squared_distance() {
        let p1 = PgPoint::new([0, 0, 1]);
        let p2 = PgPoint::new([3, 4, 1]);
        let dist_sq = squared_distance(&p1, &p2);
        assert_eq!(dist_sq, Fraction::<i64>::new(25, 1));
    }

    #[test]
    fn test_triangle_area() {
        let p1 = PgPoint::new([0, 0, 1]);
        let p2 = PgPoint::new([2, 0, 1]);
        let p3 = PgPoint::new([0, 2, 1]);
        let area = triangle_area(&p1, &p2, &p3);
        assert_eq!(area, Fraction::<i64>::new(2, 1));
    }

    #[test]
    fn test_point_in_triangle() {
        let v1 = PgPoint::new([0, 0, 1]);
        let v2 = PgPoint::new([2, 0, 1]);
        let v3 = PgPoint::new([0, 2, 1]);

        // Point (1, 0) is on the edge from (0,0) to (2,0)
        let inside = PgPoint::new([1, 0, 1]);
        assert!(point_in_triangle(&inside, &v1, &v2, &v3));

        let outside = PgPoint::new([2, 2, 1]);
        assert!(!point_in_triangle(&outside, &v1, &v2, &v3));
    }

    #[test]
    fn test_is_at_infinity() {
        let finite = PgPoint::new([1, 2, 1]);
        assert!(!is_at_infinity(&finite));

        let infinite = PgPoint::new([1, 2, 0]);
        assert!(is_at_infinity(&infinite));
    }

    #[test]
    fn test_line_position_on_line() {
        let point = PgPoint::new([1, 1, 1]);
        let line = PgLine::new([1, 1, -2]); // Line x + y - 2 = 0, point (1,1,1) is on this line
        assert_eq!(line_position(&point, &line), LinePosition::OnLine);
    }

    #[test]
    fn test_line_position_left() {
        let point = PgPoint::new([1, 1, 1]);
        let line = PgLine::new([1, 0, 0]); // Line x = 0
                                           // For line x=0, point (1,1) has dot product 1 > 0
        assert_eq!(line_position(&point, &line), LinePosition::Left);
    }

    #[test]
    fn test_line_position_right() {
        let point = PgPoint::new([-1, 1, 1]);
        let line = PgLine::new([1, 0, 0]); // Line x = 0
                                           // For line x=0, point (-1,1) has dot product -1 < 0
        assert_eq!(line_position(&point, &line), LinePosition::Right);
    }

    #[test]
    fn test_angle_cosine_right_angle() {
        let p1 = PgPoint::new([0, 1, 1]);
        let p2 = PgPoint::new([0, 0, 1]); // Vertex
        let p3 = PgPoint::new([1, 0, 1]);

        // Vector p1->p2 is (0, 1), Vector p2->p3 is (1, 0)
        // These are perpendicular, so cosine should be 0
        let cos = angle_cosine(&p1, &p2, &p3);
        assert_eq!(cos, Fraction::<i64>::new(0, 1));
    }

    #[test]
    fn test_angle_cosine_zero_angle() {
        let p1 = PgPoint::new([1, 0, 1]);
        let p2 = PgPoint::new([0, 0, 1]); // Vertex
        let p3 = PgPoint::new([2, 0, 1]);

        // Vectors are in same direction
        let cos = angle_cosine(&p1, &p2, &p3);
        assert!(cos > Fraction::<i64>::new(0, 1));
    }

    #[test]
    fn test_angle_cosine_obtuse() {
        let p1 = PgPoint::new([1, 0, 1]);
        let p2 = PgPoint::new([0, 0, 1]); // Vertex
        let p3 = PgPoint::new([-1, 0, 1]);

        // Vectors are in opposite directions
        let cos = angle_cosine(&p1, &p2, &p3);
        assert!(cos < Fraction::<i64>::new(0, 1));
    }

    #[test]
    fn test_distance() {
        let p1 = PgPoint::new([0, 0, 1]);
        let p2 = PgPoint::new([3, 4, 1]);
        let dist = distance(&p1, &p2);
        // For now, this returns squared distance
        assert_eq!(dist, Fraction::<i64>::new(25, 1));
    }

    #[test]
    fn test_squared_distance_zero() {
        let p1 = PgPoint::new([1, 1, 1]);
        let p2 = PgPoint::new([1, 1, 1]);
        let dist_sq = squared_distance(&p1, &p2);
        assert_eq!(dist_sq, Fraction::<i64>::new(0, 1));
    }

    #[test]
    fn test_squared_distance_negative_coordinates() {
        let p1 = PgPoint::new([-1, -1, 1]);
        let p2 = PgPoint::new([2, 2, 1]);
        let dist_sq = squared_distance(&p1, &p2);
        assert_eq!(dist_sq, Fraction::<i64>::new(18, 1));
    }

    #[test]
    fn test_normalize_homogeneous_negative_gcd() {
        let mut coord = [-4, -6, -8];
        normalize_homogeneous(&mut coord);
        // GCD of 4, 6, 8 is 2, so [-4, -6, -8] -> [-2, -3, -4] -> [2, 3, 4] (flip sign)
        assert_eq!(coord, [2, 3, 4]);
    }

    #[test]
    fn test_normalize_homogeneous_no_gcd() {
        let mut coord = [5, 7, 11];
        normalize_homogeneous(&mut coord);
        assert_eq!(coord, [5, 7, 11]);
    }

    #[test]
    fn test_normalize_homogeneous_zero_last() {
        let mut coord = [2, 4, 0];
        normalize_homogeneous(&mut coord);
        assert_eq!(coord, [1, 2, 0]);
    }

    #[test]
    fn test_triangle_area_negative() {
        let p1 = PgPoint::new([0, 0, 1]);
        let p2 = PgPoint::new([0, 2, 1]);
        let p3 = PgPoint::new([2, 0, 1]);
        let area = triangle_area(&p1, &p2, &p3);
        // Clockwise orientation gives negative area
        assert_eq!(area, Fraction::<i64>::new(-2, 1));
    }

    #[test]
    fn test_point_in_triangle_inside() {
        let v1 = PgPoint::new([0, 0, 1]);
        let v2 = PgPoint::new([2, 0, 1]);
        let v3 = PgPoint::new([0, 2, 1]);

        let inside = PgPoint::new([1, 1, 2]);
        assert!(point_in_triangle(&inside, &v1, &v2, &v3));
    }

    #[test]
    fn test_point_in_triangle_vertex() {
        let v1 = PgPoint::new([0, 0, 1]);
        let v2 = PgPoint::new([2, 0, 1]);
        let v3 = PgPoint::new([0, 2, 1]);

        // Point at a vertex
        assert!(point_in_triangle(&v1, &v1, &v2, &v3));
    }

    #[test]
    fn test_is_line_at_infinity() {
        let finite = PgLine::new([1, 2, 3]);
        assert!(!is_line_at_infinity(&finite));

        let infinite = PgLine::new([0, 0, 1]);
        assert!(is_line_at_infinity(&infinite));
    }

    #[test]
    fn test_is_line_at_infinity_false() {
        let line = PgLine::new([1, 0, 0]);
        assert!(!is_line_at_infinity(&line));
    }
}
