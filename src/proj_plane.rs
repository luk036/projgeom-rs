pub trait ProjPlanePrim<L>: Eq {
    // type Dual: ProjPlanePrim;
    fn circ(&self, rhs: &Self) -> L;
    fn incident(&self, line: &L) -> bool;
}

#[allow(dead_code)]
pub fn check_axiom<P, L>(p: &P, q: &P, l: &L)
where
    P: ProjPlanePrim<L>,
    L: ProjPlanePrim<P>,
{
    assert!((p == q) == (q == p));
    assert!(p.incident(l) == l.incident(p));
    assert!(p.circ(q) == q.circ(p));
    let m = p.circ(q);
    assert!(m.incident(p) && m.incident(q));
}

pub fn coincident<P, L>(p: &P, q: &P, r: &P) -> bool
where
    P: ProjPlanePrim<L>,
    L: ProjPlanePrim<P>,
{
    p.circ(q).incident(r)
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
    let g = (a.circ(e)).circ(&b.circ(d));
    let h = (a.circ(f)).circ(&c.circ(d));
    let i = (b.circ(f)).circ(&c.circ(e));
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
    L: ProjPlanePrim<P>,
{
    let [a1, a2, a3] = tri;
    assert!(!coincident(a1, a2, a3));
    [a2.circ(a3), a1.circ(a3), a1.circ(a2)]
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
    let o = a.circ(d).circ(&b.circ(e));
    c.circ(f).incident(&o)
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
    let trid1 = &tri_dual(tri1);
    let trid2 = &tri_dual(tri2);
    let b1 = persp(tri1, tri2);
    let b2 = persp(trid1, trid2);
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
    let ab = &a.circ(b);
    let p = &ab.aux1();
    let r = &p.aux2(c);
    let s = &(a.circ(r)).circ(&b.circ(p));
    let q = &(b.circ(r)).circ(&a.circ(p));
    (q.circ(s)).circ(ab)
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
    let po = p.circ(origin);
    let b = po.circ(mirror);
    harm_conj(origin, &b, p)
}

pub trait ProjPlaneGeneric<L>: Eq + ProjPlanePrim2<L> {
    type V: Default + Eq; // measurement value

    fn dot(&self, line: &L) -> Self::V; // basic measurement
    fn plucker(ld: Self::V, p: &Self, mu: Self::V, p: &Self) -> Self;

    fn incident(&self, line: &L) -> bool {
        self.dot(line) == Self::V::default()
    }
}

/*
impl<P, L> ProjPlanePrim<L> for P
where
    P: ProjPlaneGeneric<L>,
{
    fn circ(&self, rhs: &Self) -> L { L }

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
    let ab = a.circ(b);
    let lc = &ab.aux1().circ(c);
    P::plucker(a.dot(lc), a, b.dot(lc), b)
}
