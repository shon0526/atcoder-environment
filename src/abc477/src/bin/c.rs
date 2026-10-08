#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter;
use superslice::Ext;

// ランダムテストをするときは、この main の中身を
// fn solve(input_str: &str) -> String に移し、main は下記の3行だけにする。
// (そのうえで stress/naive_test.rs を末尾に貼り付ける)
//
//   let mut buf = String::new();
//   std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap();
//   println!("{}", solve(&buf));
fn main() {
    input! {
        q: usize,
        s: Chars,
        t: Chars,
    }

    let n = s.len();
    let m = t.len();
    let mut a = vec![0; n];
    for i in 0..n - m + 1 {
        if s.len() < t.len() {
            continue;
        }
        let mut is_ok = true;
        for j in 0..m {
            if s[i + j] != t[j] {
                is_ok = false;
            }
        }
        if is_ok {
            a[i] = 1;
        }
    }
    let p = iter::once(0)
        .chain(a.iter().scan(0, |sum, v| {
            *sum += *v;
            Some(*sum)
        }))
        .collect_vec();

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        if r - l + 1 < m {
            println!("No");
            continue;
        }

        if p[r - m + 1] - p[l - 1] > 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
