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
        mut s: Chars,
    }

    let mut ans = s
        .windows(3)
        .filter(|&vec| vec[0] == 'A' && vec[1] == 'B' && vec[2] == 'C')
        .count();

    for _ in 0..q {
        input! {
            x: Usize1,
            c: char,
        }

        //A, B, Cとそれ以外の場合を列挙
        for i in 0..=2 {
            let idx = x as i64 - i;
            if idx >= 0 && idx + 2 < n as i64 {
                if s[idx as usize] == 'A'
                    && s[idx as usize + 1] == 'B'
                    && s[idx as usize + 2] == 'C'
                {
                    ans -= 1;
                }
            }
        }

        s[x] = c;
        for i in 0..=2 {
            let idx = x as i64 - i;
            if idx >= 0 && idx + 2 < n as i64 {
                if s[idx as usize] == 'A'
                    && s[idx as usize + 1] == 'B'
                    && s[idx as usize + 2] == 'C'
                {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}
