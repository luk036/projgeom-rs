"""
Python bindings for projgeom-rs

This module provides a Python interface to the Rust-based projgeom-rs library
for projective geometry computations.
"""

from typing import List, Tuple, Optional
import ctypes
import os
import sys

# Load the Rust library
if sys.platform == "win32":
    lib_name = "projgeom_rs.dll"
elif sys.platform == "darwin":
    lib_name = "libprojgeom_rs.dylib"
else:
    lib_name = "libprojgeom_rs.so"

# Try to load from the package directory first
try:
    lib_path = os.path.join(os.path.dirname(__file__), lib_name)
    lib = ctypes.CDLL(lib_path)
except OSError:
    # Fall back to system library path
    lib = ctypes.CDLL(lib_name)


# Define FFI types
PgPointFFI = ctypes.c_void_p
PgLineFFI = ctypes.c_void_p
c_int = ctypes.c_int
c_int64 = ctypes.c_int64


# Define FFI functions
lib.pg_point_new.argtypes = [c_int64, c_int64, c_int64]
lib.pg_point_new.restype = PgPointFFI

lib.pg_point_free.argtypes = [PgPointFFI]
lib.pg_point_free.restype = None

lib.pg_point_get_coords.argtypes = [PgPointFFI, ctypes.POINTER(c_int64), ctypes.POINTER(c_int64), ctypes.POINTER(c_int64)]
lib.pg_point_get_coords.restype = c_int

lib.pg_line_new.argtypes = [c_int64, c_int64, c_int64]
lib.pg_line_new.restype = PgLineFFI

lib.pg_line_free.argtypes = [PgLineFFI]
lib.pg_line_free.restype = None

lib.pg_line_get_coeffs.argtypes = [PgLineFFI, ctypes.POINTER(c_int64), ctypes.POINTER(c_int64), ctypes.POINTER(c_int64)]
lib.pg_line_get_coeffs.restype = c_int

lib.pg_point_meet.argtypes = [PgPointFFI, PgPointFFI]
lib.pg_point_meet.restype = PgLineFFI

lib.pg_line_meet.argtypes = [PgLineFFI, PgLineFFI]
lib.pg_line_meet.restype = PgPointFFI

lib.pg_point_incident.argtypes = [PgPointFFI, PgLineFFI]
lib.pg_point_incident.restype = c_int

lib.pg_point_eq.argtypes = [PgPointFFI, PgPointFFI]
lib.pg_point_eq.restype = c_int

lib.pg_line_eq.argtypes = [PgLineFFI, PgLineFFI]
lib.pg_line_eq.restype = c_int


class PgPoint:
    """A point in projective space."""

    def __init__(self, coords: Tuple[int, int, int]):
        """Create a new point from homogeneous coordinates."""
        if len(coords) != 3:
            raise ValueError("Coordinates must be a tuple of 3 integers")
        self._ptr = lib.pg_point_new(coords[0], coords[1], coords[2])

    def __del__(self):
        """Free the point when garbage collected."""
        if hasattr(self, '_ptr') and self._ptr is not None:
            lib.pg_point_free(self._ptr)

    @property
    def coords(self) -> Tuple[int, int, int]:
        """Get the homogeneous coordinates."""
        x = c_int64()
        y = c_int64()
        z = c_int64()
        result = lib.pg_point_get_coords(self._ptr, ctypes.byref(x), ctypes.byref(y), ctypes.byref(z))
        if result != 0:
            raise RuntimeError("Failed to get point coordinates")
        return (x.value, y.value, z.value)

    def meet(self, other: 'PgPoint') -> 'PgLine':
        """Find the line through two points."""
        if not isinstance(other, PgPoint):
            raise TypeError("Other must be a PgPoint")
        line_ptr = lib.pg_point_meet(self._ptr, other._ptr)
        if line_ptr is None:
            raise RuntimeError("Failed to compute meet")
        return PgLine._from_ptr(line_ptr)

    def incident(self, line: 'PgLine') -> bool:
        """Check if the point is incident with the line."""
        if not isinstance(line, PgLine):
            raise TypeError("Line must be a PgLine")
        return lib.pg_point_incident(self._ptr, line._ptr) != 0

    def __eq__(self, other: object) -> bool:
        """Check if two points are equal."""
        if not isinstance(other, PgPoint):
            return False
        return lib.pg_point_eq(self._ptr, other._ptr) != 0

    def __repr__(self) -> str:
        return f"PgPoint({self.coords})"


