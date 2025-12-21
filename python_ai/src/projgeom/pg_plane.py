"""Projective Plane module.

This module defines the core traits and functions for projective plane geometry.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from typing import TypeVar

Dual = TypeVar("Dual")

Value = TypeVar("Value", bound=int)

Self = TypeVar("Self", bound="ProjectivePlanePrimitive")

ProjectivePlaneSelf = TypeVar("ProjectivePlaneSelf", bound="ProjectivePlane")


class ProjectivePlanePrimitive[Dual](ABC):
    """The `ProjectivePlanePrimitive` trait defines the behavior of points and lines in a projective plane.

    It requires two associated types: `Dual`, which represents the dual object (line or point) in the
    projective plane, and `Self`, which represents the object implementing the trait.
    """

    @abstractmethod
    def meet(self, rhs: Self) -> Dual:
        """Join or meet operation."""

        ...

    @abstractmethod
    def incident(self, dual: Dual) -> bool:
        """Incidence relation."""
        ...

    def __eq__(self, other: object) -> bool:
        """Default equality implementation."""
        if not isinstance(other, self.__class__):
            return False
        return self.incident(other) and other.incident(self)

    def __hash__(self) -> int:
        """Default hash implementation."""
        return hash(self.__class__.__name__)


def check_axiom(
    pt_p: ProjectivePlanePrimitive[Line],
    pt_q: ProjectivePlanePrimitive[Line],
    ln_l: ProjectivePlanePrimitive[Point],
) -> None:
    """Check if certain axioms hold for points and lines in a projective plane."""
    assert pt_p.incident(ln_l) == ln_l.incident(pt_p)
    assert pt_p.meet(pt_q) == pt_q.meet(pt_p)
    ln_m = pt_p.meet(pt_q)
    assert ln_m.incident(pt_p)
    assert ln_m.incident(pt_q)


def coincident(
    pt_p: ProjectivePlanePrimitive[Line],
    pt_q: ProjectivePlanePrimitive[Line],
    pt_r: ProjectivePlanePrimitive[Line],
) -> bool:
    """Check if three points `pt_p`, `pt_q`, and `pt_r` are coincident in a projective plane."""
    return pt_p.meet(pt_q).incident(pt_r)


def check_pappus(
    coline_1: list[ProjectivePlanePrimitive[Line]], coline_2: list[ProjectivePlanePrimitive[Line]]
) -> bool:
    """Check if three points on a projective plane and three lines on another
    projective plane satisfy Pappus' theorem.
    """
    if len(coline_1) != 3 or len(coline_2) != 3:
        msg = "Both coline arrays must have exactly 3 points"
        raise ValueError(msg)

    [pt_a, pt_b, pt_c] = coline_1
    [pt_d, pt_e, pt_f] = coline_2
    pt_g = pt_a.meet(pt_e).meet(pt_b.meet(pt_d))
    pt_h = pt_a.meet(pt_f).meet(pt_c.meet(pt_d))
    pt_i = pt_b.meet(pt_f).meet(pt_c.meet(pt_e))
    return coincident(pt_g, pt_h, pt_i)


def tri_dual(
    triangle: list[ProjectivePlanePrimitive[Line]],
) -> list[ProjectivePlanePrimitive[Point]]:
    """The `tri_dual` function takes a triangle and returns an array of lines that are dual to the
    triangle's vertices.

    Arguments:
        triangle: An array of length 3 containing points that define a
                  triangle in a projective plane. Each element of the array represents a vertex of the triangle.

    Returns:
        An array of three elements, where each element is of type `Line`.
    """
    if len(triangle) != 3:
        msg = "Triangle must have exactly 3 points"
        raise ValueError(msg)

    [a_1, a_2, a_3] = triangle
    assert not coincident(a_1, a_2, a_3), "Triangle vertices must not be collinear"
    return [a_2.meet(a_3), a_1.meet(a_3), a_1.meet(a_2)]


def persp(
    tri1: list[ProjectivePlanePrimitive[Line]], tri2: list[ProjectivePlanePrimitive[Line]]
) -> bool:
    """Determine whether two triangles are perspective."""
    if len(tri1) != 3 or len(tri2) != 3:
        msg = "Both triangles must have exactly 3 points"
        raise ValueError(msg)

    [pt_a, pt_b, pt_c] = tri1
    [pt_d, pt_e, pt_f] = tri2
    pt_o = pt_a.meet(pt_d).meet(pt_b.meet(pt_e))
    return pt_c.meet(pt_f).incident(pt_o)


def check_desargue(
    tri1: list[ProjectivePlanePrimitive[Line]], tri2: list[ProjectivePlanePrimitive[Line]]
) -> bool:
    """Check if two triangles satisfy the Desargue's theorem in projective geometry."""
    trid1: list[ProjectivePlanePrimitive[Point]] = tri_dual(tri1)
    trid2: list[ProjectivePlanePrimitive[Point]] = tri_dual(tri2)
    bool1 = persp(tri1, tri2)
    bool2 = persp(trid1, trid2)
    return (bool1 and bool2) or (not bool1 and not bool2)


