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
        ab: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n as usize];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut ans = vec![0; n as usize];
    for i in 0..n {
        let mut st = graph[i as usize].len() as i64;
        st += 1;
        let x = n - st;
        ans[i as usize] = (x * (x - 1) * (x - 2)) / 6;
    }

    println!("{}", ans.iter().join(" "));
}
