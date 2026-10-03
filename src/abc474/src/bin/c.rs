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
        q: usize,
        p: [usize; n],
        xs: [usize; q],
    }

    let mut idx = vec![0; n + 1];

    for i in 0..n {
        idx[p[i]] = i + 1;
    }

    let mut q_p = Vec::new();

    for i in (0..q).rev() {
        if idx[xs[i]] != usize::MAX {
            q_p.push(xs[i]);
            idx[xs[i]] = usize::MAX;
        }
    }
    //    println!("{:?}", idx);
    //    println!("{:?}", q_p);

    let mut ans = Vec::new();

    for i in 0..n {
        if idx[p[i]] != usize::MAX {
            ans.push(p[i]);
        }
    }

    while let Some(v) = q_p.pop() {
        ans.push(v);
    }

    println!("{}", ans.iter().join(" "));
}
