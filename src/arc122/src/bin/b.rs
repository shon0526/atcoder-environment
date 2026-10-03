#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use itertools_num::structs::Cumsum;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter::{self, Once};

// min(A_i, 2x)

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    let a_f = a.iter().copied().map(|v| v as f64).collect_vec();
    let mut cumsum = a
        .iter()
        .scan(0, |cum, x| {
            *cum += *x;
            Some(*cum)
        })
        .collect_vec();

    let cumsum = cumsum.iter().map(|cum| *cum as f64).collect_vec();
    let cumsum = iter::once(0.0).chain(cumsum).collect_vec();

    let mut ans = cumsum[n];
    for i in 0..n {
        let x = a_f[i] / 2.0;
        ans = ans.min(n as f64 * x + cumsum[n] - cumsum[i] - 2.0 * x * (n - i) as f64);
    }

    println!("{}", ans / n as f64);
}
