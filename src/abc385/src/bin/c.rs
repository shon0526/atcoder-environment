#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::time;

// ランダムテストをするときは、この main の中身を
// fn solve(input_str: &str) -> String に移し、main は下記の3行だけにする。
// (そのうえで stress/naive_test.rs を末尾に貼り付ける)
//
//   let mut buf = String::new();
//   std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap();
//   println!("{}", solve(&buf));
fn main() {
    let now = time::Instant::now();
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut dp = vec![vec![1; n]; n];

    for i in 0..n {
        for j in 1..n {
            if i + j > n - 1 {
                continue;
            }
            if h[i] == h[i + j] {
                dp[i + j][j] = dp[i][j] + 1;
            } else {
                dp[i + j][j] = 1;
            }
        }
    }

    let ans = dp
        .iter()
        .map(|vec| vec.iter().max().unwrap())
        .max()
        .unwrap();
    println!("{ans}");
    println!("{}ms", now.elapsed().as_millis());
}
