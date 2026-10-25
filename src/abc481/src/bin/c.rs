#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// リポジトリのURL: https://github.com/shon0526/atcoder-environment

fn main() {
    input! {
        q: usize,
    }

    let mut set = HashSet::new();
    let mut map = HashMap::new();

    let mut id = 1;
    for _ in 0..q {
        input! {
            s: String,
        }
        if set.contains(&s) {
            println!("{}", map[&s]);
        } else {
            println!("{}", id);
            *map.entry(s.clone()).or_insert(id);
            set.insert(s.clone());
            id += 1;
        }
    }
}
