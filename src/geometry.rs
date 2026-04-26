//! Unified Geometry trait for all geometry types
//!
//! This module provides a unified `Geometry` trait that encompasses all geometry types
//! in the library: projective, elliptic, hyperbolic, and Euclidean.

use crate::pg_object::{
    EllipticLine, EllipticPoint, EuclidLine, EuclidPoint, HyperbolicLine, HyperbolicPoint, PgLine,
    PgPoint,
};

/// Unified Geometry trait
///
/// This trait provides a common interface for all geometry types,
/// allowing generic operations across different geometries.
pub trait Geometry: Sized {
    /// Returns the name of this geometry type
    fn geometry_name(&self) -> &str;

    /// Returns the homogeneous coordinates
    fn coord(&self) -> &[i64; 3];
}

impl Geometry for PgPoint {
    fn geometry_name(&self) -> &str {
        "Projective"
    }

    fn coord(&self) -> &[i64; 3] {
        &self.coord
    }
}

impl Geometry for PgLine {
    fn geometry_name(&self) -> &str {
        "Projective"
    }

    fn coord(&self) -> &[i64; 3] {
        &self.coord
    }
}

impl Geometry for EllipticPoint {
    fn geometry_name(&self) -> &str {
        "Elliptic"
    }

    fn coord(&self) -> &[i64; 3] {
        &self.coord
    }
}

impl Geometry for EllipticLine {
    fn geometry_name(&self) -> &str {
        "Elliptic"
    }

    fn coord(&self) -> &[i64; 3] {
        &self.coord
    }
}

impl Geometry for HyperbolicPoint {
    fn geometry_name(&self) -> &str {
        "Hyperbolic"
    }

    fn coord(&self) -> &[i64; 3] {
        &self.coord
    }
}

impl Geometry for HyperbolicLine {
    fn geometry_name(&self) -> &str {
        "Hyperbolic"
    }

    fn coord(&self) -> &[i64; 3] {
        &self.coord
    }
}

impl Geometry for EuclidPoint {
    fn geometry_name(&self) -> &str {
        "Euclidean"
    }

    fn coord(&self) -> &[i64; 3] {
        &self.coord
    }
}

impl Geometry for EuclidLine {
    fn geometry_name(&self) -> &str {
        "Euclidean"
    }

    fn coord(&self) -> &[i64; 3] {
        &self.coord
    }
}

use std::convert::TryFrom;

impl TryFrom<PgPoint> for EllipticPoint {
    type Error = crate::error::GeometryError;

    fn try_from(value: PgPoint) -> std::result::Result<Self, Self::Error> {
        Ok(EllipticPoint::new(value.coord))
    }
}

impl TryFrom<PgPoint> for HyperbolicPoint {
    type Error = crate::error::GeometryError;

    fn try_from(value: PgPoint) -> std::result::Result<Self, Self::Error> {
        Ok(HyperbolicPoint::new(value.coord))
    }
}

impl TryFrom<PgPoint> for EuclidPoint {
    type Error = crate::error::GeometryError;

    fn try_from(value: PgPoint) -> std::result::Result<Self, Self::Error> {
        Ok(EuclidPoint::new(value.coord))
    }
}

impl TryFrom<EllipticPoint> for PgPoint {
    type Error = crate::error::GeometryError;

    fn try_from(value: EllipticPoint) -> std::result::Result<Self, Self::Error> {
        Ok(PgPoint::new(value.coord))
    }
}

impl TryFrom<HyperbolicPoint> for PgPoint {
    type Error = crate::error::GeometryError;

    fn try_from(value: HyperbolicPoint) -> std::result::Result<Self, Self::Error> {
        Ok(PgPoint::new(value.coord))
    }
}

impl TryFrom<EuclidPoint> for PgPoint {
    type Error = crate::error::GeometryError;

    fn try_from(value: EuclidPoint) -> std::result::Result<Self, Self::Error> {
        Ok(PgPoint::new(value.coord))
    }
}

impl TryFrom<PgLine> for EllipticLine {
    type Error = crate::error::GeometryError;

    fn try_from(value: PgLine) -> std::result::Result<Self, Self::Error> {
        Ok(EllipticLine::new(value.coord))
    }
}

impl TryFrom<PgLine> for HyperbolicLine {
    type Error = crate::error::GeometryError;

    fn try_from(value: PgLine) -> std::result::Result<Self, Self::Error> {
        Ok(HyperbolicLine::new(value.coord))
    }
}

impl TryFrom<PgLine> for EuclidLine {
    type Error = crate::error::GeometryError;

    fn try_from(value: PgLine) -> std::result::Result<Self, Self::Error> {
        Ok(EuclidLine::new(value.coord))
    }
}

impl TryFrom<EllipticLine> for PgLine {
    type Error = crate::error::GeometryError;

    fn try_from(value: EllipticLine) -> std::result::Result<Self, Self::Error> {
        Ok(PgLine::new(value.coord))
    }
}

impl TryFrom<HyperbolicLine> for PgLine {
    type Error = crate::error::GeometryError;

    fn try_from(value: HyperbolicLine) -> std::result::Result<Self, Self::Error> {
        Ok(PgLine::new(value.coord))
    }
}

impl TryFrom<EuclidLine> for PgLine {
    type Error = crate::error::GeometryError;

