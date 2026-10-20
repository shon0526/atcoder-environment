#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: i64,
        t: i64,
        mut a: [i64; n],
    }
    a.sort();

    let mut interval_set = interval_set::IntervalSet::new();

    interval_set.insert_range(0, t);

    let mut now = 0;
    let mut ans = 0;

    for i in 0..n {
        interval_set.remove_range(a[i], a[i] + k);
    }

    let ans = interval_set
        .ranges()
        .map(|(l, r)| num_integer::div_ceil(r - l, k))
        .sum::<i64>();

    println!("{}", ans);
    // let mut now = 0;
    // let mut ans = 0;
    // for i in 0..n {
    //     if a[i] >= t {
    //         break;
    //     }
    //     if a[i] < now {
    //         now = a[i] + k;
    //         //         println!("i: {}, now: {}", i, now);
    //         continue;
    //     }
    //     ans += floor(a[i] - now, k);
    //     now = a[i] + k;
    //
    //     //        println!("i: {}, now: {}", i, now);
    //     //        println!("i: {} ans: {}", i, ans);
    // }
    // //   println!("{}", now);
    // //  println!("{}", ans);
    //
    // //   println!("{}", now);
    // if now < t {
    //     ans += floor(t - now, k);
    // }
    // println!("{}", ans);
}

pub mod interval_set {
    use std::collections::BTreeMap;
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct IntervalSet {
        map: BTreeMap<i64, i64>,
        length: i64,
    }
    impl Default for IntervalSet {
        fn default() -> Self {
            Self::new()
        }
    }
    impl IntervalSet {
        pub fn new() -> Self {
            Self {
                map: BTreeMap::new(),
                length: 0,
            }
        }
        pub fn contains(&self, x: i64) -> bool {
            if let Some((&l, &r)) = self.map.range(..=x).last() {
                l <= x && x < r
            } else {
                false
            }
        }
        pub fn covered(&self, l: i64, r: i64) -> bool {
            assert!(l < r);
            if let Some((&p, &q)) = self.map.range(..=l).last() {
                p <= l && r <= q
            } else {
                false
            }
        }
        pub fn same(&self, p: i64, q: i64) -> bool {
            if !self.covered(p, p + 1) || !self.covered(q, q + 1) {
                return false;
            }
            if let Some((&kp, _)) = self.map.range(..=p).last() {
                if let Some((&kq, _)) = self.map.range(..=q).last() {
                    kp == kq
                } else {
                    false
                }
            } else {
                false
            }
        }
        pub fn insert_range(&mut self, l: i64, r: i64) {
            assert!(l <= r);
            if l == r {
                return;
            }
            let mut l_min = l;
            let mut r_max = r;
            let mut rm_key = Vec::new();
            let mut rm_length = 0;
            for (&left, &right) in self
                .map
                .range(..=r)
                .rev()
                .take_while(|(_, right)| l <= **right)
            {
                l_min = l_min.min(left);
                r_max = r_max.max(right);
                rm_key.push(left);
                rm_length += right - left;
            }
            for key in rm_key {
                self.map.remove(&key);
            }
            self.map.insert(l_min, r_max);
            let add_length = r_max - l_min;
            self.length += add_length - rm_length;
        }
        pub fn remove_range(&mut self, l: i64, r: i64) {
            assert!(l <= r);
            if l == r {
                return;
            }
            let mut add_v = Vec::new();
            let mut rem_v = Vec::new();
            let mut ch_length = 0;
            for (&left, &right) in self
                .map
                .range(..r)
                .rev()
                .take_while(|(_, right)| l < **right)
            {
                rem_v.push(left);
                ch_length -= right - left;
                if left < l {
                    add_v.push((left, l));
                    ch_length += l - left;
                }
                if r < right {
                    add_v.push((r, right));
                    ch_length += right - r;
                }
            }
            for key in rem_v {
                self.map.remove(&key);
            }
            for (l_add, r_add) in add_v {
                self.map.insert(l_add, r_add);
            }
            self.length += ch_length;
        }
        pub fn insert(&mut self, x: i64) {
            self.insert_range(x, x + 1);
        }
        pub fn remove(&mut self, x: i64) {
            self.remove_range(x, x + 1)
        }
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> i64 {
            self.length
        }
        pub fn count(&self) -> usize {
            self.map.len()
        }
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
        pub fn ranges(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
            self.map.iter().map(|(&l, &r)| (l, r))
        }
    }
}
