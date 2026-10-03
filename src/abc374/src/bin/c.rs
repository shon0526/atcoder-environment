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
        ks: [usize; n],
    }
    let mut ans = usize::MAX;
    let mut ans_vec = Vec::new();
    let mut ks = Vec::from(ks);
    dfs(&ks, &mut ans_vec, 0, 0, 0, ans, n);
    println!("{}", ans_vec.iter().min().unwrap());
}

fn dfs(
    ks: &Vec<usize>,
    ans_vec: &mut Vec<usize>,
    pos: usize,
    mut a: usize,
    mut b: usize,
    mut ans: usize,
    n: usize,
) {
    if pos == n {
        ans = ans.min(a.max(b));
        ans_vec.push(ans);
        return;
    }

    a += ks[pos];
    dfs(ks, ans_vec, pos + 1, a, b, ans, n);
    a -= ks[pos];

    b += ks[pos];
    dfs(ks, ans_vec, pos + 1, a, b, ans, n);
    b -= ks[pos];
    return;
}