class PgLine:
    """A line in projective space."""

    def __init__(self, coeffs: Tuple[int, int, int]):
        """Create a new line from coefficients."""
        if len(coeffs) != 3:
            raise ValueError("Coefficients must be a tuple of 3 integers")
        self._ptr = lib.pg_line_new(coeffs[0], coeffs[1], coeffs[2])

    @staticmethod
    def _from_ptr(ptr: ctypes.c_void_p) -> 'PgLine':
        """Create a PgLine from a raw pointer."""
        line = PgLine.__new__(PgLine)
        line._ptr = ptr
        return line

    def __del__(self):
        """Free the line when garbage collected."""
        if hasattr(self, '_ptr') and self._ptr is not None:
            lib.pg_line_free(self._ptr)

    @property
    def coeffs(self) -> Tuple[int, int, int]:
        """Get the line coefficients."""
        a = c_int64()
        b = c_int64()
        c = c_int64()
        result = lib.pg_line_get_coeffs(self._ptr, ctypes.byref(a), ctypes.byref(b), ctypes.byref(c))
        if result != 0:
            raise RuntimeError("Failed to get line coefficients")
        return (a.value, b.value, c.value)

    def meet(self, other: 'PgLine') -> PgPoint:
        """Find the intersection of two lines."""
        if not isinstance(other, PgLine):
            raise TypeError("Other must be a PgLine")
        point_ptr = lib.pg_line_meet(self._ptr, other._ptr)
        if point_ptr is None:
            raise RuntimeError("Failed to compute meet")
        return PgPoint._from_ptr(point_ptr)

    def incident(self, point: PgPoint) -> bool:
        """Check if the line is incident with the point."""
        if not isinstance(point, PgPoint):
            raise TypeError("Point must be a PgPoint")
        return lib.pg_point_incident(point._ptr, self._ptr) != 0

    def __eq__(self, other: object) -> bool:
        """Check if two lines are equal."""
        if not isinstance(other, PgLine):
            return False
        return lib.pg_line_eq(self._ptr, other._ptr) != 0

    def __repr__(self) -> str:
        return f"PgLine({self.coeffs})"


# Placeholder classes for other geometry types
# These would need additional FFI functions in the Rust code

class EllipticPoint(PgPoint):
    """A point in elliptic geometry."""
    pass


class EllipticLine(PgLine):
    """A line in elliptic geometry."""
    pass


class HyperbolicPoint(PgPoint):
    """A point in hyperbolic geometry."""
    pass


class HyperbolicLine(PgLine):
    """A line in hyperbolic geometry."""
    pass


class EuclidPoint(PgPoint):
    """A point in Euclidean geometry."""
    pass


class EuclidLine(PgLine):
    """A line in Euclidean geometry."""
    pass


class MyCKPoint(PgPoint):
    """A point in custom Cayley-Klein geometry."""
    pass


class MyCKLine(PgLine):
    """A line in custom Cayley-Klein geometry."""
    pass


class PerspPoint(PgPoint):
    """A point in perspective geometry."""
    pass


class PerspLine(PgLine):
    """A line in perspective geometry."""
    pass


# Placeholder functions for geometric operations
# These would need additional FFI functions in the Rust code

def orthocenter(triangle: List[PgPoint]) -> PgPoint:
    """Compute the orthocenter of a triangle."""
    raise NotImplementedError("orthocenter not yet implemented in Python bindings")

def tri_altitude(triangle: List[PgPoint]) -> List[PgLine]:
    """Compute the altitudes of a triangle."""
    raise NotImplementedError("tri_altitude not yet implemented in Python bindings")

def tri_dual(triangle: List[PgPoint]) -> List[PgLine]:
    """Compute the trilateral (dual) of a triangle."""
    raise NotImplementedError("tri_dual not yet implemented in Python bindings")

def is_perpendicular(l1: PgLine, l2: PgLine) -> bool:
    """Check if two lines are perpendicular."""
    raise NotImplementedError("is_perpendicular not yet implemented in Python bindings")

def reflect(mirror: PgLine, point: PgPoint) -> PgPoint:
    """Reflect a point across a line."""
    raise NotImplementedError("reflect not yet implemented in Python bindings")

def harm_conj(a: PgPoint, b: PgPoint, c: PgPoint) -> PgPoint:
    """Compute the harmonic conjugate."""
    raise NotImplementedError("harm_conj not yet implemented in Python bindings")

def coincident(a: PgPoint, b: PgPoint, c: PgPoint) -> bool:
    """Check if three points are collinear."""
    raise NotImplementedError("coincident not yet implemented in Python bindings")

def involution(origin: PgPoint, mirror: PgLine, point: PgPoint) -> PgPoint:
    """Apply an involution transformation."""
    raise NotImplementedError("involution not yet implemented in Python bindings")


if __name__ == "__main__":
    # Example usage
    p1 = PgPoint((1, 2, 3))
    p2 = PgPoint((4, 5, 6))
    line = p1.meet(p2)
    print(f"Line through {p1} and {p2}: {line}")
    print(f"p1 incident with line: {p1.incident(line)}")
    print(f"p2 incident with line: {p2.incident(line)}")