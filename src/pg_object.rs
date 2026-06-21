use crate::pg_plane::{ProjectivePlane, ProjectivePlanePrimitive};
// use crate::pg_plane::{check_axiom, coincident};

/// The `dot_product` function calculates the dot product of two 3-dimensional vectors.
///
/// $$ \mathbf{v}_a \cdot \mathbf{v}_b = x_a x_b + y_a y_b + z_a z_b $$
///
/// Arguments:
///
/// * `v_a`: An array of three i64 values representing the first vector.
/// * `v_b`: The parameter `v_b` is a reference to an array of `i64` integers with a length of 3. It
///   represents a vector in .
///
/// Returns:
///
/// The dot_product function returns the dot product of two vectors, which is a scalar value of type
/// i64.
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_object::dot_product;
/// let result = dot_product(&[1, 2, 3], &[3, 4, 5]);
/// assert_eq!(result, 26);
/// ```
#[inline]
pub fn dot_product(vec_a: &[i64; 3], vec_b: &[i64; 3]) -> i64 {
    // Use wrapping operations to avoid overflow
    vec_a[0].wrapping_mul(vec_b[0]).wrapping_add(
        vec_a[1]
            .wrapping_mul(vec_b[1])
            .wrapping_add(vec_a[2].wrapping_mul(vec_b[2])),
    )
}

/// Dot product (2d)
///
/// $$ \mathbf{v}_a \cdot \mathbf{v}_b = x_a x_b + y_a y_b $$
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_object::dot1;
/// let result = dot1(&[1, 2], &[3, 4]);
/// assert_eq!(result, 11);
/// ```
#[inline]
pub const fn dot1(vec_a: &[i64], vec_b: &[i64]) -> i64 {
    vec_a[0] * vec_b[0] + vec_a[1] * vec_b[1]
}

/// Cross product 0th component (y1*z2 - y2*z1)
///
/// $$ (\mathbf{v}_a \times \mathbf{v}_b)_0 = y_1 z_2 - y_2 z_1 $$
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_object::cross0;
/// let result = cross0(&[1, 2, 3], &[4, 5, 6]);
/// assert_eq!(result, 2*6 - 5*3); // -3
/// ```
#[inline]
pub const fn cross0(vec_a: &[i64; 3], vec_b: &[i64; 3]) -> i64 {
    vec_a[1] * vec_b[2] - vec_b[1] * vec_a[2]
}

/// Cross product 1st component (x1*z2 - x2*z1)
///
/// $$ (\mathbf{v}_a \times \mathbf{v}_b)_1 = x_1 z_2 - x_2 z_1 $$
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_object::cross1;
/// let result = cross1(&[1, 2, 3], &[4, 5, 6]);
/// assert_eq!(result, 1*6 - 4*3); // -6
/// ```
#[inline]
pub const fn cross1(vec_a: &[i64; 3], vec_b: &[i64; 3]) -> i64 {
    vec_a[0] * vec_b[2] - vec_b[0] * vec_a[2]
}

/// Cross product (2d)
///
/// $$ \mathbf{v}_a \times \mathbf{v}_b = x_1 y_2 - x_2 y_1 $$
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_object::cross2;
/// let result = cross2(&[1, 2], &[3, 4]);
/// assert_eq!(result, -2);
/// ```
#[inline]
pub const fn cross2(vec_a: &[i64], vec_b: &[i64]) -> i64 {
    vec_a[0] * vec_b[1] - vec_a[1] * vec_b[0]
}

/// Cross product (2d) from 3-element arrays (3rd component).
///
/// $$ (\mathbf{v}_a \times \mathbf{v}_b)_2 = x_1 y_2 - x_2 y_1 $$
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_object::cross2_3;
/// let result = cross2_3(&[1, 2, 3], &[3, 4, 5]);
/// assert_eq!(result, -2);
/// ```
#[inline]
pub const fn cross2_3(vec_a: &[i64; 3], vec_b: &[i64; 3]) -> i64 {
    vec_a[0] * vec_b[1] - vec_a[1] * vec_b[0]
}

/// Dot product (0,2)-components (x1*x2 + z1*z2)
///
/// $$ \mathbf{v}_a \cdot_{(0,2)} \mathbf{v}_b = x_1 x_2 + z_1 z_2 $$
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_object::dot2;
/// let result = dot2(&[1, 2, 3], &[4, 5, 6]);
/// assert_eq!(result, 1*4 + 3*6); // 22
/// ```
#[inline]
pub const fn dot2(vec_a: &[i64; 3], vec_b: &[i64; 3]) -> i64 {
    vec_a[0] * vec_b[0] + vec_a[2] * vec_b[2]
}

/// Square function
///
/// $$ \text{sq}(x) = x^2 $$
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_object::sq;
/// assert_eq!(sq(5), 25);
/// assert_eq!(sq(-3), 9);
/// ```
#[inline]
pub const fn sq(val: i64) -> i64 {
    val * val
}

