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
    }

    let mut ans = (1..=n)
        .into_iter()
        .filter(|&num| {
            let num_st = num.clone().to_string();
            let eight = eight_digit(num);

            let mut is_ten = true;
            let mut is_eight = true;

            for c in num_st.chars() {
                if c == '7' {
                    is_ten = false;
                }
            }

            for c in eight.chars() {
                if c == '7' {
                    is_eight = false;
                }
            }

            is_ten && is_eight
        })
        .count();

    println!("{}", ans);
}

fn eight_digit(num: usize) -> String {
    let mut s = "".to_string();
    let mut x = num;

    while x > 0 {
        s = (x % 8).to_string() + &s;
        x /= 8;
    }

    s
}
