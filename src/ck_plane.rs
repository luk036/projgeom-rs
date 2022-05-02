use crate::involution;
use crate::proj_plane::tri_dual;
use crate::proj_plane::ProjPlanePrim;
use crate::proj_plane::ProjPlanePrim2;

pub trait CKPlanePrim<L>: ProjPlanePrim<L> {
    // type Dual: ProjPlanePrim;
    fn perp(&self) -> L;
}

#[allow(dead_code)]
pub fn is_perpendicular<P, L>(m1: &L, m2: &L) -> bool
where
    P: CKPlanePrim<L>,
    L: CKPlanePrim<P>,
{
    m1.perp().incident(m2)
}

#[allow(dead_code)]
pub fn altitude<P, L>(p: &P, m: &L) -> L
where
    P: CKPlanePrim<L>,
    L: CKPlanePrim<P>,
{
    m.perp().circ(p)
}

#[allow(dead_code)]
pub fn orthcenter<P, L>(tri: &[P; 3]) -> P
where
    P: CKPlanePrim<L>,
    L: CKPlanePrim<P>,
{
    let [a1, a2, a3] = tri;
    let t1 = &altitude(a1, &a2.circ(a3));
    let t2 = &altitude(a2, &a3.circ(a1));
    t1.circ(t2)
}

#[allow(dead_code)]
pub fn tri_altitude<P, L>(tri: &[P; 3]) -> [L; 3]
where
    P: CKPlanePrim<L>,
    L: CKPlanePrim<P>,
{
    let [l1, l2, l3] = tri_dual(tri);
    let [a1, a2, a3] = tri;
    let t1 = altitude(a1, &l1);
    let t2 = altitude(a2, &l2);
    let t3 = altitude(a3, &l3);
    [t1, t2, t3]
}

pub trait CKPlanePrim2<L>: ProjPlanePrim2<L> {
    // type Dual: ProjPlanePrim;
    fn perp(&self) -> L;
}

#[allow(dead_code)]
pub fn reflect<P, L>(mirror: &L, p: &P) -> P
where
    P: CKPlanePrim2<L>,
    L: CKPlanePrim2<P>,
{
    involution(&mirror.perp(), mirror, p)
}