class ProjectivePlane[Dual, Value](ProjectivePlanePrimitive[Dual], ABC):
    """The `ProjectivePlane` trait is a trait that extends the `ProjectivePlanePrimitive` trait.
    It adds additional `aux`, `dot`, and parametrize methods to the trait.
    """

    @abstractmethod
    def aux(self) -> Dual:
        """Return the Dual not incident with Self."""
        ...

    @abstractmethod
    def dot(self, dual: Dual) -> Value:
        """Return the dot product of Self and `dual` for basic measurement."""
        ...

    @abstractmethod
    def parametrize(self, lambda_val: Value, other: Self, mu_val: Value) -> Self:
        """Parametrize between self and other point."""
        ...


def check_axiom2(
    pt_p: ProjectivePlane[ProjectivePlanePrimitive[Point], Value],
    pt_q: ProjectivePlane[ProjectivePlanePrimitive[Point], Value],
    ln_l: ProjectivePlane[ProjectivePlanePrimitive[Line], Value],
    alpha: Value,
    beta: Value,
) -> None:
    """Check if certain axioms hold true in a projective plane."""
    assert pt_p.dot(ln_l) == ln_l.dot(pt_p)
    assert not pt_p.aux().incident(pt_p)
    ln_m = pt_p.meet(pt_q)
    assert ln_m.incident(pt_p.parametrize(alpha, pt_q, beta))


def harm_conj(
    pt_a: ProjectivePlane[ProjectivePlanePrimitive[Point], Value],
    pt_b: ProjectivePlane[ProjectivePlanePrimitive[Point], Value],
    pt_c: ProjectivePlane[ProjectivePlanePrimitive[Point], Value],
) -> ProjectivePlane[ProjectivePlanePrimitive[Point], Value]:
    """Calculate the harmonic conjugate of three points in a projective plane."""
    assert coincident(pt_a, pt_b, pt_c), "Points must be collinear"
    ln_ab = pt_a.meet(pt_b)
    ln_xc = ln_ab.aux().meet(pt_c)
    return pt_a.parametrize(ln_xc.dot(pt_b), pt_b, ln_xc.dot(pt_a))


def involution(
    origin: ProjectivePlane[ProjectivePlanePrimitive[Point], Value],
    mirror: ProjectivePlane[ProjectivePlanePrimitive[Line], Value],
    pt_p: ProjectivePlane[ProjectivePlanePrimitive[Point], Value],
) -> ProjectivePlane[ProjectivePlanePrimitive[Point], Value]:
    """Perform an involution transformation on a point `pt_p` with respect to an
    origin point `origin` and a mirror line `mirror`.
    """
    ln_po = pt_p.meet(origin)
    pt_b = ln_po.meet(mirror)
    return harm_conj(origin, pt_b, pt_p)


# Type aliases for better readability
Point = TypeVar("Point", bound=ProjectivePlanePrimitive)
Line = TypeVar("Line", bound=ProjectivePlanePrimitive)
