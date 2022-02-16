pub trait ProjPlanePrim<L>: Eq {
    // type Dual: ProjPlanePrim;
    fn cross(&self, rhs: &Self) -> L;
    fn incident(&self, line: &L) -> bool;
}

pub fn coincident<P, L>(p: &P, q: &P, r: &P) -> bool
where
    P: ProjPlanePrim<L>,
{
    let l = p.cross(q);
    r.incident(&l)
}

/**
 * @brief Check Pappus Theorem
 *
 * @tparam P
 * @tparam L
 * @param[in] co1
 * @param[in] co2
 */
#[allow(dead_code)]
pub fn check_pappus<P, L>(co1: &[P; 3], co2: &[P; 3]) -> bool
where
    P: ProjPlanePrim<L>,
    L: ProjPlanePrim<P>,
{
    let [a, b, c] = co1;
    let [d, e, f] = co2;
    let g = (a.cross(e)).cross(& b.cross(d));
    let h = (a.cross(f)).cross(& c.cross(d));
    let i = (b.cross(f)).cross(& c.cross(e));
    coincident(&g, &h, &i)
}

/**
 * @brief
 *
 * @param[in] tri
 * @return auto
 */
pub fn tri_dual<P, L>(tri: &[P; 3]) -> [L; 3]
where
    P: ProjPlanePrim<L>,
{
    let [a1, a2, a3] = tri;
    assert!(!coincident(a1, a2, a3));
    [a2.cross(a3), a1.cross(a3), a1.cross(a2)]
}

/**
 * @brief return whether two triangles are perspective
 *
 * @param[in] tri1
 * @param[in] tri2
 * @return true
 * @return false
 */
pub fn persp<P, L>(tri1: &[P; 3], tri2: &[P; 3]) -> bool
where
    P: ProjPlanePrim<L>,
    L: ProjPlanePrim<P>,
{
    let [a, b, c] = tri1;
    let [d, e, f] = tri2;
    let o = (a.cross(d)).cross(& b.cross(e));
    (c.cross(f)).incident(&o)
}

/**
 * @brief
 *
 * @param[in] tri1
 * @param[in] tri2
 */
#[allow(dead_code)]
pub fn check_desargue<P, L>(tri1: &[P; 3], tri2: &[P; 3]) -> bool
where
    P: ProjPlanePrim<L>,
    L: ProjPlanePrim<P>,
{
    let trid1 = tri_dual(tri1);
    let trid2 = tri_dual(tri2);
    let b1 = persp(tri1, tri2);
    let b2 = persp(&trid1, &trid2);
    (b1 && b2) || (!b1 && !b2)
}

pub trait ProjPlanePrim2<L>: Eq + ProjPlanePrim<L> {
    // type Dual: ProjPlanePrim;
    fn aux1(&self) -> L; // line not incident with p
    fn aux2(&self, other: &Self) -> Self; // point r on pq, r != p and r != q
}

/**
 * @brief harmonic conjugate
 *
 */
pub fn harm_conj<P, L>(a: &P, b: &P, c: &P) -> P
where
    P: ProjPlanePrim2<L>,
    L: ProjPlanePrim2<P>,
{
    assert!(coincident(a, b, c));
    let ab = a.cross(b);
    let p = ab.aux1();
    let r = p.aux2(c);
    let s = (a.cross(&r)).cross(& b.cross(&p));
    let q = (b.cross(&r)).cross(& a.cross(&p));
    (q.cross(&s)).cross(&ab)
}

/**
 * @brief harmonic conjugate
 *
 */
#[allow(dead_code)]
pub fn is_harmonic<P, L>(a: &P, b: &P, c: &P, d: &P) -> bool
where
    P: ProjPlanePrim2<L>,
    L: ProjPlanePrim2<P>,
{
    harm_conj(a, b, c) == *d
}

#[allow(dead_code)]
pub fn involution<P, L>(origin: &P, mirror: &L, p: &P) -> P
where
    P: ProjPlanePrim2<L>,
    L: ProjPlanePrim2<P>,
{
    let po = p.cross(origin);
    let b = po.cross(mirror);
    harm_conj(origin, &b, p)
}

pub trait ProjPlaneGeneric<L>: Eq + ProjPlanePrim2<L> {
    type V; // measurement value
    fn dot(&self, line: &L) -> Self::V; // basic measurement
    fn plucker(ld: &Self::V, p: &Self, mu: &Self::V, p: &Self) -> Self;

    // fn incident(&self, line: &L) -> bool {
    //     self.dot(line) == Self::V::new()
    // }
}

/*
impl<P, L> ProjPlanePrim<L> for P
where
    P: ProjPlaneGeneric<L>,
{
    fn cross(&self, rhs: &Self) -> L { L }

    fn incident(&self, line: &L) -> bool {
        self.dot(line) == 0_i64
    }
}
*/

/**
 * @brief harmonic conjugate
 *
 */
#[allow(dead_code)]
pub fn harm_conj_generic<P, L>(a: &P, b: &P, c: &P) -> P
where
    P: ProjPlaneGeneric<L>,
    L: ProjPlaneGeneric<P>,
{
    assert!(coincident(a, b, c));
    let ab = a.cross(b);
    let lc = ab.aux1().cross(c);
    P::plucker(&a.dot(&lc), a, &b.dot(&lc), b)
}
