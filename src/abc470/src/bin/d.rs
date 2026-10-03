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
        p: [Usize1; n],
    }

    let mut p1 = Vec::from(p.clone()); // 2が偶数回の時
    let mut p2 = vec![0; n]; //2が奇数回の時
    let mut flag = true;

    for (i, &v) in p.iter().enumerate() {
        p2[v] = i;
    }

    for _ in 0..q {
        input! {
            num: usize,
        }

        if num == 1 {
            input! {
                x: Usize1,
                y: Usize1,
            }
            if flag {
                let p1_x = p1[x]; // p1のx番目の要素
                let p1_y = p1[y]; // p1のy番目の要素
                p1.swap(x, y);
                p2.swap(p1_x, p1_y);
            } else {
                let p2_x = p2[x];
                let p2_y = p2[y];
                p2.swap(x, y);
                p1.swap(p2_x, p2_y);
            }
        } else {
            flag ^= true;
        }
    }

    if flag {
        println!("{}", p1.iter().map(|i| i + 1).join(" "));
    } else {
        println!("{}", p2.iter().map(|i| i + 1).join(" "));
    }
}
