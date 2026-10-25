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

    if a[0] == k {
        ans += 1;
    }

    for i in 1..n {
        if a[i - 1] == k && a[i] == k {
            ans += 1;
        }
        if a[i - 1] > k && a[i] <= k {
            ans += 1;
        }
        if a[i - 1] < k && a[i] >= k {
            ans += 1;
        }
    }

    if a[0] == a[1] {
        ans += 1;
    }

    println!("{ans}");
}
