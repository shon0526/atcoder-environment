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
        lr: [(i64, i64); n],
    }

    let l = lr.iter().copied().map(|(l, _)| l).collect_vec();
    let r = lr.iter().copied().map(|(_, r)| r).collect_vec();

    let sum_l = l.iter().sum::<i64>();
    let sum_r = r.iter().sum::<i64>();

    if sum_l > 0 || sum_r < 0 {
        println!("No");
        return;
    }

    let mut x = l.iter().copied().map(|x| x).collect_vec();
    let mut sum = x.iter().sum::<i64>();

    for i in 0..n {
        let d = (r[i] - l[i]).min(-sum);
        sum += d;
        x[i] += d;
    }

    println!("Yes");
    println!("{}", x.iter().join(" "));
}
