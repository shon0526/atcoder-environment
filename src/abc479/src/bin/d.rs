#![allow(unused_imports, dead_code)]
use ac_library::{Min, Segtree};
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter;

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
        q: usize,
        l: [usize; n-1],
        mut r: [usize; n-1],
    }

    let l = iter::once(usize::MAX).chain(l).collect_vec();
    r.push(usize::MAX);
    let mut l_pmin = vec![usize::MAX; n];
    let mut r_pmin = vec![usize::MAX; n];
    l_pmin[n - 1] = l[n - 1];
    r_pmin[0] = r[0];
    for i in (1..n).rev() {
        l_pmin[i - 1] = l_pmin[i].min(l[i - 1]);
    }
    for i in (1..n) {
        r_pmin[i] = r_pmin[i - 1].min(r[i]);
    }

    for _ in 0..q {
        input! {
            s: Usize1,
            t: Usize1,
        }

        if s < t {
            if s == 0 {
                println!("{}", r[0]);
                continue;
            }
            // 左 -> 右
            let lc = l[s] + r_pmin[s - 1];
            let rc = r[s];
            println!("{}", lc.min(rc));
        } else {
            if s == n - 1 {
                println!("{}", l[n - 1]);
                continue;
            }

            let lc = l[s];
            let rc = r[s] + l_pmin[s + 1];
            //            println!("lc: {} rc: {}", lc, rc);
            println!("{}", lc.min(rc));
        }
    }
}
