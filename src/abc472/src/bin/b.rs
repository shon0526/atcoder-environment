#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::usize;

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
        l: [i64; n],
    }

    let mut prefix = vec![0; n];
    prefix[0] = l[0];

    for i in 1..n {
        prefix[i] = prefix[i - 1] + l[i];
    }

    //    println!("{:?}", prefix);
    let mut ans = i64::MAX;

    for i in 0..n - 1 {
        let diff = (prefix[n - 1] - prefix[i] * 2).abs();
        //       dbg!(diff);
        if ans > diff {
            ans = diff;
        }
    }

    println!("{}", ans);
}
