#![allow(unused_imports, dead_code)]
use ac_library::{Max, Min, Segtree};
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
        k: usize,
        a: [usize; n],
    }

    if n == 1 {
        println!("Yes");
        return;
    }

    let mut seg_min: Segtree<Min<usize>> = Segtree::new(n);
    let mut seg_max: Segtree<Max<usize>> = Segtree::new(n);

    for i in 0..n {
        seg_max.set(i, a[i]);
        seg_min.set(i, a[i]);
    }

    let mut left = 0;

    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            left = i - 1;
            break;
        }
    }

    let mut is_ok = true;

    if left + k <= n - 1 {
        for i in (left + k..n - 1) {
            if a[i] > a[i + 1] {
                is_ok = false;
            }
        }
    }

    if !is_ok {
        println!("No");
        return;
    }

    let mut ans1 = true;
    let mut ans2 = true;

    if left >= 1 {
        ans1 = a[left - 1] <= seg_min.prod(left..left + k);
    }
    if left + k <= n - 1 {
        ans2 = seg_max.prod(left..left + k) <= a[left + k];
    }
    println!("{}", if ans1 && ans2 { "Yes" } else { "No" });
}
