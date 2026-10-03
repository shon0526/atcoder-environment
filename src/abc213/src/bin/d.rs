#![allow(unused_imports, dead_code)]
use glidesort::sort;
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 頂点順にたどる -> DFSを使えば良い
// ある頂点に対して、次に通る点はまだ一度も訪れていない頂点番号が小さい順に選ぶ -> 事前にソートしておく
// ずべての辺を双方向に一度ずつ通る仕組み -> オイラーツアー (Eular Tour)

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }

    let mut graph = vec![vec![]; n + 1];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    for i in 0..n + 1 {
        graph[i].sort();
    }

    let mut path = Vec::new();
    dfs(&graph, &mut path, 1, 0);

    println!("{}", path.iter().join(" "));
}

fn dfs(graph: &Vec<Vec<usize>>, path: &mut Vec<usize>, now: usize, pre: usize) {
    path.push(now);

    for &nxt in &graph[now] {
        if pre == nxt {
            continue;
        }
        dfs(graph, path, nxt, now);
        path.push(now);
    }
}
