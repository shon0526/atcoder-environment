#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 辞書順での列挙は深さ優先探索が便利
//

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut seq = Vec::new();
    dfs(n, &mut seq, k, 0, 0);
}

fn dfs(n: usize, seq: &mut Vec<usize>, tol: usize, mut i: usize, mut coef: usize) {
    coef += 1;
    if i == n - 1 {
        if tol % coef == 0 {
            seq.push(tol / coef);
            println!("{}", seq.iter().join(" "));
            seq.pop();
        }

        return;
    }

    for x in 0..=tol / coef {
        seq.push(x);
        dfs(n, seq, tol - (coef * x), i + 1, coef);
        seq.pop();
    }
}
