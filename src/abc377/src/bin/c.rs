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
        n: i64,
        m: usize,
        ab: [(i64, i64); m],
    }

    let mut s = HashSet::new();

    for (a, b) in ab {
        s.insert((a, b));
        for i in [-1, 1] {
            for j in [-2, 2] {
                if a + i > 0 && a + i <= n && b + j > 0 && b + j <= n {
                    s.insert((a + i, b + j));
                }
                if a + j > 0 && a + j <= n && b + i > 0 && b + i <= n {
                    s.insert((a + j, b + i));
                }
            }
        }
    }

    let ans = n * n - s.len() as i64;
    println!("{}", ans);
}
