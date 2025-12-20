"""Hyperbolic Geometry Objects module.

This module defines hyperbolic geometry points and lines with their
perpendicularity operations.
"""

from .ck_plane import CayleyKleinPlane
from .pg_object import HyperbolicLine, HyperbolicPoint

# Hyperbolic perpendicularity coefficients
HYP_PERP_COEFFS = [1, 1, -1]


def _perp_point(p: HyperbolicPoint) -> HyperbolicLine:
    """Return the polar line of a hyperbolic point."""
    return HyperbolicLine(
        [
            HYP_PERP_COEFFS[0] * p.coord[0],
            HYP_PERP_COEFFS[1] * p.coord[1],
            HYP_PERP_COEFFS[2] * p.coord[2],
        ]
    )


def _perp_line(l: HyperbolicLine) -> HyperbolicPoint:
    """Return the pole of a hyperbolic line."""
    return HyperbolicPoint(
        [
            HYP_PERP_COEFFS[0] * l.coord[0],
            HYP_PERP_COEFFS[1] * l.coord[1],
            HYP_PERP_COEFFS[2] * l.coord[2],
        ]
    )


# Implement CayleyKleinPlanePrimitive for HyperbolicPoint
HyperbolicPoint.perp = _perp_point  # type: ignore[attr-defined]

# Implement CayleyKleinPlanePrimitive for HyperbolicLine
HyperbolicLine.perp = _perp_line  # type: ignore[attr-defined]

# Implement CayleyKleinPlane for HyperbolicPoint and HyperbolicLine
CayleyKleinPlane.register(HyperbolicPoint)
CayleyKleinPlane.register(HyperbolicLine)
