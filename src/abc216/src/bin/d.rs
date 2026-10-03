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
        m: usize,
    }

    let mut edges = Vec::new();

    for _ in 0..m {
        input! {
            k: usize,
            a: [Usize1; k],
        }

        for (u, v) in a.iter().copied().tuple_windows() {
            //            println!("u: {}, v: {}", u, v);
            if u == v {
                println!("No");
                return;
            }
            edges.push((u, v));
        }
    }

    let mut graph = scc::Scc::new(n, &edges);

    let scc = graph.scc();

    println!("{}", if scc.len() == n { "Yes" } else { "No" });
}

pub mod scc {
    #[derive(Debug)]
    pub struct Scc {
        g: Vec<Vec<usize>>,
        g_rev: Vec<Vec<usize>>,
        stack: Vec<usize>,
        seen: Vec<bool>,
    }
    impl Scc {
        pub fn new(n: usize, edges: &Vec<(usize, usize)>) -> Self {
            let mut g = vec![vec![]; n];
            let mut g_rev = vec![vec![]; n];
            for &(u, v) in edges {
                g[u].push(v);
                g_rev[v].push(u);
            }
            Self {
                g,
                g_rev,
                stack: Vec::new(),
                seen: vec![false; n],
            }
        }
        fn dfs(&mut self, v: usize) {
            self.seen[v] = true;
            for i in 0..self.g[v].len() {
                let w = self.g[v][i];
                if self.seen[w] {
                    continue;
                }
                self.dfs(w);
                self.stack.push(w);
            }
        }
        fn dfs_rv(&mut self, scc: &mut Vec<usize>, v: usize) {
            self.seen[v] = true;
            for i in 0..self.g_rev[v].len() {
                let w = self.g_rev[v][i];
                if self.seen[w] {
                    continue;
                }
                self.dfs_rv(scc, w);
                scc.push(w);
            }
        }
        pub fn scc(&mut self) -> Vec<Vec<usize>> {
            for v in 0..self.g.len() {
                if !self.seen[v] {
                    self.dfs(v);
                    self.stack.push(v);
                }
            }
            self.seen = vec![false; self.g.len()];
            let mut sccs = Vec::new();
            while let Some(v) = self.stack.pop() {
                if !self.seen[v] {
                    let mut scc = Vec::new();
                    self.dfs_rv(&mut scc, v);
                    scc.push(v);
                    sccs.push(scc);
                }
            }
            sccs
        }
    }
}
