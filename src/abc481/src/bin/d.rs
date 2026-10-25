use itertools::Itertools;
use libm::cos;
use pathfinding::directed::dijkstra;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::usize;

// リポジトリのURL: https://github.com/shon0526/atcoder-environment
// 01bfs
const INF: i64 = 1_000_000_000;

fn solve(n: usize, m: usize, s: usize, t: usize, ab: Vec<(usize, usize)>) -> i64 {
    let mut graph = vec![vec![]; n];

    for (a, b) in ab {
        graph[a].push(b);
    }

    for i in 0..n {
        graph[i].sort();
    }
    //    println!("{:?}", graph);

    let mut heapq = VecDeque::new();
    let mut cost: Vec<i64> = vec![INF; n];
    heapq.push_back(s);
    cost[s] = 0;

    while let Some(now) = heapq.pop_front() {
        for i in 0..graph[now].len() {
            let nxt = graph[now][i];
            let c = if i == 0 { 0 } else { 1 };

            let cos = cost[now] + c;

            if cos < cost[nxt] {
                cost[nxt] = cos;
                if c == 1 {
                    heapq.push_back(nxt);
                } else {
                    heapq.push_front(nxt);
                }
            }
        }
    }

    let ans = if cost[t] != INF { cost[t] } else { -1 };
    ans
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
        ab: [(Usize1, Usize1); m],
    }

    let mut way_g = vec![vec![]; n];
    for (a, b) in ab {
        way_g[a].push(b);
    }

    for i in 0..n {
        way_g[i].sort();
    }

    let mut graph = vec![vec![]; n];

    for i in 0..n {
        for j in 0..way_g[i].len() {
            let v = way_g[i][j];
            if j == 0 {
                graph[i].push((v, 0));
            } else {
                graph[i].push((v, 1));
            }
        }
    }

    let mut cost = dijkstra::dijkstra(&graph, s);
    println!("{}", if cost[t] != i64::MAX { cost[t] } else { -1 });
}

#[allow(clippy::module_inception)]
mod dijkstra {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    #[allow(dead_code)]
    pub fn dijkstra(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
        let n = graph.len();
        let mut dist = vec![i64::MAX; n];
        let mut hq = BinaryHeap::new();
        dist[s] = 0;
        hq.push(Reverse((dist[s], s)));
        while let Some(Reverse((c, now))) = hq.pop() {
            if dist[now] < c {
                continue;
            }
            for i in 0..graph[now].len() {
                let (nxt, w) = graph[now][i];
                if dist[nxt] > dist[now] + w {
                    dist[nxt] = dist[now] + w;
                    hq.push(Reverse((dist[nxt], nxt)))
                }
            }
        }
        dist
    }
}