/// Cross product
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_object::cross_product;
/// let v_a = cross_product(&[1, 2, 3], &[3, 4, 5]);
/// assert_eq!(v_a, [-2, 4, -2]);
/// ```
#[inline]
pub fn cross_product(vec_a: &[i64; 3], vec_b: &[i64; 3]) -> [i64; 3] {
    [
        vec_a[1]
            .wrapping_mul(vec_b[2])
            .wrapping_sub(vec_a[2].wrapping_mul(vec_b[1])),
        vec_a[2]
            .wrapping_mul(vec_b[0])
            .wrapping_sub(vec_a[0].wrapping_mul(vec_b[2])),
        vec_a[0]
            .wrapping_mul(vec_b[1])
            .wrapping_sub(vec_a[1].wrapping_mul(vec_b[0])),
    ]
}

/// Plucker operation
///
/// # Examples
///
/// ```
/// use projgeom_rs::pg_object::plucker_operation;
/// let v_a = plucker_operation(1, &[1, 2, 3], -1, &[3, 4, 5]);
/// assert_eq!(v_a, [-2, -2, -2]);
/// ```
#[inline]
pub fn plucker_operation(lambda_a: i64, vec_a: &[i64; 3], mu_b: i64, vec_b: &[i64; 3]) -> [i64; 3] {
    [
        lambda_a
            .wrapping_mul(vec_a[0])
            .wrapping_add(mu_b.wrapping_mul(vec_b[0])),
        lambda_a
            .wrapping_mul(vec_a[1])
            .wrapping_add(mu_b.wrapping_mul(vec_b[1])),
        lambda_a
            .wrapping_mul(vec_a[2])
            .wrapping_add(mu_b.wrapping_mul(vec_b[2])),
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

            /// Create a new point from individual coordinates.
            #[inline]
            pub const fn from_coords(x: i64, y: i64, z: i64) -> Self {
                Self { coord: [x, y, z] }
            }
        }

        impl From<[i64; 3]> for $point {
            #[inline]
            fn from(coord: [i64; 3]) -> Self {
                Self { coord }
            }
        }

        impl From<(i64, i64, i64)> for $point {
            #[inline]
            fn from(coords: (i64, i64, i64)) -> Self {
                Self {
                    coord: [coords.0, coords.1, coords.2],
                }
            }
        }

        impl From<&[i64; 3]> for $point {
            #[inline]
            fn from(coord: &[i64; 3]) -> Self {
                Self { coord: *coord }
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
            fn parametrize(&self, lambda_val: i64, point_q: &Self, mu_val: i64) -> Self {
                Self::new(plucker_operation(
                    lambda_val,
                    &self.coord,
                    mu_val,
                    &point_q.coord,
                ))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_array_for_pgpoint() {
        let coord = [1, 2, 3];
        let p: PgPoint = coord.into();
        assert_eq!(p.coord, [1, 2, 3]);
    }

    #[test]
    fn test_from_tuple_for_pgpoint() {
        let p: PgPoint = (1, 2, 3).into();
        assert_eq!(p.coord, [1, 2, 3]);
    }

    #[test]
    fn test_from_slice_for_pgpoint() {
        let coord = [1, 2, 3];
        let p: PgPoint = (&coord).into();
        assert_eq!(p.coord, [1, 2, 3]);
    }

    #[test]
    fn test_from_coords_method() {
        let p = PgPoint::from_coords(1, 2, 3);
        assert_eq!(p.coord, [1, 2, 3]);
    }

    #[test]
    fn test_partial_eq_reflexive() {
        let p = PgPoint::new([2, 4, 6]);
        assert_eq!(p, p);
    }

    #[test]
    fn test_partial_eq_proportional() {
        let p1 = PgPoint::new([1, 2, 3]);
        let p2 = PgPoint::new([2, 4, 6]);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_partial_eq_not_equal() {
        let p1 = PgPoint::new([1, 2, 3]);
        let p2 = PgPoint::new([1, 0, 0]);
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_from_array_for_all_point_types() {
        let coord = [1, 2, 3];
        let _pg: PgPoint = coord.into();
        let coord = [1, 2, 3];
        let _ell: EllipticPoint = coord.into();
        let coord = [1, 2, 3];
        let _hyp: HyperbolicPoint = coord.into();
        let coord = [1, 2, 3];
        let _euclid: EuclidPoint = coord.into();
        let coord = [1, 2, 3];
        let _myck: MyCKPoint = coord.into();
        let coord = [1, 2, 3];
        let _persp: PerspPoint = coord.into();
    }

    #[test]
    fn test_from_coords_all_point_types() {
        let _pg = PgPoint::from_coords(1, 2, 3);
        let _ell = EllipticPoint::from_coords(1, 2, 3);
        let _hyp = HyperbolicPoint::from_coords(1, 2, 3);
        let _euclid = EuclidPoint::from_coords(1, 2, 3);
        let _myck = MyCKPoint::from_coords(1, 2, 3);
        let _persp = PerspPoint::from_coords(1, 2, 3);
    }

    #[test]
    fn test_pgline_equality() {
        let l1 = PgLine::new([1, 2, 3]);
        let l2 = PgLine::new([2, 4, 6]);
        assert_eq!(l1, l2);
    }

    #[test]
    fn test_pgline_inequality() {
        let l1 = PgLine::new([1, 2, 3]);
        let l2 = PgLine::new([1, 0, 0]);
        assert_ne!(l1, l2);
    }
}
