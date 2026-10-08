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
         x: usize,
    }

    let mut st = (0..50)
        .map(|i| if i < 25 { 'A' } else { 'C' })
        .collect_vec();

    for _ in 0..(625 - x) {
        for i in 0..st.len() {
            if (st[i], st[i + 1]) == ('A', 'C') {
                st.swap(i, i + 1);
                break;
            }
        }
    }

    let mut ans = vec!['R'; 100];

    for i in 0..st.len() {
        ans[2 * i + 1] = st[i];
    }

    println!("{}", ans.iter().join(""));
}
