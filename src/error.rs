//! Error types for geometric operations
//!
//! This module defines error types that can occur during geometric computations,
//! such as overflow, division by zero, and invalid coordinates.

use std::fmt;

/// Error type for geometric operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeometryError {
    /// Arithmetic overflow occurred
    Overflow(String),
    /// Division by zero attempted
    DivisionByZero,
    /// Invalid homogeneous coordinates (all zeros)
    InvalidCoordinates,
    /// Point is at infinity when affine coordinates are required
    PointAtInfinity,
    /// Points are coincident when they should be distinct
    CoincidentPoints,
    /// Lines are coincident when they should be distinct
    CoincidentLines,
    /// Points are not collinear when they should be
    NotCollinear,
    /// Invalid triangle (points are collinear)
    InvalidTriangle,
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::Overflow(msg) => write!(f, "Arithmetic overflow: {}", msg),
            GeometryError::DivisionByZero => write!(f, "Division by zero"),
            GeometryError::InvalidCoordinates => {
                write!(f, "Invalid homogeneous coordinates (all zeros)")
            }
            GeometryError::PointAtInfinity => write!(f, "Point is at infinity"),
            GeometryError::CoincidentPoints => write!(f, "Points are coincident"),
            GeometryError::CoincidentLines => write!(f, "Lines are coincident"),
            GeometryError::NotCollinear => write!(f, "Points are not collinear"),
            GeometryError::InvalidTriangle => write!(f, "Invalid triangle (collinear points)"),
        }
    }
}

impl std::error::Error for GeometryError {}

/// Result type for geometric operations
pub type Result<T> = std::result::Result<T, GeometryError>;

/// Checked addition that returns a GeometryError on overflow
#[inline]
pub fn checked_add(a: i64, b: i64, context: &str) -> Result<i64> {
    a.checked_add(b)
        .ok_or_else(|| GeometryError::Overflow(format!("{}: {} + {}", context, a, b)))
}

/// Checked subtraction that returns a GeometryError on overflow
#[inline]
pub fn checked_sub(a: i64, b: i64, context: &str) -> Result<i64> {
    a.checked_sub(b)
        .ok_or_else(|| GeometryError::Overflow(format!("{}: {} - {}", context, a, b)))
}

/// Checked multiplication that returns a GeometryError on overflow
#[inline]
pub fn checked_mul(a: i64, b: i64, context: &str) -> Result<i64> {
    a.checked_mul(b)
        .ok_or_else(|| GeometryError::Overflow(format!("{}: {} * {}", context, a, b)))
}

/// Checked division that returns a GeometryError on division by zero
#[inline]
pub fn checked_div(a: i64, b: i64, context: &str) -> Result<i64> {
    if b == 0 {
        return Err(GeometryError::DivisionByZero);
    }
    a.checked_div(b)
        .ok_or_else(|| GeometryError::Overflow(format!("{}: {} / {}", context, a, b)))
}

/// Validate homogeneous coordinates (not all zeros)
#[inline]
pub fn validate_coords(coord: &[i64; 3]) -> Result<()> {
    if coord[0] == 0 && coord[1] == 0 && coord[2] == 0 {
        Err(GeometryError::InvalidCoordinates)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_add_success() {
        let result = checked_add(5, 3, "test");
        assert_eq!(result, Ok(8));
    }

    #[test]
    fn test_checked_add_overflow() {
        let result = checked_add(i64::MAX, 1, "test");
        assert!(matches!(result, Err(GeometryError::Overflow(_))));
    }

    #[test]
    fn test_checked_div_by_zero() {
        let result = checked_div(5, 0, "test");
        assert_eq!(result, Err(GeometryError::DivisionByZero));
    }

    #[test]
    fn test_validate_coords_valid() {
        let result = validate_coords(&[1, 2, 3]);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_validate_coords_invalid() {
        let result = validate_coords(&[0, 0, 0]);
        assert_eq!(result, Err(GeometryError::InvalidCoordinates));
    }
}
