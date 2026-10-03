#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 和は差分更新をする
// 各値を何個保持しているかをバケットで管理

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        bc: [(usize, usize); q],
    }

    let mut cnts = vec![0; 100_001];
    let mut sum = 0;

    for i in 0..n {
        cnts[a[i]] += 1;
        sum += a[i];
    }

    for &(b, c) in &bc {
        let cnt = cnts[b];
        sum -= cnt * b;
        sum += cnt * c;
        cnts[c] += cnt;
        println!("{}", sum);
        cnts[b] = 0;
    }
}
