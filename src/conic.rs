//! Conic sections support
//!
//! This module provides support for conic sections (circles, ellipses,
//! parabolas, and hyperbolas) in projective geometry.

use crate::linear::{adjugate3, det3, matvec3, rationalize, Mat3, Vec3};
use crate::pg_object::{PgLine, PgPoint};
use fractions::Fraction;

fn isqrt(n: i64) -> i64 {
    if n < 2 {
        return n;
    }
    let mut lo: i64 = 1;
    let mut hi: i64 = n / 2 + 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let sq = mid * mid;
        if sq == n {
            return mid;
        }
        if sq < n {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    hi
}

fn is_zero(v: &Vec3) -> bool {
    v[0] == Fraction::new(0, 1) && v[1] == Fraction::new(0, 1) && v[2] == Fraction::new(0, 1)
}

fn cross3(a: &Vec3, b: &Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn quadratic_form(q: &Mat3, x: &Vec3) -> Fraction<i64> {
    x[0] * (q[0][0] * x[0] + q[0][1] * x[1] + q[0][2] * x[2])
        + x[1] * (q[1][0] * x[0] + q[1][1] * x[1] + q[1][2] * x[2])
        + x[2] * (q[2][0] * x[0] + q[2][1] * x[1] + q[2][2] * x[2])
}

fn bilinear_form(q: &Mat3, x: &Vec3, y: &Vec3) -> Fraction<i64> {
    x[0] * (q[0][0] * y[0] + q[0][1] * y[1] + q[0][2] * y[2])
        + x[1] * (q[1][0] * y[0] + q[1][1] * y[1] + q[1][2] * y[2])
        + x[2] * (q[2][0] * y[0] + q[2][1] * y[1] + q[2][2] * y[2])
}

fn rational_sqrt(value: &Fraction<i64>) -> Option<Fraction<i64>> {
    if *value < Fraction::new(0, 1) {
        return None;
    }
    let n = isqrt(*value.numer());
    let d = isqrt(*value.denom());
    if n * n == *value.numer() && d * d == *value.denom() {
        Some(Fraction::new(n, d))
    } else {
        None
    }
}

/// Represents a conic section in homogeneous coordinates
///
/// A conic is represented by a symmetric 3x3 matrix $$ Q $$ such that
/// a point $$ x $$ lies on the conic if:
///
/// $$ x^T Q x = 0 $$
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
    /// $$ (x - c_x)^2 + (y - c_y)^2 = r^2 $$
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
    ///
    /// $$ x^2 + y^2 = 1 $$
    #[inline]
    pub fn unit_circle() -> Self {
        Conic::circle(0, 0, 1)
    }

    /// Create a parabola
    ///
    /// $$ y = a x^2 $$
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
    /// $$ x^T Q x = 0 $$
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
    /// $$ l = Q p $$
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

        PgLine::new(rationalize(&[a, b, c]))
    }

    /// Find the pole of a line with respect to the conic
    ///
    /// $$ p = Q^{-1} l $$
    ///
    /// # Arguments
    ///
    /// * `line` - The line
    ///
    /// # Returns
    ///
    /// The pole point
    pub fn pole(&self, line: &PgLine) -> PgPoint {
        if det3(&self.matrix) == Fraction::new(0, 1) {
            panic!("Cannot compute the pole of a degenerate conic");
        }
        let adj = adjugate3(&self.matrix);
        let l = [
            Fraction::<i64>::new(line.coord[0], 1),
            Fraction::<i64>::new(line.coord[1], 1),
            Fraction::<i64>::new(line.coord[2], 1),
        ];
        PgPoint::new(rationalize(&matvec3(&adj, &l)))
    }

    /// Compute the tangent line at a point on the conic
    ///
    /// At a point $$ p $$ on the conic, the tangent is the polar: $$ t = Q p $$
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
    /// Solve for $$ x $$ satisfying $$ x^T Q x = 0 $$ and $$ l^T x = 0 $$.
    ///
    /// # Arguments
    ///
    /// * `line` - The line
    ///
    /// # Returns
    ///
    /// A vector of intersection points (0, 1, or 2 points)
    pub fn intersect(&self, line: &PgLine) -> Vec<PgPoint> {
        let line_coord = [
            Fraction::<i64>::new(line.coord[0], 1),
            Fraction::<i64>::new(line.coord[1], 1),
            Fraction::<i64>::new(line.coord[2], 1),
        ];

        // A basis (origin, direction) of the line: two independent l x e_j.
        let zero = Fraction::<i64>::new(0, 1);
        let candidates: [Vec3; 3] = [
            [zero, line_coord[2], -line_coord[1]],
            [-line_coord[2], zero, line_coord[0]],
            [line_coord[1], -line_coord[0], zero],
        ];
        let mut vectors: Vec<Vec3> = Vec::with_capacity(2);
        for candidate in candidates {
            if !is_zero(&candidate) {
                vectors.push(candidate);
            }
        }
        let origin = vectors[0];
        let mut direction = [zero, zero, zero];
        for candidate in &vectors[1..] {
            if !is_zero(&cross3(&origin, candidate)) {
                direction = *candidate;
                break;
            }
        }

        let quad = quadratic_form(&self.matrix, &direction);
        let lin = Fraction::<i64>::new(2, 1) * bilinear_form(&self.matrix, &origin, &direction);
        let constant = quadratic_form(&self.matrix, &origin);

        let point_at = |t: &Fraction<i64>| -> PgPoint {
            PgPoint::new(rationalize(&[
                origin[0] + *t * direction[0],
                origin[1] + *t * direction[1],
                origin[2] + *t * direction[2],
            ]))
        };

        if quad == zero {
            if lin == zero {
                panic!("line lies on the conic");
            }
            return vec![point_at(&(-constant / lin))];
        }

        let disc = lin * lin - Fraction::<i64>::new(4, 1) * quad * constant;
        if disc < zero {
            return Vec::new();
        }
        match rational_sqrt(&disc) {
            None => panic!("intersection requires a quadratic surd"),
            Some(root) => {
                let two_q = Fraction::<i64>::new(2, 1) * quad;
                if root == zero {
                    vec![point_at(&(-lin / two_q))]
                } else {
                    vec![
                        point_at(&((-lin + root) / two_q)),
                        point_at(&((-lin - root) / two_q)),
                    ]
                }
            }
        }
    }

    /// Compute the discriminant of the conic
    ///
    /// $$ \Delta = \begin{vmatrix} a & b \\\\ d & e \end{vmatrix} = a e - b d $$
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
        let line = PgLine::new([1, 0, 2]); // Line x = -2, which doesn't meet unit circle

        // A non-secant line has no real intersection points: an empty list.
        let intersections = circle.intersect(&line);
        assert!(intersections.is_empty());
    }

    #[test]
    fn test_intersect_line_through_center() {
        let circle = Conic::unit_circle();
        let line = PgLine::new([1, 0, 0]); // Line x = 0, through the center

        let intersections = circle.intersect(&line);
        assert_eq!(intersections.len(), 2);
        for point in &intersections {
            assert!(circle.contains(point));
        }
        assert!(intersections.contains(&PgPoint::new([0, 1, 1])));
        assert!(intersections.contains(&PgPoint::new([0, -1, 1])));
    }

    #[test]
    #[should_panic]
    fn test_intersect_irrational_panics() {
        let circle = Conic::unit_circle();
        let line = PgLine::new([1, 1, 0]); // x + y = 0: irrational intersection
        circle.intersect(&line);
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
        assert_eq!(pole, PgPoint::new([1, 0, 1]));
    }

    #[test]
    fn test_polar_parabola_no_truncation() {
        let parabola = Conic::parabola(Fraction::<i64>::new(1, 1));
        let polar = parabola.polar(&PgPoint::new([0, 1, 1]));
        assert_eq!(polar, PgLine::new([0, 1, 1]));
    }

    #[test]
    fn test_pole_polar_roundtrip() {
        let circle = Conic::unit_circle();
        let line = PgLine::new([1, 0, -1]);
        assert_eq!(circle.polar(&circle.pole(&line)), line);
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
