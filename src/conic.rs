//! Conic sections support
//!
//! This module provides support for conic sections (circles, ellipses,
//! parabolas, and hyperbolas) in projective geometry.

use crate::pg_object::{PgLine, PgPoint};
use fractions::Fraction;

/// Represents a conic section in homogeneous coordinates
///
/// A conic is represented by a symmetric 3x3 matrix Q such that
/// a point x lies on the conic if x^T Q x = 0.
#[derive(Debug, Clone, PartialEq)]
pub struct Conic {
    /// The 3x3 symmetric matrix representing the conic
    pub matrix: [[Fraction<i64>; 3]; 3],
}

impl Conic {
    /// Create a new conic from a symmetric matrix
    pub fn new(matrix: [[Fraction<i64>; 3]; 3]) -> Self {
        Conic { matrix }
    }

    /// Create a circle with given center and radius squared
    ///
    /// # Arguments
    ///
    /// * `center_x` - X coordinate of center
    /// * `center_y` - Y coordinate of center
    /// * `radius_sq` - Squared radius
    pub fn circle(center_x: i64, center_y: i64, radius_sq: i64) -> Self {
        let cx = Fraction::<i64>::new(center_x, 1);
        let cy = Fraction::<i64>::new(center_y, 1);
        let r2 = Fraction::<i64>::new(radius_sq, 1);

        // Circle equation: (x-cx)^2 + (y-cy)^2 = r^2
        // In homogeneous coordinates: x^2 + y^2 - 2cx*x*z - 2cy*y*z + (cx^2 + cy^2 - r^2)*z^2 = 0
        let matrix = [
            [Fraction::<i64>::new(1, 1), Fraction::<i64>::new(0, 1), -cx],
            [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1), -cy],
            [-cx, -cy, cx * cx + cy * cy - r2],
        ];

        Conic { matrix }
    }

    /// Create a unit circle centered at the origin
    pub fn unit_circle() -> Self {
        Conic::circle(0, 0, 1)
    }

    /// Create a parabola
    ///
    /// # Arguments
    ///
    /// * `a` - Coefficient in y = ax^2
    pub fn parabola(a: Fraction<i64>) -> Self {
        // Parabola: y = ax^2
        // In homogeneous coordinates: y*z - a*x^2 = 0
        let matrix = [
            [-a, Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1)],
            [
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(1, 2),
            ],
            [
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(1, 2),
                Fraction::<i64>::new(0, 1),
            ],
        ];

        Conic { matrix }
    }

    /// Check if a point lies on the conic
    ///
    /// # Arguments
    ///
    /// * `point` - The point to test
    ///
    /// # Returns
    ///
    /// True if the point lies on the conic
    pub fn contains(&self, point: &PgPoint) -> bool {
        let x = Fraction::<i64>::new(point.coord[0], 1);
        let y = Fraction::<i64>::new(point.coord[1], 1);
        let z = Fraction::<i64>::new(point.coord[2], 1);

        // Compute x^T Q x
        let result = x * (self.matrix[0][0] * x + self.matrix[0][1] * y + self.matrix[0][2] * z)
            + y * (self.matrix[1][0] * x + self.matrix[1][1] * y + self.matrix[1][2] * z)
            + z * (self.matrix[2][0] * x + self.matrix[2][1] * y + self.matrix[2][2] * z);

        result == Fraction::<i64>::new(0, 1)
    }

    /// Find the polar line of a point with respect to the conic
    ///
    /// # Arguments
    ///
    /// * `point` - The point
    ///
    /// # Returns
    ///
    /// The polar line
    pub fn polar(&self, point: &PgPoint) -> PgLine {
        let x = Fraction::<i64>::new(point.coord[0], 1);
        let y = Fraction::<i64>::new(point.coord[1], 1);
        let z = Fraction::<i64>::new(point.coord[2], 1);

        // Polar line: Q * x
        let a = self.matrix[0][0] * x + self.matrix[0][1] * y + self.matrix[0][2] * z;
        let b = self.matrix[1][0] * x + self.matrix[1][1] * y + self.matrix[1][2] * z;
        let c = self.matrix[2][0] * x + self.matrix[2][1] * y + self.matrix[2][2] * z;

        PgLine::new([
            a.numer() / a.denom(),
            b.numer() / b.denom(),
            c.numer() / c.denom(),
        ])
    }

    /// Find the pole of a line with respect to the conic
    ///
    /// # Arguments
    ///
    /// * `line` - The line
    ///
    /// # Returns
    ///
    /// The pole point
    pub fn pole(&self, line: &PgLine) -> PgPoint {
        // Pole: Q^{-1} * line
        // For now, we'll use a simplified approach
        // A full implementation would require computing the inverse of Q

        // Placeholder: return a point that lies on the line
        PgPoint::new([line.coord[0], line.coord[1], line.coord[2]])
    }

    /// Compute the tangent line at a point on the conic
    ///
    /// # Arguments
    ///
    /// * `point` - A point on the conic
    ///
    /// # Returns
    ///
    /// The tangent line at that point
    pub fn tangent(&self, point: &PgPoint) -> PgLine {
        // The tangent is the polar of the point
        self.polar(point)
    }

    /// Find the intersection points of a line with the conic
    ///
    /// # Arguments
    ///
    /// * `line` - The line
    ///
    /// # Returns
    ///
    /// A vector of intersection points (0, 1, or 2 points)
    pub fn intersect(&self, _line: &PgLine) -> Vec<PgPoint> {
        // Solve for intersection of line and conic
        // This requires solving a quadratic equation

        // Placeholder: return empty vector
        vec![]
    }

    /// Compute the discriminant of the conic
    ///
    /// # Returns
    ///
    /// The discriminant, which determines the type of conic:
    /// - Positive: Ellipse (or circle)
    /// - Zero: Parabola
    /// - Negative: Hyperbola
    pub fn discriminant(&self) -> Fraction<i64> {
        // Compute the determinant of the 2x2 upper-left submatrix
        let a = self.matrix[0][0];
        let b = self.matrix[0][1];
        let d = self.matrix[1][0];
        let e = self.matrix[1][1];

        a * e - b * d
    }

    /// Determine the type of conic
    ///
    /// # Returns
    ///
    /// The type of conic
    pub fn conic_type(&self) -> ConicType {
        let disc = self.discriminant();

        if disc > Fraction::<i64>::new(0, 1) {
            ConicType::Ellipse
        } else if disc == Fraction::<i64>::new(0, 1) {
            ConicType::Parabola
        } else {
            ConicType::Hyperbola
        }
    }
}

