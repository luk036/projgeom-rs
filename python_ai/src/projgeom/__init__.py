"""Projective Geometry in Python.

This package provides implementations of projective geometry concepts
including points, lines, and various geometric transformations.
"""

from .ck_plane import (
    CayleyKleinPlane,
    CayleyKleinPlanePrimitive,
    altitude,
    is_perpendicular,
    orthocenter,
    reflect,
    tri_altitude,
)
from .ell_object import EllipticLine, EllipticPoint
from .euclid_object import EuclidLine, EuclidPoint
from .hyp_object import HyperbolicLine, HyperbolicPoint
from .myck_object import MyCKLine, MyCKPoint
from .persp_object import PerspLine, PerspPoint
from .pg_object import (
    PgLine,
    PgPoint,
    cross2,
    cross_product,
    dot1,
    dot_product,
    plucker_operation,
)
from .pg_plane import (
    ProjectivePlane,
    ProjectivePlanePrimitive,
    check_axiom,
    check_axiom2,
    check_desargue,
    check_pappus,
    coincident,
    harm_conj,
    involution,
    persp,
    tri_dual,
)

__all__ = [
    # Core traits
    "ProjectivePlanePrimitive",
    "ProjectivePlane",
    "CayleyKleinPlanePrimitive",
    "CayleyKleinPlane",
    # Basic geometric objects
    "PgPoint",
    "PgLine",
    "EuclidPoint",
    "EuclidLine",
    "HyperbolicPoint",
    "HyperbolicLine",
    "EllipticPoint",
    "EllipticLine",
    "MyCKPoint",
    "MyCKLine",
    "PerspPoint",
    "PerspLine",
    # Utility functions
    "dot_product",
    "dot1",
    "cross2",
    "cross_product",
    "plucker_operation",
    "check_axiom",
    "check_axiom2",
    "coincident",
    "tri_dual",
    "persp",
    "check_desargue",
    "check_pappus",
    "harm_conj",
    "involution",
    "is_perpendicular",
    "altitude",
    "orthocenter",
    "tri_altitude",
    "reflect",
]
