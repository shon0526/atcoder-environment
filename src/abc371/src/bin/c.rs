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
        gm: usize,
        uv: [(Usize1, Usize1); gm],
        hm: usize,
        ab: [(Usize1, Usize1); hm],
    }

    let mut g = vec![vec![false; n]; n];
    let mut h = vec![vec![false; n]; n];

    for (u, v) in uv {
        g[u][v] = true;
        g[v][u] = true;
    }
    for (a, b) in ab {
        h[a][b] = true;
        h[b][a] = true;
    }

    let mut cost = Vec::new();

    for i in 0..n {
        let mut vec = Vec::new();
        for _ in 0..i + 1 {
            vec.push(0);
        }
        input! {
            xs: [usize; n-i -1],
        }
        for x in xs {
            vec.push(x);
        }
        cost.push(vec);
    }

    let ans = (0..n)
        .permutations(n)
        .map(|perm| {
            let mut res = 0;
            for i in 0..n - 1 {
                for j in i + 1..n {
                    if h[i][j] && g[perm[i]][perm[j]] {
                        continue;
                    }
                    if !h[i][j] && !g[perm[i]][perm[j]] {
                        continue;
                    }
                    res += cost[i][j];
                }
            }
            res
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
