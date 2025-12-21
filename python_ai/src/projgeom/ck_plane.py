"""Cayley-Klein Plane module.

This module defines Cayley-Klein plane geometry with perpendicularity
and related operations.
"""

from __future__ import annotations

from typing import TypeVar

from .pg_plane import ProjectivePlane, ProjectivePlanePrimitive, coincident, involution, tri_dual

Dual = TypeVar("Dual")
Value = TypeVar("Value", bound=int)


class CayleyKleinPlanePrimitive[Dual](ProjectivePlanePrimitive[Dual]):
    """The `CayleyKleinPlanePrimitive` trait is a trait that extends the `ProjectivePlanePrimitive` trait.
    It adds an additional method `perp(&self) -> Line` to the trait. This method returns the polar line to the given
    point or the pole of a given line.
    """

    def perp(self) -> Dual:
        """Return the polar line to the given point or the pole of a given line."""
        msg = "perp method must be implemented"
        raise NotImplementedError(msg)


def is_perpendicular(m_1: Line, m_2: Line) -> bool:
    """Check if two lines are perpendicular to each other."""
    return m_1.perp().incident(m_2)


def altitude(pt_p: Point, ln_m: Line) -> Line:
    """Calculate the altitude of a point `pt_p` with respect to a line `ln_m`."""
    return ln_m.perp().meet(pt_p)


def orthocenter(triangle: list[Point]) -> Point:
    """Calculate the orthocenter of a triangle given its three vertices."""
    if len(triangle) != 3:
        msg = "Triangle must have exactly 3 points"
        raise ValueError(msg)

    [a_1, a_2, a_3] = triangle
    assert not coincident(a_1, a_2, a_3), "Triangle vertices must not be collinear"
    t_1 = altitude(a_1, a_2.meet(a_3))
    t_2 = altitude(a_2, a_3.meet(a_1))
    return t_1.meet(t_2)


def tri_altitude(triangle: list[Point]) -> list[Line]:
    """Calculate the altitudes of a triangle given its three vertices."""
    if len(triangle) != 3:
        msg = "Triangle must have exactly 3 points"
        raise ValueError(msg)

    [l_1, l_2, l_3] = tri_dual(triangle)
    [a_1, a_2, a_3] = triangle
    assert not coincident(a_1, a_2, a_3), "Triangle vertices must not be collinear"
    t_1 = altitude(a_1, l_1)
    t_2 = altitude(a_2, l_2)
    t_3 = altitude(a_3, l_3)
    return [t_1, t_2, t_3]


class CayleyKleinPlane[Dual, Value: int](
    ProjectivePlane[Dual, Value],
    CayleyKleinPlanePrimitive[Dual],
):
    """Cayley-Klein plane trait combining ProjectivePlane and CayleyKleinPlanePrimitive."""


def reflect(mirror: Line, pt_p: Point) -> Point:
    """Reflect a point `pt_p` across a mirror plane `mirror`."""
    return involution(mirror.perp(), mirror, pt_p)


# Type aliases for better readability
Point = TypeVar("Point", bound=CayleyKleinPlanePrimitive)
Line = TypeVar("Line", bound=CayleyKleinPlanePrimitive)
