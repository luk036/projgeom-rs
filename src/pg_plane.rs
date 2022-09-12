pub trait ProjPlanePrim<L>: Eq {
    fn circ(&self, rhs: &Self) -> L; // join or meet
    fn incident(&self, line: &L) -> bool; // incidence
}

#[allow(dead_code)]
pub fn check_axiom<P, L>(p: &P, q: &P, l: &L)
where
    P: ProjPlanePrim<L> + std::fmt::Debug,
    L: ProjPlanePrim<P> + std::fmt::Debug,
{
    assert_eq!(p, p);
    assert_eq!(p == q,  q == p);
    assert_eq!(p.incident(l), l.incident(p));
    assert_eq!(p.circ(q), q.circ(p));
    let m = p.circ(q);
    assert!(m.incident(p) && m.incident(q));
}

#[inline]
pub fn coincident<P, L>(p: &P, q: &P, r: &P) -> bool
where
    P: ProjPlanePrim<L>,
    L: ProjPlanePrim<P>,
{
    p.circ(q).incident(r)
}

/**
 * Check Pappus Theorem
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
 */
#[inline]
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
 * Return whether two triangles are perspective
 */
#[inline]
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
    fn aux(&self) -> L; // line not incident with P
    fn dot(&self, line: &L) -> V; // for basic measurement
    fn plucker(&self, ld: V, q: &Self, mu: V) -> Self;
}

#[allow(dead_code)]
pub fn check_axiom2<P, L, V>(p: &P, q: &P, l: &L, a: V, b: V)
where
    V: Default + Eq,
    P: ProjPlane<L, V>,
    L: ProjPlane<P, V>,
{
    assert!(p.dot(l) == l.dot(p));
    assert!(!p.aux().incident(p));
    let m = p.circ(q);
    assert!(m.incident(&P::plucker(p, a, q, b)));
}

/*
impl<P, L> ProjPlanePrim<L> for P
where
    P: ProjPlaneGeneric<L>,
{
    fn circ(&self, rhs: &Self) -> L { L }

    fn incident(&self, line: &L) -> bool {
        self.dot(line) == 0_i128
    }
}
*/

/**
 * @brief harmonic conjugate
 *
 */
#[allow(dead_code)]
#[inline]
pub fn harm_conj<P, L, V>(a: &P, b: &P, c: &P) -> P
where
    V: Default + Eq,
    P: ProjPlane<L, V>,
    L: ProjPlane<P, V>,
{
    assert!(coincident(a, b, c));
    let ab = a.circ(b);
    let lc = ab.aux().circ(c);
    P::plucker(a, lc.dot(b), b, lc.dot(a))
}

#[allow(dead_code)]
#[inline]
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

#[cfg(test)]
mod tests {
    use crate::pg_plane::ProjPlanePrim;
    use crate::pg_plane::{check_axiom, coincident};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct PArch {}

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct LArch {}

    impl PArch {
        #[inline]
        fn new() -> Self {
            Self {}
        }
    }

    impl LArch {
        #[inline]
        fn new() -> Self {
            Self {}
        }
    }

    impl ProjPlanePrim<LArch> for PArch {
        #[inline]
        fn incident(&self, _rhs: &LArch) -> bool {
            true
        }
        #[inline]
        fn circ(&self, _rhs: &Self) -> LArch {
            LArch::new()
        }
    }

    // impl PartialEq for LArch {
    //     fn eq(&self, _rhs: &Self) -> bool {
    //         false
    //     }
    // }
    // impl Eq for LArch {}

    impl ProjPlanePrim<PArch> for LArch {
        #[inline]
        fn incident(&self, _rhs: &PArch) -> bool {
            true
        }
        #[inline]
        fn circ(&self, _rhs: &Self) -> PArch {
            PArch::new()
        }
    }

    #[test]
    fn it_works() {
        let p = PArch::new();
        let q = PArch::new();
        let r = PArch::new();
        let l = LArch::new();
        println!("{}", p == q);
        println!("{}", p.incident(&l));
        println!("{}", coincident(&p, &q, &r));
        check_axiom(&p, &q, &l);
    }
}
