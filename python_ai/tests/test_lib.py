"""Test suite for the projgeom Python library."""

from math import gcd
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from projgeom import (
        PgPoint,
        PgLine,
        EuclidPoint,
        EuclidLine,
        EllipticPoint,
        EllipticLine,
        HyperbolicPoint,
        HyperbolicLine,
        MyCKPoint,
        MyCKLine,
        PerspPoint,
        PerspLine,
    )

from projgeom import (
    EllipticLine,
    EllipticPoint,
    EuclidLine,
    EuclidPoint,
    HyperbolicLine,
    HyperbolicPoint,
    MyCKLine,
    MyCKPoint,
    PerspPoint,
    PgLine,
    PgPoint,
    check_desargue,
    check_pappus,
    coincident,
    cross2,
    cross_product,
    dot1,
    dot_product,
    harm_conj,
    is_perpendicular,
    orthocenter,
    persp,
    plucker_operation,
    reflect,
    tri_altitude,
    tri_dual,
)


def test_gcd() -> None:
    """Test basic gcd function."""
    result = gcd(4, -6)
    assert result == 2


def test_cross_product() -> None:
    """Test cross product function."""
    result = cross_product([1, 2, 3], [3, 4, 5])
    assert result == [-2, 4, -2]


def test_dot_product() -> None:
    """Test dot product function."""
    result = dot_product([1, 2, 3], [3, 4, 5])
    assert result == 26


def test_dot1() -> None:
    """Test 2D dot product function."""
    result = dot1([1, 2], [3, 4])
    assert result == 11


def test_cross2() -> None:
    """Test 2D cross product function."""
    result = cross2([1, 2], [3, 4])
    assert result == -2


def test_plucker_operation() -> None:
    """Test Plucker operation."""
    result = plucker_operation(1, [1, 2, 3], -1, [3, 4, 5])
    assert result == [-2, -2, -2]


def test_pg_point_creation() -> None:
    """Test PgPoint creation."""
    p = PgPoint([1, 2, 3])
    assert p.coord == [1, 2, 3]


def test_pg_line_creation() -> None:
    """Test PgLine creation."""
    l = PgLine([1, 2, 3])
    assert l.coord == [1, 2, 3]


def test_pg_point_equality() -> None:
    """Test PgPoint equality."""
    p1 = PgPoint([1, 2, 3])
    p2 = PgPoint([2, 4, 6])  # Homogeneous equivalent
    p3 = PgPoint([1, 2, 4])  # Different point

    assert p1 == p2
    assert p1 != p3


def test_pg_line_equality() -> None:
    """Test PgLine equality."""
    l1 = PgLine([1, 2, 3])
    l2 = PgLine([2, 4, 6])  # Homogeneous equivalent
    l3 = PgLine([1, 2, 4])  # Different line

    assert l1 == l2
    assert l1 != l3


def test_pg_point_incident() -> None:
    """Test point-line incidence."""
    p = PgPoint([1, 1, 1])  # Point (1,1) in Euclidean plane
    l = PgLine([1, 1, 0])  # Line x + y = 0 in Euclidean plane

    # Point (1,1) is not on line x+y=0
    assert not p.incident(l)

    p_on_l = PgPoint([1, -1, 1])  # Point (1,-1) on line x+y=0
    assert p_on_l.incident(l)

    l_through_p = PgLine([1, -1, 0])  # Line x-y=0, passes through (1,1)
    assert p.incident(l_through_p)


def test_pg_point_meet() -> None:
    """Test point meet operation."""
    p1 = PgPoint([1, 0, 0])  # Point at infinity on x-axis
    p2 = PgPoint([0, 1, 0])  # Point at infinity on y-axis
    line_at_infinity = PgLine([0, 0, 1])  # Line at infinity

    # Meet of two points is the line connecting them
    assert p1.meet(p2) == line_at_infinity

    p3 = PgPoint([1, 2, 1])  # Euclidean point (1,2)
    p4 = PgPoint([3, 4, 1])  # Euclidean point (3,4)
    line_p3_p4 = p3.meet(p4)
    # The line connecting (1,2) and (3,4) should be x - y + 1 = 0, or [1, -1, 1]
    # cross_product([1,2,1], [3,4,1]) = [2*1 - 1*4, 1*3 - 1*1, 1*4 - 2*3] = [-2, 2, -2]
    # This is homogeneous to [1, -1, 1]
    assert line_p3_p4 == PgLine([1, -1, 1])