    fn try_from(value: EuclidLine) -> std::result::Result<Self, Self::Error> {
        Ok(PgLine::new(value.coord))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pgpoint_geometry_name() {
        let p = PgPoint::new([1, 2, 3]);
        assert_eq!(p.geometry_name(), "Projective");
    }

    #[test]
    fn test_ellipticpoint_geometry_name() {
        let p = EllipticPoint::new([1, 0, 0]);
        assert_eq!(p.geometry_name(), "Elliptic");
    }

    #[test]
    fn test_hyperbolicpoint_geometry_name() {
        let p = HyperbolicPoint::new([1, 0, 1]);
        assert_eq!(p.geometry_name(), "Hyperbolic");
    }

    #[test]
    fn test_euclidpoint_geometry_name() {
        let p = EuclidPoint::new([1, 2, 1]);
        assert_eq!(p.geometry_name(), "Euclidean");
    }

    #[test]
    fn test_geometry_coord() {
        let p = PgPoint::new([1, 2, 3]);
        assert_eq!(p.coord(), &[1, 2, 3]);
    }

    #[test]
    fn test_try_from_pgpoint_to_elliptic_point() {
        let pg = PgPoint::new([1, 2, 3]);
        let elliptic: EllipticPoint = pg.try_into().unwrap();
        assert_eq!(elliptic.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_pgpoint_to_hyperbolic_point() {
        let pg = PgPoint::new([1, 2, 3]);
        let hyperbolic: HyperbolicPoint = pg.try_into().unwrap();
        assert_eq!(hyperbolic.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_pgpoint_to_euclid_point() {
        let pg = PgPoint::new([1, 2, 3]);
        let euclid: EuclidPoint = pg.try_into().unwrap();
        assert_eq!(euclid.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_elliptic_point_to_pgpoint() {
        let elliptic = EllipticPoint::new([1, 2, 3]);
        let pg: PgPoint = elliptic.try_into().unwrap();
        assert_eq!(pg.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_hyperbolic_point_to_pgpoint() {
        let hyperbolic = HyperbolicPoint::new([1, 2, 3]);
        let pg: PgPoint = hyperbolic.try_into().unwrap();
        assert_eq!(pg.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_euclid_point_to_pgpoint() {
        let euclid = EuclidPoint::new([1, 2, 3]);
        let pg: PgPoint = euclid.try_into().unwrap();
        assert_eq!(pg.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_pgline_to_elliptic_line() {
        let pg = PgLine::new([1, 2, 3]);
        let elliptic: EllipticLine = pg.try_into().unwrap();
        assert_eq!(elliptic.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_pgline_to_hyperbolic_line() {
        let pg = PgLine::new([1, 2, 3]);
        let hyperbolic: HyperbolicLine = pg.try_into().unwrap();
        assert_eq!(hyperbolic.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_pgline_to_euclid_line() {
        let pg = PgLine::new([1, 2, 3]);
        let euclid: EuclidLine = pg.try_into().unwrap();
        assert_eq!(euclid.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_elliptic_line_to_pgline() {
        let elliptic = EllipticLine::new([1, 2, 3]);
        let pg: PgLine = elliptic.try_into().unwrap();
        assert_eq!(pg.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_hyperbolic_line_to_pgline() {
        let hyperbolic = HyperbolicLine::new([1, 2, 3]);
        let pg: PgLine = hyperbolic.try_into().unwrap();
        assert_eq!(pg.coord, [1, 2, 3]);
    }

    #[test]
    fn test_try_from_euclid_line_to_pgline() {
        let euclid = EuclidLine::new([1, 2, 3]);
        let pg: PgLine = euclid.try_into().unwrap();
        assert_eq!(pg.coord, [1, 2, 3]);
    }

    #[test]
    fn test_pgline_geometry_name() {
        let l = PgLine::new([1, 0, 0]);
        assert_eq!(l.geometry_name(), "Projective");
    }

    #[test]
    fn test_ellipticline_geometry_name() {
        let l = EllipticLine::new([1, 0, 0]);
        assert_eq!(l.geometry_name(), "Elliptic");
    }

    #[test]
    fn test_hyperbolicline_geometry_name() {
        let l = HyperbolicLine::new([1, 0, 0]);
        assert_eq!(l.geometry_name(), "Hyperbolic");
    }

    #[test]
    fn test_euclidline_geometry_name() {
        let l = EuclidLine::new([1, 2, 1]);
        assert_eq!(l.geometry_name(), "Euclidean");
    }

    #[test]
    fn test_pgline_coord() {
        let l = PgLine::new([1, 2, 3]);
        assert_eq!(l.coord(), &[1, 2, 3]);
    }

    #[test]
    fn test_ellipticline_coord() {
        let l = EllipticLine::new([1, 2, 3]);
        assert_eq!(l.coord(), &[1, 2, 3]);
    }

    #[test]
    fn test_hyperbolicline_coord() {
        let l = HyperbolicLine::new([1, 2, 3]);
        assert_eq!(l.coord(), &[1, 2, 3]);
    }

    #[test]
    fn test_euclidline_coord() {
        let l = EuclidLine::new([1, 2, 3]);
        assert_eq!(l.coord(), &[1, 2, 3]);
    }
}
