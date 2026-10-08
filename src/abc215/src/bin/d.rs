#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use primal::is_prime;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;

// ランダムテストをするときは、この main の中身を
// fn solve(input_str: &str) -> String に移し、main は下記の3行だけにする。
// (そのうえで stress/naive_test.rs を末尾に貼り付ける)
//
//   let mut buf = String::new();
//   std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap();
//   println!("{}", solve(&buf));

// 明日実装し直す
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [usize; n],
    }

    let mut is_p = vec![true; m + 1];
    is_p[0] = false;

    for x in xs {
        let fact = math::prime_fact(x);
        for &(p, _) in &fact {
            if p == 1 || p > m {
                continue;
            }
            if !is_p[p] {
                continue;
            }
            let mut v = p;
            let mut cnt = 1;
            while v <= m {
                is_p[v] = false;
                cnt += 1;
                v = p * cnt;
            }
        }
    }

    println!("{}", is_p.iter().copied().filter(|&b| b).count());
    println!("{}", (1..=m).filter(|&i| is_p[i]).join("\n"));
}

pub mod math {
    pub fn prime_fact(n: usize) -> Vec<(usize, usize)> {
        if n < 4 {
            return vec![(n, 1)];
        }
        let mut n = n;
        let mut fact = Vec::new();
        for i in 2..=n.isqrt() {
            if n % i == 0 {
                let mut cnt = 0;
                while n % i == 0 {
                    cnt += 1;
                    n /= i;
                }
                fact.push((i, cnt));
            }
        }
        if n != 1 {
            fact.push((n, 1));
        }
        fact
    }
}
