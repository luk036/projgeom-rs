pub trait ProjPlanePrim<L>: Eq {
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

pub trait ProjPlane<L, V: Default + Eq>: ProjPlanePrim<L> {
    fn aux(&self) -> L;
    fn dot(&self, line: &L) -> V; // basic measurement
    fn plucker(ld: V, p: &Self, mu: V, q: &Self) -> Self;

    fn incident(&self, line: &L) -> bool {
        self.dot(line) == V::default()
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
pub fn harm_conj<P, L, V>(a: &P, b: &P, c: &P) -> P
where
    V: Default + Eq,
    P: ProjPlane<L, V>,
    L: ProjPlane<P, V>,
{
    assert!(coincident(a, b, c));
    let ab = a.circ(b);
    let lc = ab.aux().circ(c);
    P::plucker(lc.dot(a), a, lc.dot(b), b)
}

#[allow(dead_code)]
pub fn involution<P, L, V>(origin: &P, mirror: &L, p: &P) -> P
where
    V: Default + Eq,
    P: ProjPlane<L, V>,
    L: ProjPlane<P, V>,
{
    let po = p.circ(origin);
    let b = po.circ(mirror);
    harm_conj(origin, &b, p)
}
