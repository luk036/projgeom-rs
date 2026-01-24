//! Geometric transformations
//!
//! This module provides various geometric transformations including
//! rotations, translations, projections, and projective transformations.

use crate::pg_object::{PgPoint, PgLine};
use crate::pg_plane::ProjectivePlanePrimitive;
use fractions::Fraction;

/// A 3x3 transformation matrix for projective geometry
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    /// The 3x3 matrix elements in row-major order
    pub matrix: [[Fraction<i64>; 3]; 3],
}

impl Transform {
    /// Create a new identity transformation
    pub fn identity() -> Self {
        Transform {
            matrix: [
                [Fraction::<i64>::new(1, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1)],
                [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1), Fraction::<i64>::new(0, 1)],
                [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1)],
            ],
        }
    }

    /// Create a translation transformation
    ///
    /// # Arguments
    ///
    /// * `tx` - Translation in x direction
    /// * `ty` - Translation in y direction
    pub fn translation(tx: i64, ty: i64) -> Self {
        Transform {
            matrix: [
                [Fraction::<i64>::new(1, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(tx, 1)],
                [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1), Fraction::<i64>::new(ty, 1)],
                [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1)],
            ],
        }
    }

    /// Create a rotation transformation
    ///
    /// # Arguments
    ///
    /// * `angle_cos` - Cosine of the rotation angle
    /// * `angle_sin` - Sine of the rotation angle
    pub fn rotation(angle_cos: Fraction<i64>, angle_sin: Fraction<i64>) -> Self {
        Transform {
            matrix: [
                [angle_cos.clone(), -angle_sin.clone(), Fraction::<i64>::new(0, 1)],
                [angle_sin, angle_cos, Fraction::<i64>::new(0, 1)],
                [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1)],
            ],
        }
    }

    /// Create a scaling transformation
    ///
    /// # Arguments
    ///
    /// * `sx` - Scale factor in x direction
    /// * `sy` - Scale factor in y direction
    pub fn scaling(sx: Fraction<i64>, sy: Fraction<i64>) -> Self {
        Transform {
            matrix: [
                [sx, Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1)],
                [Fraction::<i64>::new(0, 1), sy, Fraction::<i64>::new(0, 1)],
                [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1)],
            ],
        }
    }

    /// Create a shear transformation
    ///
    /// # Arguments
    ///
    /// * `shx` - Shear factor in x direction
    /// * `shy` - Shear factor in y direction
    pub fn shear(shx: Fraction<i64>, shy: Fraction<i64>) -> Self {
        Transform {
            matrix: [
                [Fraction::<i64>::new(1, 1), shx.clone(), Fraction::<i64>::new(0, 1)],
                [shy, Fraction::<i64>::new(1, 1), Fraction::<i64>::new(0, 1)],
                [Fraction::<i64>::new(0, 1), Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1)],
            ],
        }
    }

    /// Compose this transformation with another
    pub fn compose(&self, other: &Transform) -> Transform {
        let mut result = Transform::identity();

        for i in 0..3 {
            for j in 0..3 {
                let mut sum = Fraction::<i64>::new(0, 1);
                for k in 0..3 {
                    sum = sum + self.matrix[i][k].clone() * other.matrix[k][j].clone();
                }
                result.matrix[i][j] = sum;
            }
        }

        result
    }

    /// Apply the transformation to a point
    pub fn apply_point(&self, point: &PgPoint) -> PgPoint {
        let x = Fraction::<i64>::new(point.coord[0], 1);
        let y = Fraction::<i64>::new(point.coord[1], 1);
        let z = Fraction::<i64>::new(point.coord[2], 1);

        let x_new = self.matrix[0][0].clone() * x.clone()
            + self.matrix[0][1].clone() * y.clone()
            + self.matrix[0][2].clone() * z.clone();
        let y_new = self.matrix[1][0].clone() * x.clone()
            + self.matrix[1][1].clone() * y.clone()
            + self.matrix[1][2].clone() * z.clone();
        let z_new = self.matrix[2][0].clone() * x
            + self.matrix[2][1].clone() * y
            + self.matrix[2][2].clone() * z;

        // Convert back to integer coordinates if possible
        PgPoint::new([
            x_new.numer() / x_new.denom(),
            y_new.numer() / y_new.denom(),
            z_new.numer() / z_new.denom(),
        ])
    }

    /// Apply the transformation to a line
    pub fn apply_line(&self, line: &PgLine) -> PgLine {
        // For lines, we need to use the inverse transpose
        let inverse = self.inverse();
        let x = Fraction::<i64>::new(line.coord[0], 1);
        let y = Fraction::<i64>::new(line.coord[1], 1);
        let z = Fraction::<i64>::new(line.coord[2], 1);

        let x_new = inverse.matrix[0][0].clone() * x.clone()
            + inverse.matrix[1][0].clone() * y.clone()
            + inverse.matrix[2][0].clone() * z.clone();
        let y_new = inverse.matrix[0][1].clone() * x.clone()
            + inverse.matrix[1][1].clone() * y.clone()
            + inverse.matrix[2][1].clone() * z.clone();
        let z_new = inverse.matrix[0][2].clone() * x
            + inverse.matrix[1][2].clone() * y
            + inverse.matrix[2][2].clone() * z;

        PgLine::new([
            x_new.numer() / x_new.denom(),
            y_new.numer() / y_new.denom(),
            z_new.numer() / z_new.denom(),
        ])
    }

    /// Compute the inverse of this transformation
    pub fn inverse(&self) -> Transform {
        // Compute the inverse of a 3x3 matrix
        let a = self.matrix[0][0].clone();
        let b = self.matrix[0][1].clone();
        let c = self.matrix[0][2].clone();
        let d = self.matrix[1][0].clone();
        let e = self.matrix[1][1].clone();
        let f = self.matrix[1][2].clone();
        let g = self.matrix[2][0].clone();
        let h = self.matrix[2][1].clone();
        let i = self.matrix[2][2].clone();

        // Compute determinant
        let det = a.clone() * (e.clone() * i.clone() - f.clone() * h.clone())
            - b.clone() * (d.clone() * i.clone() - f.clone() * g.clone())
            + c.clone() * (d.clone() * h.clone() - e.clone() * g.clone());

        if det == Fraction::<i64>::new(0, 1) {
            panic!("Cannot compute inverse of singular matrix");
        }

        let inv_det = Fraction::<i64>::new(1, 1) / det;

        // Compute adjugate matrix
        let matrix = [
            [
                inv_det.clone() * (e.clone() * i.clone() - f.clone() * h.clone()),
                inv_det.clone() * (c.clone() * h.clone() - b.clone() * i.clone()),
                inv_det.clone() * (b.clone() * f.clone() - c.clone() * e.clone()),
            ],
            [
                inv_det.clone() * (f.clone() * g.clone() - d.clone() * i.clone()),
                inv_det.clone() * (a.clone() * i.clone() - c.clone() * g.clone()),
                inv_det.clone() * (c.clone() * d.clone() - a.clone() * f.clone()),
            ],
            [
                inv_det.clone() * (d.clone() * h.clone() - e.clone() * g.clone()),
                inv_det.clone() * (b.clone() * g.clone() - a.clone() * h.clone()),
                inv_det.clone() * (a.clone() * e.clone() - b.clone() * d.clone()),
            ],
        ];

        Transform { matrix }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

/// Rotate a point around the origin
///
/// # Arguments
///
/// * `point` - The point to rotate
/// * `angle_cos` - Cosine of the rotation angle
/// * `angle_sin` - Sine of the rotation angle
pub fn rotate_point(point: &PgPoint, angle_cos: Fraction<i64>, angle_sin: Fraction<i64>) -> PgPoint {
    let transform = Transform::rotation(angle_cos, angle_sin);
    transform.apply_point(point)
}

/// Translate a point
///
/// # Arguments
///
/// * `point` - The point to translate
/// * `tx` - Translation in x direction
/// * `ty` - Translation in y direction
pub fn translate_point(point: &PgPoint, tx: i64, ty: i64) -> PgPoint {
    let transform = Transform::translation(tx, ty);
    transform.apply_point(point)
}

/// Scale a point
///
/// # Arguments
///
/// * `point` - The point to scale
/// * `sx` - Scale factor in x direction
/// * `sy` - Scale factor in y direction
pub fn scale_point(point: &PgPoint, sx: Fraction<i64>, sy: Fraction<i64>) -> PgPoint {
    let transform = Transform::scaling(sx, sy);
    transform.apply_point(point)
}

/// Apply a projective transformation defined by four point pairs
///
/// This computes the unique projective transformation that maps
/// four points to four other points.
///
/// # Arguments
///
/// * `src` - Array of four source points
/// * `dst` - Array of four destination points
///
/// # Returns
///
/// The transformation matrix
pub fn projective_transform(src: &[PgPoint; 4], dst: &[PgPoint; 4]) -> Transform {
    // This is a simplified implementation
    // A full implementation would require solving a system of linear equations
    // to find the transformation matrix that maps src to dst

    // For now, return identity as a placeholder
    Transform::identity()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let t = Transform::identity();
        let p = PgPoint::new([1, 2, 3]);
        let p_transformed = t.apply_point(&p);
        assert_eq!(p, p_transformed);
    }

    #[test]
    fn test_translation() {
        let t = Transform::translation(5, 3);
        let p = PgPoint::new([1, 2, 1]);
        let p_transformed = t.apply_point(&p);
        assert_eq!(p_transformed, PgPoint::new([6, 5, 1]));
    }

    #[test]
    fn test_rotation_90_degrees() {
        // Rotate 90 degrees counter-clockwise
        let t = Transform::rotation(Fraction::<i64>::new(0, 1), Fraction::<i64>::new(1, 1));
        let p = PgPoint::new([1, 0, 1]);
        let p_transformed = t.apply_point(&p);
        assert_eq!(p_transformed, PgPoint::new([0, 1, 1]));
    }

    #[test]
    fn test_scaling() {
        let t = Transform::scaling(Fraction::<i64>::new(2, 1), Fraction::<i64>::new(3, 1));
        let p = PgPoint::new([1, 2, 1]);
        let p_transformed = t.apply_point(&p);
        assert_eq!(p_transformed, PgPoint::new([2, 6, 1]));
    }

    #[test]
    fn test_compose() {
        let t1 = Transform::translation(2, 3);
        let t2 = Transform::scaling(Fraction::<i64>::new(2, 1), Fraction::<i64>::new(2, 1));
        let t_composed = t1.compose(&t2);

        let p = PgPoint::new([1, 1, 1]);
        let p_transformed = t_composed.apply_point(&p);

        // Scale first, then translate
        assert_eq!(p_transformed, PgPoint::new([4, 5, 1]));
    }

    #[test]
    fn test_inverse() {
        let t = Transform::translation(5, 3);
        let t_inv = t.inverse();
        let p = PgPoint::new([1, 2, 1]);

        let p_transformed = t.apply_point(&p);
        let p_restored = t_inv.apply_point(&p_transformed);

        assert_eq!(p, p_restored);
    }
}
