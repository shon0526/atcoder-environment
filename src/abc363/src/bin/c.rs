#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use permutohedron::LexicalPermutation;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// リポジトリのURL: https://github.com/shon0526/atcoder-environment
// 順列全探索（場合によって重複があることがある）
// next_permutationを使えば高速で候補を列挙することが可能

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    }

    let mut ans = 0;

    s.sort();
    loop {
        let cnt = s.windows(k).filter(|&vec| is_pal(&vec)).count();
        if cnt == 0 {
            ans += 1;
        }
        if !s.next_permutation() {
            break;
        }
    }

    println!("{}", ans);
}

fn is_pal(s: &[char]) -> bool {
    let len = s.len();
    let mut is_ok = true;
    for i in 0..len / 2 {
        if s[i] != s[len - i - 1] {
            is_ok = false;
        }
    }
    is_ok
}
