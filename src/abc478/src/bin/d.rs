#![allow(unused_imports, dead_code)]
use amplify::impl_into_stringly;
use itertools::Itertools;
use ndarray::ShapeBuilder;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use varisat::config::SolverConfig;

// ランダムテストをするときは、この main の中身を
// fn solve(input_str: &str) -> String に移し、main は下記の3行だけにする。
// (そのうえで stress/naive_test.rs を末尾に貼り付ける)
//
//   let mut buf = String::new();
//   std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap();
//   println!("{}", solve(&buf));

fn solve(n: usize, q: usize, queries: Vec<(usize, usize, usize)>) -> Vec<i64> {
    let mut imos: Vec<i64> = vec![0; n];
    let mut right_id: Vec<i64> = vec![-1; q + 1]; // 区間の右端を管理
    let mut queries = Vec::from(queries);

    queries.sort();

    for (l, r, x) in queries {
        //    println!("l: {} r: {} x: {}", l, r, x);
        if l as i64 > right_id[x] {
            //         println!("good");
            imos[l] += 1;
            if r + 1 <= n - 1 {
                imos[r + 1] -= 1;
            }
            right_id[x] = r as i64;
        } else if r as i64 > right_id[x] {
            //          println!("Yes");
            if right_id[x] as usize + 1 <= n - 1 {
                imos[right_id[x] as usize + 1] += 1;
            }
            if r + 1 <= n - 1 {
                imos[r + 1] -= 1;
            }
            right_id[x] = r as i64;
        }
        //        println!("{:?}", right_id);
    }
    for i in 0..n - 1 {
        imos[i + 1] += imos[i];
    }
    imos
}
fn main() {
    input! {
        n: usize,
        q: usize,
        mut queries: [(Usize1, Usize1, usize); q],
    }

    let queries = Vec::from(queries);
    let ans = solve(n, q, queries);
    println!("{}", ans.iter().join(" "));
}
//
// #[cfg(test)]
// mod test {
//     use crate::solve;
//     use rand::Rng;
//
//     fn random_generate() -> (usize, usize, Vec<(usize, usize, usize)>) {
//         let rnd = rand::rng();
//         let n = rnd.clone().random_range(1..=10);
//         let q = rnd.clone().random_range(1..=10);
//         let mut queries = Vec::new();
//         for _ in 0..q {
//             let mut l = rnd.clone().random_range(1..=n);
//             let mut r = rnd.clone().random_range(l..=n);
//             let x = rnd.clone().random_range(1..=q);
//             l -= 1;
//             r -= 1;
//             queries.push((l, r, x));
//         }
//         (n, q, queries)
//     }
//
//     #[test]
//     fn random_test() {
//         for _ in 0..100 {
//             let (n, q, queries) = random_generate();
//             eprintln!("n: {} queries: {:?}", n, queries);
//             solve(n, q, queries)
//         }
//     }
// }
