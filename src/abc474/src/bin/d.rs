#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// ランダムテストをするときは、この main の中身を
// fn solve(input_str: &str) -> String に移し、main は下記の3行だけにする。
// (そのうえで stress/naive_test.rs を末尾に貼り付ける)
//
//   let mut buf = String::new();
//   std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap();
//   println!("{}", solve(&buf));
//
const INF: i64 = 1_000_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    let mut ans = (0..n)
        .map(|i| if a[i] > b[i] { INF } else { 1 })
        .collect_vec();

    if ans.iter().copied().all(|v| v == 1) {
        println!("No");
    } else {
        println!("Yes");
        println!("{}", ans.iter().join(" "));
    }
}