def test_pg_line_meet() -> None:
    """Test line meet operation."""
    l1 = PgLine([1, 0, 0])  # Line x=0 (y-axis)
    l2 = PgLine([0, 1, 0])  # Line y=0 (x-axis)
    origin = PgPoint([0, 0, 1])  # Origin (0,0)

    # Meet of two lines is their intersection point
    assert l1.meet(l2) == origin

    l3 = PgLine([1, -1, 0])  # Line x - y = 0
    l4 = PgLine([1, 1, -2])  # Line x + y - 2 = 0
    intersection_point = l3.meet(l4)
    # Intersection of x-y=0 and x+y-2=0 is (1,1)
    # cross_product([1,-1,0], [1,1,-2]) = [(-1)*(-2) - 0*1, 0*1 - 1*(-2), 1*1 - (-1)*1] = [2, 2, 2]
    # This is homogeneous to [1, 1, 1]
    assert intersection_point == PgPoint([1, 1, 1])


def test_pg_point_parametrize() -> None:
    """Test point parametrization."""
    p1 = PgPoint([1, 0, 0])
    p2 = PgPoint([0, 1, 0])

    # Parametrize with lambda=1, mu=1 should give [1,1,0]
    p_mid = p1.parametrize(1, p2, 1)
    assert p_mid == PgPoint([1, 1, 0])

    # Parametrize with lambda=2, mu=1 should give [2,1,0]
    p_weighted = p1.parametrize(2, p2, 1)
    assert p_weighted == PgPoint([2, 1, 0])

    # Parametrize with lambda=0, mu=1 should give p2
    p_only_p2 = p1.parametrize(0, p2, 1)
    assert p_only_p2 == p2


def check_pg_plane(pt_p: "PgPoint", pt_q: "PgPoint") -> None:
    """Helper function to check projective plane properties."""
    ln_l = pt_p.meet(pt_q)
    assert ln_l == pt_q.meet(pt_p)
    assert ln_l.incident(pt_p)
    assert ln_l.incident(pt_q)
    pq = pt_p.parametrize(2, pt_q, 3)
    assert coincident(pt_p, pt_q, pq)

    h = harm_conj(pt_p, pt_q, pq)
    assert harm_conj(pt_p, pt_q, h) == pq


def test_pg_point_plane() -> None:
    """Test PgPoint projective plane properties."""
    pt_p = PgPoint([1, 3, 2])
    pt_q = PgPoint([-2, 1, -1])
    check_pg_plane(pt_p, pt_q)


def test_pg_line_plane() -> None:
    """Test PgLine projective plane properties."""
    pt_p = PgLine([1, 3, 2])
    pt_q = PgLine([-2, 1, -1])
    check_pg_plane(pt_p, pt_q)


def check_ck_plane(a_1: "PgPoint", a_2: "PgPoint", a_3: "PgPoint") -> None:
    """Helper function to check Cayley-Klein plane properties."""
    triangle: list["PgPoint"] = [a_1, a_2, a_3]
    trilateral: list["PgLine"] = tri_dual(triangle)
    l_1: "PgLine" = trilateral[0]
    assert l_1.incident(triangle[1])

    t_1: "PgLine"
    t_2: "PgLine" 
    t_3: "PgLine"
    t_1, t_2, t_3 = tri_altitude(triangle)
    assert is_perpendicular(t_1, l_1)
    pt_o = orthocenter(triangle)
    assert pt_o == t_2.meet(t_3)


def test_elliptic_point() -> None:
    """Test EllipticPoint properties."""
    a_1 = EllipticPoint([13, 23, 32])
    a_2 = EllipticPoint([44, -34, 2])
    a_3 = EllipticPoint([-2, 12, 23])
    check_ck_plane(a_1, a_2, a_3)


def test_elliptic_line() -> None:
    """Test EllipticLine properties."""
    a_1 = EllipticLine([13, 23, 32])
    a_2 = EllipticLine([44, -34, 2])
    a_3 = EllipticLine([-2, 12, 23])
    check_ck_plane(a_1, a_2, a_3)


def test_hyperbolic_point() -> None:
    """Test HyperbolicPoint properties."""
    a_1 = HyperbolicPoint([13, 23, 32])
    a_2 = HyperbolicPoint([44, -34, 2])
    a_3 = HyperbolicPoint([-2, 12, 23])
    check_ck_plane(a_1, a_2, a_3)


def test_hyperbolic_line() -> None:
    """Test HyperbolicLine properties."""
    a_1 = HyperbolicLine([13, 23, 32])
    a_2 = HyperbolicLine([44, -34, 2])
    a_3 = HyperbolicLine([-2, 12, 23])
    check_ck_plane(a_1, a_2, a_3)


