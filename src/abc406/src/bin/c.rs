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

//大小関係は折れ線グラフを書くとよい
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut ineq: Vec<(char, usize)> = Vec::new();

    for i in 0..n - 1 {
        if p[i] > p[i + 1] {
            if ineq.len() > 0 {
                if ineq[ineq.len() - 1].0 == '<' {
                    ineq.push(('>', 1));
                } else {
                    let len = ineq.len();
                    ineq[len - 1].1 += 1;
                }
            } else {
                ineq.push(('>', 1));
            }
        } else {
            //            println!("<");
            if ineq.len() > 0 {
                if ineq[ineq.len() - 1].0 == '>' {
                    ineq.push(('<', 1));
                } else {
                    let len = ineq.len();
                    ineq[len - 1].1 += 1;
                }
            } else {
                ineq.push(('<', 1));
            }
        }
    }

    let mut ans = 0;
    for i in 1..ineq.len() - 1 {
        if ineq[i].0 == '>' {
            ans += ineq[i - 1].1 * ineq[i + 1].1;
        }
    }
    println!("{}", ans);
}
