use cargo_snippet::snippet;

#[snippet(name = "range-max-segtree")]
pub mod range_max_segtree {

    use std::cmp::{max, Ord};
    use std::fmt;
    use std::ops::{Bound, RangeBounds};

    pub trait MaxIdentity: Copy + Ord + fmt::Display {
        fn min_identity() -> Self;
    }

    macro_rules! impl_max_identity {
        ($($t: ty), *) => {
           $(
               impl MaxIdentity for $t {
                   fn min_identity() -> Self {
                       <$t>::MIN
                   }
               }
               )*
        };
    }

    impl_max_identity!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

    #[derive(Debug, Clone)]
    pub struct SegTree<T> {
        pub size: usize,
        pub node: Vec<T>,
    }
    impl<T: MaxIdentity> SegTree<T> {
        // 単位元で初期化されたセグメント木を構築
        // n = 0の時に壊れないようにする
        pub fn new(n: usize) -> SegTree<T> {
            SegTree {
                size: n,
                node: vec![T::min_identity(); 2 * n],
            }
        }

        // idx番目の値をvに更新する
        pub fn update(&mut self, idx: usize, v: T) {
            assert!(idx < self.size);
            let mut i = self.size + idx;
            self.node[i] = v;

            while i > 1 {
                i >>= 1;
                self.node[i] = max(self.node[i << 1], self.node[i << 1 | 1]);
            }
        }

        // 指定された区間の最大値を取る
        pub fn get_range<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            let (mut l, mut r) = self.to_half_open(range);
            l += self.size;
            r += self.size;

            let mut res = T::min_identity();

            while l < r {
                if l % 2 == 1 {
                    res = res.max(self.node[l]);
                    l += 1;
                }

                if r % 2 == 1 {
                    r -= 1;
                    res = res.max(self.node[r]);
                }

                l >>= 1;
                r >>= 1;
            }

            res
        }

        // 指定したRangeBoundsを半開区間に変換する
        // 範囲外の指定はパニックさせる
        fn to_half_open<R>(&self, range: R) -> (usize, usize)
        where
            R: RangeBounds<usize>,
        {
            let l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l.checked_add(1).expect("range start is too large"),
                Bound::Unbounded => 0,
            };

            let r = match range.end_bound() {
                Bound::Included(r) => r.checked_add(1).expect("range end is too large"),
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.size,
            };

            assert!(l <= r && r <= self.size);
            (l, r)
        }

        // 全要素の最大値を取る
        pub fn get_all(&self) -> T {
            self.get_range(..)
        }

        // idx番目の値を取る
        pub fn get(&self, idx: usize) -> T {
            assert!(idx < self.size);
            self.node[idx + self.size]
        }
    }

    impl<T: MaxIdentity> From<Vec<T>> for SegTree<T> {
        fn from(vec: Vec<T>) -> Self {
            let n = vec.len();
            let mut node = vec![T::min_identity(); 2 * n];
            node[n..2 * n].clone_from_slice(&vec);
            let mut ret = SegTree {
                size: n,
                node: node,
            };

            for i in (1..n).rev() {
                ret.node[i] = ret.node[i << 1].max(ret.node[i << 1 | 1]);
            }

            ret
        }
    }

    impl<T: MaxIdentity> FromIterator<T> for SegTree<T> {
        // イテレータからセグメント木を構築する
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            SegTree::from(iter.into_iter().collect::<Vec<T>>())
        }
    }
}

//------------------------
// 次はテストの実装から
//------------------------
#[cfg(test)]
mod test {
    use super::range_max_segtree::*;
    use std::ops::Bound::{Excluded, Included};

    #[test]
    fn test_max_segtree() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let mut segtree = SegTree::from(base.clone());
        let n = base.len();
        for i in 0..n {
            assert_eq!(segtree.get(i), base[i]);
        }

        assert_eq!(segtree.get_range(0..3), 4);
        assert_eq!(segtree.get_range(3..=5), 9);
        assert_eq!(segtree.get_range((Excluded(0), Included(2))), 4);
        assert_eq!(segtree.get_range(..), 9);
        assert_eq!(segtree.get_all(), 9);

        segtree.update(n - 1, 10);
        assert_eq!(segtree.get_range(..), 10);
        assert_eq!(segtree.get_all(), 10);
    }

    #[test]
    fn test_empty_range_and_empty_segtree() {
        let segtree = SegTree::from(vec![5i64, 3, 8, 1, 9, 2]);
        assert_eq!(segtree.get_range((Excluded(4), Excluded(5))), i64::MIN);

        let segtree = SegTree::<i64>::from(vec![]);
        assert_eq!(segtree.get_range(..), i64::MIN);
        assert_eq!(segtree.get_all(), i64::MIN);
    }

    #[test]
    fn test_from_iter() {
        let segtree: SegTree<i64> = (0..10).map(|x| x * x - 5 * x).collect();
        assert_eq!(segtree.get_range(..), 36);
        assert_eq!(segtree.get_range(0..5), 0);
    }

    #[test]
    #[should_panic]
    fn test_out_of_range() {
        let segtree = SegTree::from(vec![1i64, 2]);
        segtree.get_range(0..3);
    }
}
