use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::cmp::min;
use std::collections::{BinaryHeap, HashMap};
//コードの修正を行う
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut rem = a.iter().map(|v| v % k).collect_vec();
    rem.sort();
    for i in 0..n {
        rem.push(rem[i] + k);
    }
    let mut ans = k;
    for i in 0..n {
        let now = rem[i + n - 1] - rem[i];
        ans = min(ans, now);
    }
    println!("{}", ans);
}
