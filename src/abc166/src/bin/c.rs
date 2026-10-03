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
        m: usize,
        h: [usize; n],
        ab: [(Usize1, Usize1); m],
    }

    let mut grid = vec![vec![]; n];
    let mut set = HashSet::new();
    for &(a, b) in &ab {
        if !set.contains(&(a, b)) {
            grid[a].push(b);
            grid[b].push(a);
        }

        set.insert((a, b));
    }

    //   println!("{:?}", grid);
    let mut ans = 0;

    for i in 0..n {
        let mut is_ok = true;
        if grid[i].len() == 0 {
            ans += 1;
            //         println!("i: {}, ans: {}", i, ans);
            continue;
        }

        for &j in &grid[i] {
            if h[i] <= h[j] {
                is_ok = false;
            }
            //            println!("h[i]: {}, h[j]: {}", h[i], h[j]);
        }
        if is_ok {
            ans += 1;
        }
        //      println!("i: {}, ans: {}", i, ans);
    }

    println!("{}", ans);
}
