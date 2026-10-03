#![allow(unused_imports, dead_code)]
use ac_library::{fenwicktree, segtree};
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter;

fn main() {
    input! {
        n: usize,
        s: Usize1,
        l: usize,
        a: [usize; n-1],
    }

    let mut prefix = iter::once(0)
        .chain(
            a.iter()
                .scan(0, |sum, x| {
                    *sum += x;
                    Some(*sum)
                })
                .collect_vec(),
        )
        .collect_vec();
    //   dbg!(&prefix);

    let ans = (0..n)
        .tuple_combinations()
        .filter(|&(left, right)| {
            if !(left..=right).contains(&s) {
                return false;
            }

            let x = prefix[s] - prefix[left];
            let y = prefix[right] - prefix[s];

            let dist1 = 2 * x + y;
            let dist2 = 2 * y + x;

            dist1.min(dist2) <= l
        })
        .map(|(left, right)| right - left + 1)
        .max()
        .unwrap_or(1);

    println!("{ans}");
}
