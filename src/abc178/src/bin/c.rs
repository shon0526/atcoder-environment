#![allow(unused_imports, dead_code)]
use ac_library::{Mod1000000007, ModInt1000000007, StaticModInt};
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

    let ans = mod_pow(10, n) - mod_pow(9, n) - mod_pow(9, n) + mod_pow(8, n);

    println!("{}", ans);
}

fn mod_pow(x: usize, n: usize) -> StaticModInt<Mod1000000007> {
    let mut num = ModInt1000000007::new(1);

    for _ in 0..n {
        num = num * ModInt1000000007::new(x);
    }

    num
}
