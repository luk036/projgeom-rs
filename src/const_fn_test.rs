// 1.68.0-nightly (2022-12-11 bdb07a8ec8e77aa10fb8)

#![feature(const_trait_impl)]  // const trait impls are (still) experimental/unstable

#[const_trait]  // This is needed now
trait Example {
    fn usage(&self) -> Self;
}

impl const Example for u8 {
    fn usage(&self) -> Self {
        42
    }
}

const fn demo<T: ~const Example>(v: &T) -> T {
    v.usage()
}

const VALUE: u8 = demo(&1);

// fn main() {
//     dbg!(VALUE);
// }