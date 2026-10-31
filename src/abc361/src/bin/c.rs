#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// リポジトリのURL: https://github.com/shon0526/atcoder-environment

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    if k >= n - 1 {
        println!("{}", 0);
        return;
    }

    a.sort();
    let ans = a
        .windows(n - k)
        .map(|vec| vec[n - k - 1] - vec[0])
        .min()
        .unwrap();
    println!("{}", ans);
}
