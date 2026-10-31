#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use permutohedron::LexicalPermutation;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// リポジトリのURL: https://github.com/shon0526/atcoder-environment

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ss: [Chars; n],
    }

    let mut ans = n;

    for bit in 0..(1 << n) {
        let mut seen = vec![false; m];
        let mut cnt = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                cnt += 1;
                for j in 0..m {
                    if ss[i][j] == 'o' {
                        seen[j] = true;
                    }
                }
            }
        }
        if seen.iter().all(|&b| b) {
            ans = ans.min(cnt);
        }
    }

    println!("{}", ans);
}
