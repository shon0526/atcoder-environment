#![allow(unused_imports, dead_code)]
use indexing::algorithms::lower_bound;
use itertools::{Itertools, cloned};
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
        a: [usize;n],
    }

    let mut map = HashMap::new();
    let mut ans = vec![-1; n];

    for i in 0..n {
        if let Some(&v) = map.get(&a[i]) {
            ans[i] = v as i64;
            *map.get_mut(&a[i]).unwrap() = i + 1;
        } else {
            map.entry(a[i]).or_insert(i + 1);
        }
    }
    println!("{}", ans.iter().join(" "));
}
