#![allow(unused_imports, dead_code)]
use ac_library::{FenwickTree, fenwicktree};
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 単純に全てにX_iを足していると、時間がかかってしまう
// offsetを持たせて、x - offsetを優先度付きキューにプッシュすると良い
// 最後にキューから取り出した値にoffsetをつけるのを忘れずに

fn main() {
    input! {
        q: usize,
    }

    let mut heapq = BinaryHeap::new();
    let mut add = 0;

    for _ in 0..q {
        input! {
            n: usize,
        }

        match n {
            1 => {
                input! {
                    x: i64,
                }

                heapq.push(Reverse(x - add));
            }
            2 => {
                input! {
                    x: i64,
                }
                add += x;
            }
            _ => {
                if let Some(Reverse(v)) = heapq.pop() {
                    let ans = v + add;
                    println!("{}", ans);
                }
            }
        }
    }
}
