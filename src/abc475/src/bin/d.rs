#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use num_traits::pow;
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
    mut        s: Chars,
        }

    let n = s.len();
    //    println!("{}", n);
    let mut i_prime = vec![true; pow(10, n)];
    i_prime[0] = false;
    i_prime[1] = false;

    for i in 2..pow(10, n) {
        if !i_prime[i] {
            continue;
        }

        let mut cnt = 2;
        while i * cnt < pow(10, n) {
            i_prime[i * cnt] = false;
            cnt += 1;
        }
    }

    for i in pow(10, n - 1)..pow(10, n) {
        if !i_prime[i] {
            continue;
        }

        let t = i.to_string().chars().collect_vec();

        let mut is_ok = true;

        for j in 0..n {
            for k in j..n {
                if (s[j] == s[k] && t[j] != t[k]) || (s[j] != s[k] && t[j] == t[k]) {
                    is_ok = false;
                }
            }
        }
        if is_ok {
            println!("{}", t.iter().join(""));
            return;
        }
    }

    println!("{}", -1);
}

fn prime(n: usize) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    let mut is_ok = true;
    for i in 2..=n.isqrt() {
        if n % i == 0 {
            is_ok = false;
        }
    }
    is_ok
}
