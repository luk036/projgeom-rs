//! FFI (Foreign Function Interface) layer for C/C++ integration
//!
//! This module provides C-compatible functions for interfacing with
//! the projgeom-rs library from C or C++ code.

use crate::pg_object::{PgPoint, PgLine};
use crate::pg_plane::ProjectivePlanePrimitive;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

/// Opaque pointer to a PgPoint
#[repr(C)]
pub struct PgPointFFI {
    _private: [u8; 0],
}

/// Opaque pointer to a PgLine
#[repr(C)]
pub struct PgLineFFI {
    _private: [u8; 0],
}

/// Create a new point from coordinates
///
/// # Safety
///
/// The returned pointer must be freed using `pg_point_free`.
#[no_mangle]
pub extern "C" fn pg_point_new(x: i64, y: i64, z: i64) -> *mut PgPointFFI {
    let point = Box::new(PgPoint::new([x, y, z]));
    Box::into_raw(point) as *mut PgPointFFI
}

/// Free a point
///
/// # Safety
///
/// The pointer must have been created by `pg_point_new`.
#[no_mangle]
pub unsafe extern "C" fn pg_point_free(ptr: *mut PgPointFFI) {
    if !ptr.is_null() {
        let _ = Box::from_raw(ptr as *mut PgPoint);
    }
}

/// Get point coordinates
///
/// # Safety
///
/// The pointer must be valid and non-null.
#[no_mangle]
pub unsafe extern "C" fn pg_point_get_coords(
    ptr: *const PgPointFFI,
    x: *mut i64,
    y: *mut i64,
    z: *mut i64,
) -> c_int {
    if ptr.is_null() || x.is_null() || y.is_null() || z.is_null() {
        return -1;
    }

    let point = &*(ptr as *const PgPoint);
    *x = point.coord[0];
    *y = point.coord[1];
    *z = point.coord[2];

    0
}

/// Create a new line from coefficients
///
/// # Safety
///
/// The returned pointer must be freed using `pg_line_free`.
#[no_mangle]
pub extern "C" fn pg_line_new(a: i64, b: i64, c: i64) -> *mut PgLineFFI {
    let line = Box::new(PgLine::new([a, b, c]));
    Box::into_raw(line) as *mut PgLineFFI
}

/// Free a line
///
/// # Safety
///
/// The pointer must have been created by `pg_line_new`.
#[no_mangle]
pub unsafe extern "C" fn pg_line_free(ptr: *mut PgLineFFI) {
    if !ptr.is_null() {
        let _ = Box::from_raw(ptr as *mut PgLine);
    }
}

/// Get line coefficients
///
/// # Safety
///
/// The pointer must be valid and non-null.
#[no_mangle]
pub unsafe extern "C" fn pg_line_get_coeffs(
    ptr: *const PgLineFFI,
    a: *mut i64,
    b: *mut i64,
    c: *mut i64,
) -> c_int {
    if ptr.is_null() || a.is_null() || b.is_null() || c.is_null() {
        return -1;
    }

    let line = &*(ptr as *const PgLine);
    *a = line.coord[0];
    *b = line.coord[1];
    *c = line.coord[2];

    0
}

/// Compute the line through two points (meet operation)
///
/// # Safety
///
/// All pointers must be valid and non-null.
/// The returned pointer must be freed using `pg_line_free`.
#[no_mangle]
pub unsafe extern "C" fn pg_point_meet(
    p1: *const PgPointFFI,
    p2: *const PgPointFFI,
) -> *mut PgLineFFI {
    if p1.is_null() || p2.is_null() {
        return std::ptr::null_mut();
    }

    let point1 = &*(p1 as *const PgPoint);
    let point2 = &*(p2 as *const PgPoint);

    let line = point1.meet(point2);
    Box::into_raw(Box::new(line)) as *mut PgLineFFI
}

/// Compute the intersection of two lines
///
/// # Safety
///
/// All pointers must be valid and non-null.
/// The returned pointer must be freed using `pg_point_free`.
#[no_mangle]
pub unsafe extern "C" fn pg_line_meet(
    l1: *const PgLineFFI,
    l2: *const PgLineFFI,
) -> *mut PgPointFFI {
    if l1.is_null() || l2.is_null() {
        return std::ptr::null_mut();
    }

    let line1 = &*(l1 as *const PgLine);
    let line2 = &*(l2 as *const PgLine);

    let point = line1.meet(line2);
    Box::into_raw(Box::new(point)) as *mut PgPointFFI
}

