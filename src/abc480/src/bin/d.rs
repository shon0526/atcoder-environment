#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter;
use superslice::Ext;
// 必要な配列
// Aの累積和
// 長さNの配列Bを用意する (個数の累積和を管理)
// https://github.com/shon0526/atcoder-environment

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }

    let acumsum = iter::once(0)
        .chain(a.iter().scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        }))
        .collect_vec();

    let mut b = Vec::new();
    b.push((0, 0));
    for i in 1..=n {
        let (x, y) = b[b.len() - 1];
        b.push((x + i, y + acumsum[i]));
    }

    let f = |x: usize| -> i64 {
        let idx = b.upper_bound(&(x, 10_i64.pow(18))) - 1;
        let res = b[idx].1 + (acumsum[x - b[idx].0]);
        res
    };
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        println!("{}", f(r) - f(l - 1));
    }
}
