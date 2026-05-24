use proconio::input;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      a: [usize; n],
    }
}

pub mod btree_multiset {
    use std::{
        borrow::Borrow,
        collections::{
            btree_map::{self},
            BTreeMap,
        },
        ops::{Bound, RangeBounds},
    };
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MultiSet<T> {
        size: usize,
        map: BTreeMap<T, usize>,
    }

    impl<T: Ord> Default for MultiSet<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: Ord> From<Vec<T>> for MultiSet<T> {
        fn from(vec: Vec<T>) -> Self {
            let size = vec.len();
            let mut btree_map = BTreeMap::new();

            for key in vec {
                *btree_map.entry(key).or_insert(0) += 1;
            }

            Self {
                size: size,
                map: btree_map,
            }
        }
    }
    impl<T: Ord> MultiSet<T> {
        pub fn new() -> Self {
            Self {
                size: 0,
                map: BTreeMap::new(),
            }
        }
    }
}
