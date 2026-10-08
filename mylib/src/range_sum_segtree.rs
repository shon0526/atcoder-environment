use cargo_snippet::snippet;

#[snippet(name = "range-sum-segtree")]

pub mod range_sum_segtree {
    use std::cmp::{max, Ord};
    use std::fmt;
    use std::ops::{Bound, RangeBounds};

    pub trait SumIdentity: Copy + Ord + fmt::Display {
        fn e() -> Self;
    }

    macro_rules! impl_max_identity {
        ($($t: ty), *) => {
           $(
               impl SumIdentity for $t {
                   fn e() -> Self {
                      0
                   }
               }
               )*
        };
    }

    impl_max_identity!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

    #[derive(Debug)]
    pub struct SegTree<T> {
        pub size: usize,
        pub node: Vec<T>,
    }

    impl<T: SumIdentity> SegTree<T> {
        fn new(&self, n: usize) -> Self {
            Self {
                size: n,
                node: vec![T::e(); 2 * n],
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::range_sum_segtree::*;
}
