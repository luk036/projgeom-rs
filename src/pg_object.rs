use crate::pg_plane::{ProjPlane, ProjPlanePrim};
// use crate::pg_plane::{check_axiom, coincident};

/**
Dot product

Examples:

```rust
use projgeom_rs::pg_object::dot;
let a = dot(&[1, 2, 3], &[3, 4, 5]);
assert_eq!(a, 26);
```
*/
#[inline]
pub fn dot(a: &[i128; 3], b: &[i128; 3]) -> i128 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/**
Dot product (2d)

Examples:

```rust
use projgeom_rs::pg_object::dot;
let a = dot(&[1, 2, 3], &[3, 4, 5]);
assert_eq!(a, 26);
```
*/
#[inline]
pub fn dot1(a: &[i128], b: &[i128]) -> i128 {
    a[0] * b[0] + a[1] * b[1]
}

/**
Cross product (2d)

Examples:

```rust
use projgeom_rs::pg_object::cross2;
let a = cross2(&[1, 2, 3], &[3, 4, 5]);
assert_eq!(a, -2);
```
*/
#[inline]
pub fn cross2(a: &[i128], b: &[i128]) -> i128 {
    a[0] * b[1] - a[1] * b[0]
}

/**
Cross product

Examples:

```rust
use projgeom_rs::pg_object::cross;
let a = cross(&[1, 2, 3], &[3, 4, 5]);
assert_eq!(a, [-2, 4, -2]);
```
*/
#[inline]
pub fn cross(a: &[i128; 3], b: &[i128; 3]) -> [i128; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/**
Plucker operation

Examples:

```rust
use projgeom_rs::pg_object::plckr;
let a = plckr(&1, &[1, 2, 3], &-1, &[3, 4, 5]);
assert_eq!(a, [-2, -2, -2]);
```
*/
#[inline]
pub fn plckr(ld: &i128, p: &[i128; 3], mu: &i128, q: &[i128; 3]) -> [i128; 3] {
    [
        ld * p[0] + mu * q[0],
        ld * p[1] + mu * q[1],
        ld * p[2] + mu * q[2],
    ]
}

macro_rules! define_point_or_line {
    (impl $point:ident) => {
        #[derive(Debug, Clone)]
        pub struct $point {
            pub coord: [i128; 3],
        }

        impl $point {
            #[inline]
            pub fn new(coord: [i128; 3]) -> Self {
                Self { coord }
            }
        }

        impl PartialEq for $point {
            #[inline]
            fn eq(&self, other: &$point) -> bool {
                cross(&self.coord, &other.coord) == [0, 0, 0]
            }
        }
        impl Eq for $point {}
    };
}

macro_rules! define_line_for_point {
    (impl $line:ident, $point:ident) => {
        impl ProjPlane<$line, i128> for $point {
            #[inline]
            fn aux(&self) -> $line {
                $line::new(self.coord.clone())
            }

            #[inline]
            fn dot(&self, line: &$line) -> i128 {
                dot(&self.coord, &line.coord)
            } // basic measurement

            #[inline]
            fn plucker(&self, ld: &i128, q: &Self, mu: &i128) -> Self {
                Self::new(plckr(ld, &self.coord, mu, &q.coord))
            }
        }

        impl ProjPlanePrim<$line> for $point {
            #[inline]
            fn incident(&self, _rhs: &$line) -> bool {
                dot(&self.coord, &_rhs.coord) == 0
            }

            #[inline]
            fn circ(&self, _rhs: &Self) -> $line {
                $line::new(cross(&self.coord, &_rhs.coord))
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
define_point_and_line!(impl HypPoint, HypLine);
define_point_and_line!(impl EllPoint, EllLine);
define_point_and_line!(impl MyCKPoint, MyCKLine);
define_point_and_line!(impl PerspPoint, PerspLine);
define_point_and_line!(impl EuclidPoint, EuclidLine);
// You may add your own geometry here
