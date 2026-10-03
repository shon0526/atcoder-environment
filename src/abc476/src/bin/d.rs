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
        n: usize,
        m: usize,
        k: i64,
        mut x: i64,
        mut y: i64,
        a: [i64; n],
        b: [i64; m],
    }

    let mut a = a.iter().copied().collect_vec();
    let mut b = b.iter().copied().collect_vec();
    a.sort();
    b.sort();

    let pa = iter::once(0)
        .chain(a.iter().scan(0, |sum, v| {
            *sum += *v;
            Some(*sum)
        }))
        .collect_vec();
    let mut v_cnt = vec![0; m + 1];

    for i in 0..m {
        v_cnt[i + 1] = v_cnt[i] + floor(b[i], k);
    }
    let mut v_res = vec![0; m + 1];
    for i in 0..m {
        v_res[i + 1] = v_res[i] + (k * floor(b[i], k) - b[i]);
    }

    let mut ans = 0;
    for i in 0..=m {
        if v_cnt[i] > y {
            break;
        }
        let res_x = x + (y - v_cnt[i]) * k + v_res[i];
        ans = ans.max(pa.upper_bound(&res_x) + i - 1);
    }

    println!("{}", ans);
}

fn floor(num: i64, k: i64) -> i64 {
    if num % k == 0 { num / k } else { (num + k) / k }
}
