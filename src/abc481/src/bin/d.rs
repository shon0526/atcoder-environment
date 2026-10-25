use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::usize;

// リポジトリのURL: https://github.com/shon0526/atcoder-environment
const INF: i64 = 1_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
        ab: [(Usize1, Usize1); m],
    }

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
            if cost[nxt] != INF {
                continue;
            }
            if i == 0 {
                heapq.push_front(nxt);
                cost[nxt] = cost[now];
            } else {
                heapq.push_back(nxt);
                cost[nxt] = cost[now] + 1;
            }
        }
    }

    let ans = if cost[t] != INF { cost[t] } else { -1 };
    println!("{ans}");
}
