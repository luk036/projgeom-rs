"""Perspective Geometry Objects module.

This module defines perspective geometry points and lines with their
specialized operations.
"""

from .ck_plane import CayleyKleinPlane
from .pg_object import PerspLine, PerspPoint

# Special points and lines for perspective geometry
I_RE = PerspPoint([0, 1, 1])
I_IM = PerspPoint([1, 0, 0])
L_INF = PerspLine([0, -1, 1])


def _perp_point(p: PerspPoint) -> PerspLine:
    """Return the polar line of a perspective point (always the line at infinity)."""
    return PerspLine(L_INF.coord.copy())


def _perp_line(l: PerspLine) -> PerspPoint:
    """Return the pole of a perspective line."""
    alpha = I_RE.dot(l)
    beta = I_IM.dot(l)
    return PerspPoint.parametrize(I_RE, alpha, I_IM, beta)


# Implement CayleyKleinPlanePrimitive for PerspPoint
PerspPoint.perp = _perp_point  # type: ignore[attr-defined]

# Implement CayleyKleinPlanePrimitive for PerspLine
PerspLine.perp = _perp_line  # type: ignore[attr-defined]

# Implement CayleyKleinPlane for PerspPoint and PerspLine
CayleyKleinPlane.register(PerspPoint)
CayleyKleinPlane.register(PerspLine)


def _is_parallel(self: PerspLine, other: PerspLine) -> bool:
    """Check if two perspective lines are parallel."""
    return L_INF.dot(self.meet(other)) == 0


def _midpoint(self: PerspPoint, other: PerspPoint) -> PerspPoint:
    """Calculate the midpoint between two perspective points."""
    alpha = L_INF.dot(other)
    beta = L_INF.dot(self)
    return PerspPoint.parametrize(self, alpha, other, beta)


# Add methods to PerspLine
PerspLine.is_parallel = _is_parallel  # type: ignore[attr-defined]

# Add methods to PerspPoint
PerspPoint.midpoint = _midpoint  # type: ignore[attr-defined]
