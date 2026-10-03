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
        t: usize,
        a: [usize; n],
    }

    if n == 0 {
        println!("{}", t);
        return;
    }

    let mut res = 0;
    let mut open = 0;

    for i in 0..n {
        if open < a[i] {
            res += (a[i] - open);
            open = a[i] + 100;
        }
    }

    if open < t {
        res += (t - open);
    }
}
