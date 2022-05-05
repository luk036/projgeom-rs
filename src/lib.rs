pub mod ck_plane;
// pub mod hyperbolic;
// pub mod elliptic;
pub mod pg_object;
pub mod pg_plane;
pub mod hyp_object;
pub mod ell_object;

pub use crate::ck_plane::*;
pub use crate::pg_plane::*;

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
        let l = &LArch::new();
        println!("{}", p == q);
        println!("{}", p.incident(l));
        println!("{}", coincident(&p, &q, &r));
        check_axiom(&p, &q, &l);
    }
}
