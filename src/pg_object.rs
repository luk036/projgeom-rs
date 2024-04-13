use crate::pg_plane::{ProjectivePlane, ProjectivePlanePrimitive};
// use crate::pg_plane::{check_axiom, coincident};

/// The `dot_product` function calculates the dot product of two 3-dimensional vectors.
///
/// Arguments:
///
/// * `v_a`: An array of three i64 values representing the first vector.
/// * `v_b`: The parameter `v_b` is a reference to an array of `i64` integers with a length of 3. It
/// represents a vector in .
///
/// Returns:
///
/// The dot_product function returns the dot product of two vectors, which is a scalar value of type
/// i64.
///
/// Examples:
///
/// ```rust
/// use projgeom_rs::pg_object::dot_product;
/// let result = dot_product(&[1, 2, 3], &[3, 4, 5]);
/// assert_eq!(result, 26);
/// ```
#[inline]
pub const fn dot_product(v_a: &[i64; 3], v_b: &[i64; 3]) -> i64 {
    v_a[0] * v_b[0] + v_a[1] * v_b[1] + v_a[2] * v_b[2]
}

/// Dot product (2d)
///
/// Examples:
///
/// ```rust
/// use projgeom_rs::pg_object::dot1;
/// let result = dot1(&[1, 2], &[3, 4]);
/// assert_eq!(result, 11);
/// ```
#[inline]
pub const fn dot1(v_a: &[i64], v_b: &[i64]) -> i64 {
    v_a[0] * v_b[0] + v_a[1] * v_b[1]
}

/// Cross product (2d)
///
/// Examples:
///
/// ```rust
/// use projgeom_rs::pg_object::cross2;
/// let result = cross2(&[1, 2, 3], &[3, 4, 5]);
/// assert_eq!(result, -2);
/// ```
#[inline]
pub const fn cross2(v_a: &[i64], v_b: &[i64]) -> i64 {
    v_a[0] * v_b[1] - v_a[1] * v_b[0]
}

/// Cross product
///
/// Examples:
///
/// ```rust
/// use projgeom_rs::pg_object::cross_product;
/// let v_a = cross_product(&[1, 2, 3], &[3, 4, 5]);
/// assert_eq!(v_a, [-2, 4, -2]);
/// ```
#[inline]
pub const fn cross_product(v_a: &[i64; 3], v_b: &[i64; 3]) -> [i64; 3] {
    [
        v_a[1] * v_b[2] - v_a[2] * v_b[1],
        v_a[2] * v_b[0] - v_a[0] * v_b[2],
        v_a[0] * v_b[1] - v_a[1] * v_b[0],
    ]
}

/// Plucker operation
///
/// Examples:
///
/// ```rust
/// use projgeom_rs::pg_object::plucker_operation;
/// let v_a = plucker_operation(1, &[1, 2, 3], -1, &[3, 4, 5]);
/// assert_eq!(v_a, [-2, -2, -2]);
/// ```
#[inline]
pub const fn plucker_operation(
    lambda_a: i64,
    v_a: &[i64; 3],
    mu_b: i64,
    v_b: &[i64; 3],
) -> [i64; 3] {
    [
        lambda_a * v_a[0] + mu_b * v_b[0],
        lambda_a * v_a[1] + mu_b * v_b[1],
        lambda_a * v_a[2] + mu_b * v_b[2],
    ]
}

macro_rules! define_point_or_line {
    (impl $point:ident) => {
        #[derive(Debug, Clone)]
        pub struct $point {
            /// Homogeneous coordinate
            pub coord: [i64; 3],
        }

        impl $point {
            /// Create a new point with the given coordinates.
            #[inline]
            pub const fn new(coord: [i64; 3]) -> Self {
                Self { coord }
            }
        }

        impl PartialEq for $point {
            /// Check if two points are equal.
            #[inline]
            fn eq(&self, other: &$point) -> bool {
                cross_product(&self.coord, &other.coord) == [0, 0, 0]
            }
        }
        impl Eq for $point {}
    };
}

macro_rules! define_line_for_point {
    (impl $line:ident, $point:ident) => {
        impl ProjectivePlane<$line, i64> for $point {
            /// Return the Dual not incident with Self
            #[inline]
            fn aux(&self) -> $line {
                $line::new(self.coord.clone())
            }

            /// Return the dot product of Self and `line`
            #[inline]
            fn dot(&self, line: &$line) -> i64 {
                dot_product(&self.coord, &line.coord)
            } // basic measurement

            #[inline]
            fn parametrize(&self, lambda: i64, pt_q: &Self, mu: i64) -> Self {
                Self::new(plucker_operation(lambda, &self.coord, mu, &pt_q.coord))
            }
        }

        impl ProjectivePlanePrimitive<$line> for $point {
            #[inline]
            fn incident(&self, _rhs: &$line) -> bool {
                dot_product(&self.coord, &_rhs.coord) == 0
            }

            #[inline]
            fn meet(&self, _rhs: &Self) -> $line {
                $line::new(cross_product(&self.coord, &_rhs.coord))
            }
        }
    };
}

macro_rules! define_point_and_line {
    (impl $point:ident, $line:ident) => {
        define_point_or_line!(impl $point);
        define_point_or_line!(impl $line);
        define_line_for_point!(impl $line, $point);
        define_line_for_point!(impl $point, $line);
    };
}

define_point_and_line!(impl PgPoint, PgLine);
define_point_and_line!(impl HyperbolicPoint, HyperbolicLine);
define_point_and_line!(impl EllipticPoint, EllipticLine);
define_point_and_line!(impl MyCKPoint, MyCKLine);
define_point_and_line!(impl PerspPoint, PerspLine);
define_point_and_line!(impl EuclidPoint, EuclidLine);
// You may add your own geometry here