/// Check if a point is incident with a line
///
/// # Safety
///
/// All pointers must be valid and non-null.
#[no_mangle]
pub unsafe extern "C" fn pg_point_incident(
    point: *const PgPointFFI,
    line: *const PgLineFFI,
) -> c_int {
    if point.is_null() || line.is_null() {
        return 0;
    }

    let p = &*(point as *const PgPoint);
    let l = &*(line as *const PgLine);

    if p.incident(l) { 1 } else { 0 }
}

/// Check if two points are equal
///
/// # Safety
///
/// All pointers must be valid and non-null.
#[no_mangle]
pub unsafe extern "C" fn pg_point_eq(
    p1: *const PgPointFFI,
    p2: *const PgPointFFI,
) -> c_int {
    if p1.is_null() || p2.is_null() {
        return 0;
    }

    let point1 = &*(p1 as *const PgPoint);
    let point2 = &*(p2 as *const PgPoint);

    if point1 == point2 { 1 } else { 0 }
}

/// Check if two lines are equal
///
/// # Safety
///
/// All pointers must be valid and non-null.
#[no_mangle]
pub unsafe extern "C" fn pg_line_eq(
    l1: *const PgLineFFI,
    l2: *const PgLineFFI,
) -> c_int {
    if l1.is_null() || l2.is_null() {
        return 0;
    }

    let line1 = &*(l1 as *const PgLine);
    let line2 = &*(l2 as *const PgLine);

    if line1 == line2 { 1 } else { 0 }
}

/// Get the last error message
///
/// # Safety
///
/// The returned string must not be freed by the caller.
#[no_mangle]
pub extern "C" fn pg_get_last_error() -> *const c_char {
    static mut LAST_ERROR: Option<CString> = None;

    unsafe {
        match &LAST_ERROR {
            Some(s) => s.as_ptr(),
            None => std::ptr::null(),
        }
    }
}

/// Set the last error message
///
/// # Safety
///
/// The message string must be valid and null-terminated.
#[no_mangle]
pub unsafe extern "C" fn pg_set_last_error(message: *const c_char) {
    static mut LAST_ERROR: Option<CString> = None;

    if !message.is_null() {
        if let Ok(msg) = CStr::from_ptr(message).to_str() {
            LAST_ERROR = Some(CString::new(msg).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_point_creation() {
        let point = unsafe { pg_point_new(1, 2, 3) };
        assert!(!point.is_null());

        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        let result = unsafe { pg_point_get_coords(point, &mut x, &mut y, &mut z) };
        assert_eq!(result, 0);
        assert_eq!(x, 1);
        assert_eq!(y, 2);
        assert_eq!(z, 3);

        unsafe { pg_point_free(point) };
    }

    #[test]
    fn test_ffi_line_creation() {
        let line = unsafe { pg_line_new(1, 2, 3) };
        assert!(!line.is_null());

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let result = unsafe { pg_line_get_coeffs(line, &mut a, &mut b, &mut c) };
        assert_eq!(result, 0);
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);

        unsafe { pg_line_free(line) };
    }

    #[test]
    fn test_ffi_meet() {
        let p1 = unsafe { pg_point_new(1, 0, 0) };
        let p2 = unsafe { pg_point_new(0, 1, 0) };

        let line = unsafe { pg_point_meet(p1, p2) };
        assert!(!line.is_null());

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        unsafe { pg_line_get_coeffs(line, &mut a, &mut b, &mut c) };
        assert_eq!(a, 0);
        assert_eq!(b, 0);
        assert_eq!(c, 1);

        unsafe { pg_point_free(p1) };
        unsafe { pg_point_free(p2) };
        unsafe { pg_line_free(line) };
    }

    #[test]
    fn test_ffi_incident() {
        // Point (1, 2, 3) and line (3, -1, 1) satisfy: 3*1 + (-1)*2 + 1*3 = 3 - 2 + 3 = 4 ≠ 0
        // Let's use point (1, 1, 1) and line (1, 1, -2): 1*1 + 1*1 + (-2)*1 = 0 ✓
        let p = unsafe { pg_point_new(1, 1, 1) };
        let l = unsafe { pg_line_new(1, 1, -2) };

        let incident = unsafe { pg_point_incident(p, l) };
        assert_eq!(incident, 1);

        unsafe { pg_point_free(p) };
        unsafe { pg_line_free(l) };
    }
}