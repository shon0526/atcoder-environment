#![allow(unused_imports, dead_code)]
use ac_library::Dsu;
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
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut uf = Dsu::new(n);

    let mut graph = vec![vec![]; n];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
        uf.merge(a, b);
    }

    let is_ok = graph.iter().all(|vec| vec.len() == 2) && uf.size(0) == n;
    println!("{}", if is_ok { "Yes" } else { "No" });
}
