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
fn main() {
    input! {
        n: usize,
        v: usize,
        ws: [usize; n],
    }

    let mut ans = (0..n)
        .permutations(3)
        .filter(|vec| {
            let i = vec[0];
            let j = vec[1];
            let k = vec[2];
            (i + 1) + (j + 1) + (k + 1) <= v
        })
        .map(|vec| {
            let i = vec[0];
            let j = vec[1];
            let k = vec[2];
            ws[i] + ws[j] + ws[k]
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
