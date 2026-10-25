#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// リポジトリのURL: https://github.com/shon0526/atcoder-environment

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    //    println!("{}", (a + b) / 10 + 1);
    let ans = (a + b).to_string().len();
    println!("{}", ans);
}
