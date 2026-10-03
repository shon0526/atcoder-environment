#![allow(unused_imports, dead_code)]
use indexing::algorithms::lower_bound;
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
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
        xs: [i64; n],
    }

    let mut ans = 0;
    let mut cur = 0;

    let mut p_heap = BinaryHeap::new();
    let mut n_heap = BinaryHeap::new();

    for x in xs {
        if x < 0 {
            n_heap.push(x);
        } else {
            p_heap.push(Reverse(x));
        }
    }
    loop {
        match (n_heap.peek().copied(), p_heap.peek().copied()) {
            (Some(x), Some(Reverse(y))) => {
                let x_abs = (cur - x).abs();
                let y_abs = (cur - y).abs();

                if x_abs <= y_abs {
                    ans += x_abs;
                    cur = x;
                    n_heap.pop().unwrap();
                } else {
                    ans += y_abs;
                    cur = y;
                    p_heap.pop().unwrap();
                }
            }
            (Some(x), None) => {
                ans += (x - cur).abs();
                cur = x;
                n_heap.pop().unwrap();
            }
            (None, Some(Reverse(y))) => {
                ans += (y - cur).abs();
                cur = y;
                p_heap.pop().unwrap();
            }
            (None, None) => break,
        }
    }

    println!("{}", ans);
}
