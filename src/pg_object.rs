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
pub fn dot(a: &[i64; 3], b: &[i64; 3]) -> i64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
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
pub fn cross(a: &[i64; 3], b: &[i64; 3]) -> [i64; 3] {
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
let a = plckr(1, &[1, 2, 3], -1, &[3, 4, 5]);
assert_eq!(a, [-2, -2, -2]);
```
*/
#[inline]
pub fn plckr(ld: i64, p: &[i64; 3], mu: i64, q: &[i64; 3]) -> [i64; 3] {
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
            pub coord: [i64; 3],
        }

        impl $point {
            #[inline]
            pub fn new(coord: [i64; 3]) -> Self {
                Self { coord }
            }
        }

        impl PartialEq for $point {
            fn eq(&self, other: &$point) -> bool {
                cross(&self.coord, &other.coord) == [0, 0, 0]
            }
        }
        impl Eq for $point {}
    }
}

macro_rules! define_line_for_point {
    (impl $line:ident, $point:ident) => {
        impl ProjPlane<$line, i64> for $point {
            fn aux(&self) -> $line {
                $line::new(self.coord.clone())
            }

            fn dot(&self, line: &$line) -> i64 {
                dot(&self.coord, &line.coord)
            } // basic measurement

            fn plucker(ld: i64, p: &Self, mu: i64, q: &Self) -> Self {
                Self::new(plckr(ld, &p.coord, mu, &q.coord))
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
    }
}

macro_rules! define_point_and_line {
    (impl $point:ident, $line:ident) => {
        define_point_or_line!(impl $point);
        define_point_or_line!(impl $line);
        define_line_for_point!(impl $line, $point);
        define_line_for_point!(impl $point, $line);
    }
}

define_point_and_line!(impl PgPoint, PgLine);
define_point_and_line!(impl HypPoint, HypLine);
define_point_and_line!(impl EllPoint, EllLine);
