#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

const WAY: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn main() {
    input! {
        s: [Chars; 2],
    }

    let a = s[0][0];
    let b = s[0][1];
    let c = s[1][0];
    let d = s[1][1];

    let ans = [[a, b], [a, c], [b, d], [c, d]]
        .iter()
        .any(|&[x, y]| [x, y] == ['A', 'D'] || [x, y] == ['D', 'A']);
    println!("{}", if ans { "Yes" } else { "No" });
    //
    // for i in 0..2 {
    //     for j in 0..2 {
    //         if s[i][j] == 'A' {
    //             for &(x, y) in &WAY {
    //                 if i as i64 + x < 0 || i as i64 + x > 1 || j as i64 + y < 0 || j as i64 + y > 1
    //                 {
    //                     continue;
    //                 }
    //                 let nx = i as i64 + x;
    //                 let ny = j as i64 + y;
    //                 if s[nx as usize][ny as usize] == 'D' {
    //                     println!("Yes");
    //                     return;
    //                 }
    //             }
    //         }
    //         if s[i][j] == 'D' {
    //             for &(x, y) in &WAY {
    //                 if i as i64 + x < 0 || i as i64 + x > 1 || j as i64 + y < 0 || j as i64 + y > 1
    //                 {
    //                     continue;
    //                 }
    //                 let nx = i as i64 + x;
    //                 let ny = j as i64 + y;
    //                 if s[nx as usize][ny as usize] == 'A' {
    //                     println!("Yes");
    //                     return;
    //                 }
    //             }
    //         }
    //     }
    // }

    //    println!("No");
}
