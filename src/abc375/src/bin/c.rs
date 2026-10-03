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
        b: [Chars; n]
    }

    let mut b = Vec::from(b);
    let mut ans_vec = vec![vec!['.'; n]; n];

    for i in 0..n {
        for j in 0..n {
            let d = *[i + 1, j + 1, n - i, n - j].iter().min().unwrap();
            let mut ni = i;
            let mut nj = j;
            let mut c = d % 4;

            for _ in 0..c {
                let ti = nj;
                let tj = n - ni - 1;
                ni = ti;
                nj = tj;
            }
            ans_vec[ni][nj] = b[i][j];
        }
    }

    for i in 0..n {
        println!("{}", ans_vec[i].iter().join(""));
    }
}
