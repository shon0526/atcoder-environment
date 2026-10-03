#![allow(unused_imports, dead_code)]
use alga::linear::Transformation;
use itertools::{Itertools, any};
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
        p: [usize; n],
    }

    let mut is_ok = true;

    for i in 0..n {
        if i / 10 != (p[i] - 1) / 10 {
            is_ok = false;
        }
    }

    println!("{}", if is_ok { "Yes" } else { "No" });
}
