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

    let mut ans = 0;

    for i in 0..n {
        if a[i] == k {
            ans += 1;
        }
    }

    for i in 0..n - 1 {
        if a[i] < k && k < a[i + 1] || a[i + 1] < k && k < a[i] {
            ans += 1;
        }
    }

    println!("{ans}");
}
