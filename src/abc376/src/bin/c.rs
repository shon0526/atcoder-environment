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
        a: [usize; n],
        b: [usize; n-1],
    }

    let mut a = Vec::from(a);
    a.sort();
    let mut b = Vec::from(b);

    let mut f = |x: usize| -> bool {
        let mut fb = b.clone();
        fb.push(x);
        fb.sort();
        for i in 0..n {
            if a[i] > fb[i] {
                return false;
            }
        }
        true
    };

    let mut left = 1;
    let mut right = 1_000_000_001;

    while right > left {
        let mid = (right + left) / 2;
        if f(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    if left == 1_000_000_001 {
        println!("-1");
    } else {
        println!("{}", left);
    }
}
