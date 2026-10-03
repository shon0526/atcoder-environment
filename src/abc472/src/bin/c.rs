#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

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
        k: usize,
        a: [usize; n],
    }

    let mut t_col = 0; // i-1日目までの直近m-1日間のカロリー
    let mut is_eat = vec![false; n];

    for i in 0..n {
        //        println!("i: {}, t_col: {}", i, t_col);
        //        println!("is_eat: {:?}", is_eat);
        if t_col + a[i] > k {
            if i >= m - 1 && is_eat[i + 1 - m] {
                t_col -= a[i + 1 - m];
            }
            println!("No");
            continue;
        }

        println!("Yes");
        t_col += a[i];
        is_eat[i] = true;

        if i >= m - 1 && is_eat[i + 1 - m] {
            t_col -= a[i + 1 - m];
        }
    }
}
