pub mod ck_plane;
// pub mod hyperbolic;
// pub mod elliptic;
pub mod ell_object;
pub mod hyp_object;
pub mod pg_object;
pub mod pg_plane;

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
        let l = LArch::new();
        println!("{}", p == q);
        println!("{}", p.incident(&l));
        println!("{}", coincident(&p, &q, &r));
        check_axiom(&p, &q, &l);
    }
 
    use std::str::FromStr;
    use fraction::{Fraction, Sign};

    #[test]
    fn test_fraction() {
        // There are several ways to construct a fraction, depending on your use case

        let f = Fraction::new_generic(Sign::Minus, 3i32, 4u32).unwrap();  // with numerator/denominator of different integer types
        // let inf = Fraction::new_generic(Sign::Plus, 1u32, 0u32).unwrap();  // with numerator/denominator of different integer types
        assert_eq!(f, Fraction::new_generic(Sign::Plus, -3i32, 4u8).unwrap());  // with numerator/denominator of different integer types
        assert_eq!(f, Fraction::from_str("-0.75").unwrap());  // parse a string
        assert_eq!(f, Fraction::from_str("-3/4").unwrap());  // parse a string

        // assert_eq!(inf, Fraction::from_str("1/0").unwrap());  // parse a string
    }
}
