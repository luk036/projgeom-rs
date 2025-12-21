"""Elliptic Geometry Objects module.

This module defines elliptic geometry points and lines with their
perpendicularity operations.
"""

from .ck_plane import CayleyKleinPlane
from .pg_object import EllipticLine, EllipticPoint


def _perp_point(point: EllipticPoint) -> EllipticLine:
    """Return the polar line of an elliptic point."""
    return EllipticLine(point.coord.copy())


def _perp_line(line: EllipticLine) -> EllipticPoint:
    """Return the pole of an elliptic line."""
    return EllipticPoint(line.coord.copy())


# Implement CayleyKleinPlanePrimitive for EllipticPoint
EllipticPoint.perp = _perp_point  # type: ignore[attr-defined]

# Implement CayleyKleinPlanePrimitive for EllipticLine
EllipticLine.perp = _perp_line  # type: ignore[attr-defined]

# Implement CayleyKleinPlane for EllipticPoint and EllipticLine
CayleyKleinPlane.register(EllipticPoint)
CayleyKleinPlane.register(EllipticLine)
