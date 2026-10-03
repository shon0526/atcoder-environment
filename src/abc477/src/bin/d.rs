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

    let mut tile = vec![0; n];
    let mut color = vec![' '; n];
    let mut s = HashSet::new();
    let mut query = Vec::new();

    for _ in 0..q {
        input! {
            num: usize,
        }
        match num {
            1 => {
                input! {x: Usize1}
                tile[x] ^= 1;
                query.push((1, x, ' '));
            }
            _ => {
                input! {c: char}
                query.push((2, usize::MAX, c))
            }
        }
    }

    for i in 0..n {
        if tile[i] == 0 {
            s.insert(i);
        }
    }

    for i in (0..q).rev() {
        let (num, x, c) = query[i];

        match num {
            1 => {
                if tile[x] == 1 && color[x] == ' ' {
                    s.insert(x);
                } else if tile[x] == 0 && color[x] == ' ' {
                    s.remove(&x);
                }
                tile[x] ^= 1;
            }
            _ => {
                for &v in &s {
                    color[v] = c;
                }
                s.clear();
            }
        }
    }
    println!(
        "{}",
        color
            .iter()
            .map(|&c| if c == ' ' { 'a' } else { c })
            .join("")
    );
}
