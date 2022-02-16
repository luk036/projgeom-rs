// use std::cmp::{Eq, PartialEq};
mod lib;
use crate::lib::{coincident, ProjPlanePrim, ProjPlanePrim2};

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
    fn cross(&self, _rhs: &Self) -> LArch {
        LArch::new()
    }
}

impl ProjPlanePrim2<LArch> for PArch {
    #[inline]
    fn aux1(&self) -> LArch {
        LArch::new()
    } // line not incident with p
    #[inline]
    fn aux2(&self, _other: &Self) -> Self {
        Self::new()
    } // point r on p * q, r != p and r != q
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
    fn cross(&self, _rhs: &Self) -> PArch {
        PArch::new()
    }
}

impl ProjPlanePrim2<PArch> for LArch {
    #[inline]
    fn aux1(&self) -> PArch {
        PArch::new()
    } // line not incident with p
    #[inline]
    fn aux2(&self, _other: &Self) -> Self {
        Self::new()
    } // point r on p * q, r != p and r != q
}

fn main() {
    let p = PArch::new();
    let q = PArch::new();
    let r = PArch::new();
    let l = LArch::new();
    println!("{}", p == q);
    println!("{}", p.incident(&l));
    println!("{}", coincident(&p, &q, &r));
}
