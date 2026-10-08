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
        xs: [Usize1; m],
        ys: [usize; m],
    }

    let mut xy = (0..m).map(|i| (xs[i], ys[i])).collect_vec();
    xy.sort();
    xy.push((n - 1, 0));
    if xy[0].0 != 0 {
        println!("-1");
        return;
    }

    let mut ans = 0;

    for i in 0..m {
        let (x, y) = xy[i];
        let (nx, ny) = xy[i + 1];
        let diff = (nx - x);
        if y < diff {
            println!("-1");
            return;
        }
        ans += diff * (diff - 1) / 2;
        ans += diff * (y - diff);
        xy[i + 1].1 += y - diff;
    }
    //    println!("{:?}", xy);

    if xy[m].1 != 1 {
        println!("-1");
        return;
    }

    println!("{ans}");
}
