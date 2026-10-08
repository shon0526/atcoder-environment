#![allow(unused_imports, dead_code)]
use ac_library::Dsu;
use ac_library::{Max, Segtree, segtree};
use itertools::Itertools;
use petgraph::graph;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 最小全域木の問題
// 重みの昇順に並び替える
// ある重みw_iのに対して、w_i * (uのグループ数) * (グループ数)を求めれば良い (u,vは元々別グループである)

fn main() {
    input! {
        n: usize,
        uvw: [(Usize1, Usize1, usize); n-1],
    }

    let mut wuv = uvw.iter().copied().map(|(u, v, w)| (w, u, v)).collect_vec();
    let mut uf = Dsu::new(n + 1);
    wuv.sort();

    let mut ans = 0;

    for &(w, u, v) in &wuv {
        ans += w * uf.size(u) * uf.size(v);
        uf.merge(u, v);
    }

    println!("{}", ans);
}
