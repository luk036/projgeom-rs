use crate::pg_plane::{coincident, involution, tri_dual};
use crate::pg_plane::{ProjPlane, ProjPlanePrim};

/// The `CKPlanePrim` trait is a trait that extends the `ProjPlanePrim` trait. It adds an additional
/// method `perp(&self) -> L` to the trait. This method returns the perpendicular line to the given line
/// `self`. The `L` type parameter represents a line in the projective plane.
pub trait CKPlanePrim<L>: ProjPlanePrim<L> {
    // type Dual: ProjPlanePrim;
    fn perp(&self) -> L;
}

/// The function `is_perpendicular` checks if two lines are perpendicular to each other.
/// 
/// Arguments:
/// 
/// * `m1`: A reference to an object of type L, which represents a line or a plane in a geometric space.
/// * `m2`: m2 is a reference to an object of type L, which is a generic type parameter. The specific
/// type of L is not specified in the function signature.
/// 
/// Returns:
/// 
/// a boolean value.
#[allow(dead_code)]
#[inline]
pub fn is_perpendicular<P, L>(m1: &L, m2: &L) -> bool
where
    P: CKPlanePrim<L>,
    L: CKPlanePrim<P>,
{
    m1.perp().incident(m2)
}

/// The `altitude` function calculates the altitude of a point `p` with respect to a line `m`.
/// 
/// Arguments:
/// 
/// * `p`: A point on a plane.
/// * `m`: The parameter `m` represents a line in a two-dimensional plane.
/// 
/// Returns:
/// 
/// The function `altitude` returns a value of type `L`.
#[allow(dead_code)]
#[inline]
pub fn altitude<P, L>(p: &P, m: &L) -> L
where
    P: CKPlanePrim<L>,
    L: CKPlanePrim<P>,
{
    m.perp().circ(p)
}

/// The `orthocenter` function calculates the orthocenter of a triangle given its three vertices.
/// 
/// Arguments:
/// 
/// * `tri`: The `tri` parameter is an array of three elements of type `P`. Each element represents a
/// point in a triangle.
/// 
/// Returns:
/// 
/// The function `orthocenter` returns a value of type `P`, which is the type parameter specified in the
/// function signature.
#[allow(dead_code)]
#[inline]
pub fn orthocenter<P, L>(tri: &[P; 3]) -> P
where
    P: CKPlanePrim<L>,
    L: CKPlanePrim<P>,
{
    let [a1, a2, a3] = tri;
    assert!(!coincident(a1, a2, a3));
    let t1 = altitude(a1, &a2.circ(a3));
    let t2 = altitude(a2, &a3.circ(a1));
    t1.circ(&t2)
}

/// The function `tri_altitude` calculates the altitudes of a triangle given its three vertices and
/// three lines.
/// 
/// Arguments:
/// 
/// * `tri`: tri is an array of 3 elements of type P, representing the vertices of a triangle.
/// 
/// Returns:
/// 
/// The function `tri_altitude` returns an array of three elements, where each element represents the
/// altitude of a vertex in a triangle.
#[allow(dead_code)]
#[inline]
pub fn tri_altitude<P, L>(tri: &[P; 3]) -> [L; 3]
where
    P: CKPlanePrim<L>,
    L: CKPlanePrim<P>,
{
    let [l1, l2, l3] = tri_dual(tri);
    let [a1, a2, a3] = tri;
    assert!(!coincident(a1, a2, a3));
    let t1 = altitude(a1, &l1);
    let t2 = altitude(a2, &l2);
    let t3 = altitude(a3, &l3);
    [t1, t2, t3]
}

pub trait CKPlane<L, V: Default + Eq>: ProjPlane<L, V> + CKPlanePrim<L> {}

/// The `reflect` function in Rust reflects a point `p` across a mirror plane `mirror`.
/// 
/// Arguments:
/// 
/// * `mirror`: A reference to an object of type L, which represents a mirror or a plane.
/// * `p`: The parameter `p` represents a point in a geometric space.
/// 
/// Returns:
/// 
/// The function `reflect` returns a value of type `P`, which is the same type as the input parameter
/// `p`.
#[allow(dead_code)]
#[inline]
pub fn reflect<P, L, V>(mirror: &L, p: &P) -> P
where
    V: Default + Eq,
    P: CKPlane<L, V>,
    L: CKPlane<P, V>,
{
    involution(&mirror.perp(), mirror, p)
}
