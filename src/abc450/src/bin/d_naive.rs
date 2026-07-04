use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::{
    collections::{BinaryHeap, HashMap},
    usize,
};

use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    let mut rem = a.iter().map(|v| v % k).collect_vec();
    rem.sort();
    let ans_vec = (0..n - 1)
        .into_iter()
        .map(|i| rem[i + 1] - rem[i])
        .collect_vec();

    let ans = ans_vec.iter().max().unwrap();
    println!("{}", k - ans);
}
