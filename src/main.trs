// use std::cmp::{Eq, PartialEq};
mod pg_plane;
use crate::pg_plane::{coincident, ProjPlanePrim};

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

// impl PartialEq for PArch {
//     fn eq(&self, _rhs: &Self) -> bool {
//         false
//     }
// }
// impl Eq for PArch {}

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

fn main() {
    let p = PArch::new();
    let q = PArch::new();
    let r = PArch::new();
    let l = &LArch::new();
    println!("{}", p == q);
    println!("{}", p.incident(l));
    println!("{}", coincident(&p, &q, &r));
}
