#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use permutohedron::LexicalPermutation;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// リポジトリのURL: https://github.com/shon0526/atcoder-environment
// 順列全探索

fn main() {
    input! {
        n: i64,
        p: [i64; n],
        q: [i64; n],
    }

    let mut cnt = 1;
    let mut a: i64 = 0;
    let mut b: i64 = 0;

    let mut cad: Vec<i64> = (1..=n).collect_vec();

    loop {
        if cad == p {
            a = cnt;
        }
        if cad == q {
            b = cnt;
        }

        cnt += 1;

        if !cad.next_permutation() {
            break;
        }
    }
    println!("{}", (a - b).abs());
}
