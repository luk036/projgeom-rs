[![Crates.io](https://img.shields.io/crates/v/projgeom-rs.svg)](https://crates.io/crates/projgeom-rs)
[![Docs.rs](https://docs.rs/projgeom-rs/badge.svg)](https://docs.rs/projgeom-rs)
[![CI](https://github.com/luk036/projgeom-rs/workflows/CI/badge.svg)](https://github.com/luk036/projgeom-rs/actions)
[![codecov](https://codecov.io/gh/luk036/projgeom-rs/branch/master/graph/badge.svg?token=b6IwWju7J6)](https://codecov.io/gh/luk036/projgeom-rs)

<p align="center">
  <img src="./projective-geometry.svg" />
</p>

# 🔯 projgeom-rs

Projective geometry in Rust - A comprehensive library for projective, elliptic, hyperbolic, and Euclidean geometry computations.

## 🛠️ Installation

### 📦 Cargo

- Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
- run `cargo install projgeom-rs`

### Add to your project

```toml
[dependencies]
projgeom-rs = "0.1"
```

## 📚 Usage Examples

### Projective Geometry

```rust
use projgeom_rs::{PgPoint, PgLine, ProjectivePlane};

// Create two points in projective space
let p1 = PgPoint::new([1, 2, 3]);
let p2 = PgPoint::new([4, 5, 6]);

// Find the line through two points (meet operation)
let line = p1.meet(&p2);

// Check if a point lies on a line
assert!(line.incident(&p1));
assert!(line.incident(&p2));

// Parametrize a point on the line
let p3 = p1.parametrize(2, &p2, 3);
assert!(line.incident(&p3));

// Find the intersection of two lines
let l1 = PgLine::new([1, 0, 0]);
let l2 = PgLine::new([0, 1, 0]);
let intersection = l1.meet(&l2);
```

### Elliptic Geometry

```rust
use projgeom_rs::{EllipticPoint, EllipticLine, CayleyKleinPlane};

// Create points in elliptic geometry
let a1 = EllipticPoint::new([1, 0, 0]);
let a2 = EllipticPoint::new([0, 1, 0]);
let a3 = EllipticPoint::new([0, 0, 1]);

// Compute perpendicular line
let perp_line = a1.perp();

// Compute orthocenter of a triangle
let triangle = [a1, a2, a3];
let orthocenter = orthocenter(&triangle);

// Compute altitudes
let altitudes = tri_altitude(&triangle);
```

### Hyperbolic Geometry

```rust
use projgeom_rs::{HyperbolicPoint, HyperbolicLine, CayleyKleinPlane};

// Create points in hyperbolic geometry
let p1 = HyperbolicPoint::new([1, 0, 1]);
let p2 = HyperbolicPoint::new([0, 1, 1]);

// Find the line through two points
let line = p1.meet(&p2);

// Compute perpendicular
let perp = p1.perp();
```

### Euclidean Geometry

```rust
use projgeom_rs::{EuclidPoint, EuclidLine, CayleyKleinPlane};

// Create points in Euclidean geometry
let p1 = EuclidPoint::new([1, 2, 1]);
let p2 = EuclidPoint::new([3, 4, 1]);

// Compute distance between points
let line = p1.meet(&p2);

// Reflect a point across a line
let mirror = EuclidLine::new([1, -1, 0]);
let reflected = reflect(&mirror, &p1);
```

### Harmonic Conjugates

```rust
use projgeom_rs::{PgPoint, harm_conj};

// Compute harmonic conjugate
let a = PgPoint::new([1, 0, 0]);
let b = PgPoint::new([0, 1, 0]);
let c = PgPoint::new([1, 1, 0]);

let d = harm_conj(&a, &b, &c);
// d is the harmonic conjugate of c with respect to a and b
```

### Projective Transformations

```rust
use projgeom_rs::{PgPoint, involution};

// Apply involution transformation
let origin = PgPoint::new([1, 0, 0]);
let mirror = PgLine::new([0, 1, 0]);
let point = PgPoint::new([1, 1, 1]);

let transformed = involution(&origin, &mirror, &point);
```

## 📜 License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
