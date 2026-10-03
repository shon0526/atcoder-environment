#![allow(unused_imports, dead_code)]
use ac_library::{Max, Min, Segtree};
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
        n: usize,
        m: usize,
        mut a: [usize; n],
        b: [usize; m],
    }

    let cummin = iter::once(200_001)
        .chain(a.iter().scan(200_001, |m: &mut usize, v| {
            *m = (*m).min(*v);
            Some(*m)
        }))
        .collect_vec();

    for i in 0..m {
        let ans = cummin.partition_point(|&x| x > b[i]);
        if ans <= n {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }

    // let mut min_segtree: Segtree<Min<usize>> = Segtree::new(n);
    //
    // for i in 0..n {
    //     min_segtree.set(i, a[i]);
    // }
    //
    // for i in 0..m {
    //     let idx = min_segtree.max_right(0, |&j| b[i] < j);
    //
    //     if idx == n {
    //         println!("-1");
    //     } else {
    //         println!("{}", idx + 1);
    //     }
    // }
}
