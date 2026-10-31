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
        m: usize,
        mut a: [usize; n],
    }
    a.sort();
    let cumsum = std::iter::once(0)
        .chain(a.iter().scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        }))
        .collect_vec();

    if cumsum[n] <= m {
        println!("infinite");
        return;
    }
    let ans = (1..n)
        .filter(|&i| {
            let res = cumsum[i] + a[i - 1] * (n - i);
            res <= m
        })
        .max();
    let ans = if let Some(i) = ans { a[i - 1] } else { m / n };
    println!("{}", ans);
}