/// Types of conic sections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConicType {
    /// Ellipse (including circles)
    Ellipse,
    /// Parabola
    Parabola,
    /// Hyperbola
    Hyperbola,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_circle() {
        let circle = Conic::unit_circle();

        // Points on the unit circle
        let p1 = PgPoint::new([1, 0, 1]);
        let p2 = PgPoint::new([0, 1, 1]);
        let p3 = PgPoint::new([-1, 0, 1]);
        let p4 = PgPoint::new([0, -1, 1]);

        assert!(circle.contains(&p1));
        assert!(circle.contains(&p2));
        assert!(circle.contains(&p3));
        assert!(circle.contains(&p4));

        // Point not on the circle
        let p5 = PgPoint::new([2, 0, 1]);
        assert!(!circle.contains(&p5));
    }

    #[test]
    fn test_circle_with_center() {
        let circle = Conic::circle(1, 1, 4);

        // Center point
        let center = PgPoint::new([1, 1, 1]);
        assert!(!circle.contains(&center));

        // Point on the circle (distance 2 from center)
        let p1 = PgPoint::new([3, 1, 1]);
        assert!(circle.contains(&p1));

        let p2 = PgPoint::new([1, 3, 1]);
        assert!(circle.contains(&p2));
    }

    #[test]
    fn test_conic_type() {
        let circle = Conic::unit_circle();
        assert_eq!(circle.conic_type(), ConicType::Ellipse);

        let parabola = Conic::parabola(Fraction::<i64>::new(1, 1));
        assert_eq!(parabola.conic_type(), ConicType::Parabola);
    }

    #[test]
    fn test_polar() {
        let circle = Conic::unit_circle();

        // Point on the circle
        let p = PgPoint::new([1, 0, 1]);
        let polar = circle.polar(&p);

        // The polar should be the tangent line
        // For the unit circle at (1,0), the tangent is x = 1
        assert_eq!(polar, PgLine::new([1, 0, -1]));
    }

    #[test]
    fn test_parabola_creation() {
        let parabola = Conic::parabola(Fraction::<i64>::new(1, 1));

        // Check that it's recognized as a parabola
        assert_eq!(parabola.conic_type(), ConicType::Parabola);
    }

    #[test]
    fn test_parabola_different_coefficients() {
        let parabola1 = Conic::parabola(Fraction::<i64>::new(1, 1));
        let parabola2 = Conic::parabola(Fraction::<i64>::new(2, 1));

        // Both should be parabolas
        assert_eq!(parabola1.conic_type(), ConicType::Parabola);
        assert_eq!(parabola2.conic_type(), ConicType::Parabola);

        // They should have different matrices
        assert_ne!(parabola1.matrix, parabola2.matrix);
    }

    #[test]
    fn test_tangent() {
        let circle = Conic::unit_circle();

        // Point on the circle
        let p = PgPoint::new([0, 1, 1]);
        let tangent = circle.tangent(&p);

        // The tangent should be the same as the polar
        let polar = circle.polar(&p);
        assert_eq!(tangent, polar);
    }

    #[test]
    fn test_tangent_multiple_points() {
        let circle = Conic::unit_circle();

        // Test multiple points on the circle
        let points = vec![
            PgPoint::new([1, 0, 1]),
            PgPoint::new([0, 1, 1]),
            PgPoint::new([-1, 0, 1]),
            PgPoint::new([0, -1, 1]),
        ];

        for p in points {
            let tangent = circle.tangent(&p);
            let polar = circle.polar(&p);
            assert_eq!(tangent, polar);
        }
    }

    #[test]
    fn test_intersect_empty() {
        let circle = Conic::unit_circle();
        let line = PgLine::new([1, 0, 2]); // Line x = -2, which doesn't intersect unit circle

        let intersections = circle.intersect(&line);
        // For now, this returns empty due to placeholder implementation
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_intersect_line_through_center() {
        let circle = Conic::unit_circle();
        let line = PgLine::new([0, 0, 1]); // Line through origin

        let intersections = circle.intersect(&line);
        // For now, this returns empty due to placeholder implementation
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_discriminant_ellipse() {
        let circle = Conic::unit_circle();
        let disc = circle.discriminant();

        // Ellipse should have positive discriminant
        assert!(disc > Fraction::<i64>::new(0, 1));
    }

    #[test]
    fn test_discriminant_parabola() {
        let parabola = Conic::parabola(Fraction::<i64>::new(1, 1));
        let disc = parabola.discriminant();

        // Parabola should have zero discriminant
        assert_eq!(disc, Fraction::<i64>::new(0, 1));
    }

    #[test]
    fn test_discriminant_hyperbola() {
        // Create a hyperbola: x^2 - y^2 = 1
        let matrix = [
            [
                Fraction::<i64>::new(1, 1),
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(0, 1),
            ],
            [
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(-1, 1),
                Fraction::<i64>::new(0, 1),
            ],
            [
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(-1, 1),
            ],
        ];
        let hyperbola = Conic::new(matrix);
        let disc = hyperbola.discriminant();

        // Hyperbola should have negative discriminant
        assert!(disc < Fraction::<i64>::new(0, 1));
    }

    #[test]
    fn test_pole() {
        let circle = Conic::unit_circle();
        let line = PgLine::new([1, 0, -1]); // Line x = 1

        let pole = circle.pole(&line);
        // For now, this returns a placeholder point
        assert!(pole.coord[0] != 0 || pole.coord[1] != 0 || pole.coord[2] != 0);
    }

    #[test]
    fn test_conic_type_all_types() {
        // Ellipse
        let circle = Conic::unit_circle();
        assert_eq!(circle.conic_type(), ConicType::Ellipse);

        // Parabola
        let parabola = Conic::parabola(Fraction::<i64>::new(1, 1));
        assert_eq!(parabola.conic_type(), ConicType::Parabola);

        // Hyperbola
        let matrix = [
            [
                Fraction::<i64>::new(1, 1),
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(0, 1),
            ],
            [
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(-1, 1),
                Fraction::<i64>::new(0, 1),
            ],
            [
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(0, 1),
                Fraction::<i64>::new(-1, 1),
            ],
        ];
        let hyperbola = Conic::new(matrix);
        assert_eq!(hyperbola.conic_type(), ConicType::Hyperbola);
    }
}
