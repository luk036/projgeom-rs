"""MyCK Geometry Objects module.

This module defines MyCK (custom Cayley-Klein) geometry points and lines
with their perpendicularity operations.
"""

from .ck_plane import CayleyKleinPlane
from .pg_object import MyCKLine, MyCKPoint

# MyCK perpendicularity coefficients
MYCK_POINT_PERP_COEFFS = [-2, 1, -2]
MYCK_LINE_PERP_COEFFS = [-1, 2, -1]


def _perp_point(p: MyCKPoint) -> MyCKLine:
    """Return the polar line of a MyCK point."""
    return MyCKLine(
        [
            MYCK_POINT_PERP_COEFFS[0] * p.coord[0],
            MYCK_POINT_PERP_COEFFS[1] * p.coord[1],
            MYCK_POINT_PERP_COEFFS[2] * p.coord[2],
        ]
    )


def _perp_line(l: MyCKLine) -> MyCKPoint:
    """Return the pole of a MyCK line."""
    return MyCKPoint(
        [
            MYCK_LINE_PERP_COEFFS[0] * l.coord[0],
            MYCK_LINE_PERP_COEFFS[1] * l.coord[1],
            MYCK_LINE_PERP_COEFFS[2] * l.coord[2],
        ]
    )


# Implement CayleyKleinPlanePrimitive for MyCKPoint
MyCKPoint.perp = _perp_point  # type: ignore[attr-defined]

# Implement CayleyKleinPlanePrimitive for MyCKLine
MyCKLine.perp = _perp_line  # type: ignore[attr-defined]

# Implement CayleyKleinPlane for MyCKPoint and MyCKLine
CayleyKleinPlane.register(MyCKPoint)
CayleyKleinPlane.register(MyCKLine)