def test_myck_point() -> None:
    """Test MyCKPoint properties."""
    a_1 = MyCKPoint([13, 23, 32])
    a_2 = MyCKPoint([44, -34, 2])
    a_3 = MyCKPoint([-2, 12, 23])
    check_ck_plane(a_1, a_2, a_3)


def test_myck_line() -> None:
    """Test MyCKLine properties."""
    a_1 = MyCKLine([13, 23, 32])
    a_2 = MyCKLine([44, -34, 2])
    a_3 = MyCKLine([-2, 12, 23])
    check_ck_plane(a_1, a_2, a_3)


def test_persp_point() -> None:
    """Test PerspPoint properties."""
    a_1 = PerspPoint([13, 23, 32])
    a_2 = PerspPoint([44, -34, 2])
    a_3 = PerspPoint([-2, 12, 23])
    check_ck_plane(a_1, a_2, a_3)


def test_euclid_point() -> None:
    """Test EuclidPoint properties."""
    a_1 = EuclidPoint([13, 23, 32])
    a_2 = EuclidPoint([44, -34, 2])
    a_3 = EuclidPoint([-2, 12, 23])
    check_ck_plane(a_1, a_2, a_3)


def test_euclid_line_parallel() -> None:
    """Test Euclidean line parallelism."""
    l1 = EuclidLine([1, 0, -1])  # x = 1
    l2 = EuclidLine([2, 0, -5])  # x = 2.5 (parallel to l1)
    assert l1.is_parallel(l2)

    l3 = EuclidLine([0, 1, -1])  # y = 1 (not parallel to l1)
    assert not l1.is_parallel(l3)


def test_euclid_line_perpendicular() -> None:
    """Test Euclidean line perpendicularity."""
    l1 = EuclidLine([1, 0, -1])  # x = 1
    l2 = EuclidLine([0, 1, -1])  # y = 1
    assert l1.is_perpendicular(l2)

    l3 = EuclidLine([1, 1, -1])  # x + y = 1
    assert not l1.is_perpendicular(l3)


def test_euclid_point_midpoint() -> None:
    """Test Euclidean point midpoint."""
    p1 = EuclidPoint([0, 0, 1])
    p2 = EuclidPoint([2, 4, 1])
    mid = p1.midpoint(p2)
    # Midpoint of (0,0) and (2,4) is (1,2)
    assert mid == EuclidPoint([1, 2, 1])


def test_pappus() -> None:
    """Test Pappus' theorem."""
    a = PgPoint([0, 0, 1])
    b = PgPoint([1, 0, 1])
    c = PgPoint([2, 0, 1])

    d = PgPoint([0, 1, 1])
    e = PgPoint([1, 1, 1])
    f = PgPoint([2, 1, 1])

    coline_1 = [a, b, c]
    coline_2 = [d, e, f]

    # These points are collinear, so Pappus' theorem should hold
    assert check_pappus(coline_1, coline_2)


def test_desargue() -> None:
    """Test Desargue's theorem."""
    t1_p1 = PgPoint([0, 0, 1])
    t1_p2 = PgPoint([1, 0, 1])
    t1_p3 = PgPoint([0, 1, 1])
    tri1 = [t1_p1, t1_p2, t1_p3]

    t2_p1 = PgPoint([0, 0, 2])
    t2_p2 = PgPoint([2, 0, 2])
    t2_p3 = PgPoint([0, 2, 2])
    tri2 = [t2_p1, t2_p2, t2_p3]

    # These two triangles are perspective from the origin (0,0,1)
    assert persp(tri1, tri2)
    assert check_desargue(tri1, tri2)


def test_reflect() -> None:
    """Test point reflection."""
    p = EuclidPoint([1, 2, 1])  # Point (1,2)
    mirror = EuclidLine([1, 0, 0])  # Line x = 0 (y-axis)
    reflected_p = reflect(mirror, p)
    # Reflecting (1,2) across x=0 should give (-1,2)
    assert reflected_p == EuclidPoint([-1, 2, 1])

    p2 = EuclidPoint([3, 3, 1])  # Point (3,3)
    mirror2 = EuclidLine([0, 1, -2])  # Line y = 2
    reflected_p2 = reflect(mirror2, p2)
    # Reflecting (3,3) across y=2 should give (3,1)
    assert reflected_p2 == EuclidPoint([3, 1, 1])
