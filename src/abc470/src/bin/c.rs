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
        q: usize,
    }

    let mut distr = vec![0; n];

    let mut ans = 0;
    let mut pos_vec = Vec::new();

    for _ in 0..q {
        input! {
            num: usize,
        }

        if num == 1 {
            input! {
                x:Usize1,
            }

            if distr[x] == 0 {
                pos_vec.push(x);
            }

            ans ^= distr[x];
            distr[x] += 1;
            ans ^= distr[x];
        } else {
            let mut cad = Vec::new();
            while let Some(idx) = pos_vec.pop() {
                ans ^= distr[idx];
                distr[idx] -= 1;
                ans ^= distr[idx];
                if distr[idx] >= 1 {
                    cad.push(idx);
                }
            }

            pos_vec = cad.clone();
        }
        println!("{}", ans);
    }
}
