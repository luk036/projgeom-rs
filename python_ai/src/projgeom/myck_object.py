"""MyCK Geometry Objects module.

This module defines MyCK (custom Cayley-Klein) geometry points and lines
with their perpendicularity operations.
"""

from .ck_plane import CayleyKleinPlane
from .pg_object import MyCKLine, MyCKPoint

# MyCK perpendicularity coefficients
MYCK_POINT_PERP_COEFFS = [-2, 1, -2]
MYCK_LINE_PERP_COEFFS = [-1, 2, -1]


def _perp_point(point: MyCKPoint) -> MyCKLine:
    """Return the polar line of a MyCK point."""
    return MyCKLine(
        [
            MYCK_POINT_PERP_COEFFS[0] * point.coord[0],
            MYCK_POINT_PERP_COEFFS[1] * point.coord[1],
            MYCK_POINT_PERP_COEFFS[2] * point.coord[2],
        ]
    )


def _perp_line(line: MyCKLine) -> MyCKPoint:
    """Return the pole of a MyCK line."""
    return MyCKPoint(
        [
            MYCK_LINE_PERP_COEFFS[0] * line.coord[0],
            MYCK_LINE_PERP_COEFFS[1] * line.coord[1],
            MYCK_LINE_PERP_COEFFS[2] * line.coord[2],
        ]
    )


# Implement CayleyKleinPlanePrimitive for MyCKPoint
MyCKPoint.perp = _perp_point  # type: ignore[attr-defined]

# Implement CayleyKleinPlanePrimitive for MyCKLine
MyCKLine.perp = _perp_line  # type: ignore[attr-defined]

# Implement CayleyKleinPlane for MyCKPoint and MyCKLine
CayleyKleinPlane.register(MyCKPoint)
CayleyKleinPlane.register(MyCKLine)
