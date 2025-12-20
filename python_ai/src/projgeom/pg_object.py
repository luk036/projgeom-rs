"""Projective Geometry Objects module.

This module defines basic geometric objects like points and lines
with their associated operations.
"""

from __future__ import annotations

from typing import TypeVar

T = TypeVar("T", bound=int)


def dot_product(v_a: list[int], v_b: list[int]) -> int:
    """Calculate the dot product of two 3-dimensional vectors."""
    if len(v_a) != 3 or len(v_b) != 3:
        msg = "Both vectors must have length 3"
        raise ValueError(msg)
    return v_a[0] * v_b[0] + v_a[1] * v_b[1] + v_a[2] * v_b[2]


def dot1(v_a: list[int], v_b: list[int]) -> int:
    """Dot product (2d)."""
    if len(v_a) != 2 or len(v_b) != 2:
        msg = "Both vectors must have length 2"
        raise ValueError(msg)
    return v_a[0] * v_b[0] + v_a[1] * v_b[1]


def cross2(v_a: list[int], v_b: list[int]) -> int:
    """Cross product (2d)."""
    if len(v_a) != 2 or len(v_b) != 2:
        msg = "Both vectors must have length 2"
        raise ValueError(msg)
    return v_a[0] * v_b[1] - v_a[1] * v_b[0]


def cross_product(v_a: list[int], v_b: list[int]) -> list[int]:
    """Cross product of 3-dimensional vectors."""
    if len(v_a) != 3 or len(v_b) != 3:
        msg = "Both vectors must have length 3"
        raise ValueError(msg)
    return [
        v_a[1] * v_b[2] - v_a[2] * v_b[1],
        v_a[2] * v_b[0] - v_a[0] * v_b[2],
        v_a[0] * v_b[1] - v_a[1] * v_b[0],
    ]


def plucker_operation(
    lambda_a: int,
    v_a: list[int],
    mu_b: int,
    v_b: list[int],
) -> list[int]:
    """Plucker operation."""
    if len(v_a) != 3 or len(v_b) != 3:
        msg = "Both vectors must have length 3"
        raise ValueError(msg)
    return [
        lambda_a * v_a[0] + mu_b * v_b[0],
        lambda_a * v_a[1] + mu_b * v_b[1],
        lambda_a * v_a[2] + mu_b * v_b[2],
    ]


class BaseGeometricObject:
    """Base class for geometric objects with homogeneous coordinates."""

    def __init__(self, coord: list[int]) -> None:
        if len(coord) != 3:
            msg = "Coordinate must have length 3"
            raise ValueError(msg)
        self.coord: list[int] = coord.copy()

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.coord})"


def _define_point_or_line(cls_name: str) -> type:
    """Factory function to create point or line classes."""

    class GeometricObject(BaseGeometricObject):
        def __init__(self, coord: list[int]) -> None:
            super().__init__(coord)

        def __eq__(self, other: object) -> bool:
            if not isinstance(other, GeometricObject):
                return False
            return cross_product(self.coord, other.coord) == [0, 0, 0]

        def __hash__(self) -> int:
            return hash(tuple(self.coord))

    GeometricObject.__name__ = cls_name
    return GeometricObject


def _define_line_for_point(
    line_cls: type,
    point_cls: type,
) -> None:
    """Define ProjectivePlane methods for a point class."""

    def aux(self) -> line_cls:
        """Return the Dual not incident with Self."""
        return line_cls(self.coord.copy())

    def dot(self, line: line_cls) -> int:
        """Return the dot product of Self and `line`."""
        return dot_product(self.coord, line.coord)

    def parametrize(self, lambda_val: int, pt_q: point_cls, mu_val: int) -> point_cls:
        """Parametrize between self and another point."""
        return point_cls(plucker_operation(lambda_val, self.coord, mu_val, pt_q.coord))

    def incident(self, line: line_cls) -> bool:
        """Check if point is incident with line."""
        return dot_product(self.coord, line.coord) == 0

    def meet(self, other: point_cls) -> line_cls:
        """Meet operation (join of two points)."""
        return line_cls(cross_product(self.coord, other.coord))

    # Assign methods to point class
    point_cls.aux = aux
    point_cls.dot = dot
    point_cls.parametrize = parametrize
    point_cls.incident = incident
    point_cls.meet = meet


def _define_point_for_line(
    point_cls: type,
    line_cls: type,
) -> None:
    """Define ProjectivePlane methods for a line class."""

    def aux(self) -> point_cls:
        """Return the Dual not incident with Self."""
        return point_cls(self.coord.copy())

    def dot(self, point: point_cls) -> int:
        """Return the dot product of Self and `point`."""
        return dot_product(self.coord, point.coord)

    def parametrize(self, lambda_val: int, ln_q: line_cls, mu_val: int) -> line_cls:
        """Parametrize between self and another line."""
        return line_cls(plucker_operation(lambda_val, self.coord, mu_val, ln_q.coord))

    def incident(self, point: point_cls) -> bool:
        """Check if line is incident with point."""
        return dot_product(self.coord, point.coord) == 0

    def meet(self, other: line_cls) -> point_cls:
        """Meet operation (intersection of two lines)."""
        return point_cls(cross_product(self.coord, other.coord))

    # Assign methods to line class
    line_cls.aux = aux
    line_cls.dot = dot
    line_cls.parametrize = parametrize
    line_cls.incident = incident
    line_cls.meet = meet


def _define_point_and_line_pair(point_name: str, line_name: str) -> tuple[type, type]:
    """Create a pair of point and line classes with all required methods."""
    point_cls = _define_point_or_line(point_name)
    line_cls = _define_point_or_line(line_name)

    _define_line_for_point(line_cls, point_cls)
    _define_point_for_line(point_cls, line_cls)

    return point_cls, line_cls


# Create all point and line pairs
PgPoint, PgLine = _define_point_and_line_pair("PgPoint", "PgLine")
HyperbolicPoint, HyperbolicLine = _define_point_and_line_pair("HyperbolicPoint", "HyperbolicLine")
EllipticPoint, EllipticLine = _define_point_and_line_pair("EllipticPoint", "EllipticLine")
MyCKPoint, MyCKLine = _define_point_and_line_pair("MyCKPoint", "MyCKLine")
PerspPoint, PerspLine = _define_point_and_line_pair("PerspPoint", "PerspLine")
EuclidPoint, EuclidLine = _define_point_and_line_pair("EuclidPoint", "EuclidLine")
