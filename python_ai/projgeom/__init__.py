"""
projgeom - Python bindings for projgeom-rs

This module provides Python bindings for the projgeom-rs library,
which implements projective geometry in Rust.
"""

from .projgeom import (
    PgPoint,
    PgLine,
    EllipticPoint,
    EllipticLine,
    HyperbolicPoint,
    HyperbolicLine,
    EuclidPoint,
    EuclidLine,
    MyCKPoint,
    MyCKLine,
    PerspPoint,
    PerspLine,
    orthocenter,
    tri_altitude,
    tri_dual,
    is_perpendicular,
    reflect,
    harm_conj,
    coincident,
    involution,
)

__version__ = "0.1.3"
__all__ = [
    "PgPoint",
    "PgLine",
    "EllipticPoint",
    "EllipticLine",
    "HyperbolicPoint",
    "HyperbolicLine",
    "EuclidPoint",
    "EuclidLine",
    "MyCKPoint",
    "MyCKLine",
    "PerspPoint",
    "PerspLine",
    "orthocenter",
    "tri_altitude",
    "tri_dual",
    "is_perpendicular",
    "reflect",
    "harm_conj",
    "coincident",
    "involution",
]