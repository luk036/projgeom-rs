"""Euclidean Geometry Objects module.

This module defines Euclidean geometry points and lines with their
specialized operations.
"""

from .ck_plane import CayleyKleinPlane
from .pg_object import EuclidLine, EuclidPoint, dot1
from .pg_plane import coincident, tri_dual

# Line at infinity for Euclidean geometry
L_INF = EuclidLine([0, 0, 1])


def _perp_point(point: EuclidPoint) -> EuclidLine:
    """Return the polar line of a Euclidean point (always the line at infinity)."""
    return EuclidLine(L_INF.coord.copy())


def _perp_line(line: EuclidLine) -> EuclidPoint:
    """Return the pole of a Euclidean line."""
    return EuclidPoint([line.coord[0], line.coord[1], 0])


# Implement CayleyKleinPlanePrimitive for EuclidPoint
EuclidPoint.perp = _perp_point  # type: ignore[attr-defined]

# Implement CayleyKleinPlanePrimitive for EuclidLine
EuclidLine.perp = _perp_line  # type: ignore[attr-defined]

# Implement CayleyKleinPlane for EuclidPoint and EuclidLine
CayleyKleinPlane.register(EuclidPoint)
CayleyKleinPlane.register(EuclidLine)


def _is_parallel(self: EuclidLine, other: EuclidLine) -> bool:
    """Check if two Euclidean lines are parallel."""
    return self.coord[0] * other.coord[1] == self.coord[1] * other.coord[0]


def _is_perpendicular(self: EuclidLine, other: EuclidLine) -> bool:
    """Check if two Euclidean lines are perpendicular."""
    return dot1(self.coord[:2], other.coord[:2]) == 0


def _altitude(self: EuclidLine, pt_a: EuclidPoint) -> EuclidLine:
    """Calculate the perpendicular line from a given point to a line."""
    return self.perp().meet(pt_a)


def _midpoint(self: EuclidPoint, other: EuclidPoint) -> EuclidPoint:
    """Calculate the midpoint between two Euclidean points."""
    return self.parametrize(other.coord[2], other, self.coord[2])


# Add methods to EuclidLine
EuclidLine.is_parallel = _is_parallel  # type: ignore[attr-defined]
EuclidLine.is_perpendicular = _is_perpendicular  # type: ignore[attr-defined]
EuclidLine.altitude = _altitude  # type: ignore[attr-defined]

# Add methods to EuclidPoint
EuclidPoint.midpoint = _midpoint  # type: ignore[attr-defined]


def tri_altitude(triangle: list[EuclidPoint]) -> list[EuclidLine]:
    """Calculate the altitudes of a triangle given its three vertices."""
    if len(triangle) != 3:
        msg = "Triangle must have exactly 3 points"
        raise ValueError(msg)

    [l_1, l_2, l_3] = tri_dual(triangle)
    [a_1, a_2, a_3] = triangle
    assert not coincident(a_1, a_2, a_3), "Triangle vertices must not be collinear"
    t_1 = l_1.altitude(a_1)
    t_2 = l_2.altitude(a_2)
    t_3 = l_3.altitude(a_3)
    return [t_1, t_2, t_3]


def orthocenter(triangle: list[EuclidPoint]) -> EuclidPoint:
    """Calculate the orthocenter of a triangle given its three vertices."""
    if len(triangle) != 3:
        msg = "Triangle must have exactly 3 points"
        raise ValueError(msg)

    [a_1, a_2, a_3] = triangle
    assert not coincident(a_1, a_2, a_3), "Triangle vertices must not be collinear"
    t_1 = a_2.meet(a_3).altitude(a_1)
    t_2 = a_3.meet(a_1).altitude(a_2)
    return t_1.meet(t_2)
