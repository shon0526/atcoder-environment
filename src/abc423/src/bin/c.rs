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
//

fn solve(n: usize, mut r: usize, l: Vec<usize>) -> usize {
    let min = (0..n).filter(|&i| l[i] == 0).min().unwrap_or(r);
    let max = (0..n).filter(|&i| l[i] == 0).max().unwrap_or(r);

    let mut ans = 0;

    if min < r {
        for i in min + 1..r {
            if l[i] == 1 {
                ans += 2;
            }
        }
    }

    if max >= r {
        for i in r..max {
            if l[i] == 1 {
                ans += 2;
            }
        }
    }

    for i in 0..n {
        if l[i] == 0 {
            ans += 1;
        }
    }
    ans
}

fn naive(n: usize, r: usize, l: Vec<usize>) -> usize {
    let min = (0..n).filter(|&i| l[i] == 0).min().unwrap_or(r);
    let max = (0..n).filter(|&i| l[i] == 0).max().unwrap_or(r);

    let mut ans = 0;

    if min < r {
        for i in min..r {
            if i == min {
                ans += 1;
            } else if l[i] == 1 {
                ans += 2;
            } else {
                ans += 1;
            }
        }
    }
    //println!("min_ans: {}", ans);

    if max > r {
        for i in r + 1..=max {
            if i == max {
                ans += 1;
            } else if l[i] == 1 {
                ans += 2;
            } else {
                ans += 1;
            }
        }
    }
    // println!("max_ans: {}", ans);

    if min < r && max > r {
        //      println!("ok");
        if l[r] == 1 {
            ans += 2;
        } else {
            //           println!("okno");
            ans += 1;
        }
    } else {
        //        println!("no");
        if l[r] == 0 {
            ans += 1;
        }
    }

    ans
}

fn main() {
    input! {
        n: usize,
        r: usize,
        l: [usize; n],
    }

    println!("{}", solve(n, r, l));
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use rand::prelude::*;

    use crate::solve;

    fn sampling() -> (usize, usize, Vec<usize>) {
        let mut rng = rand::rng();
        let n: usize = rng.random_range(2..=250000);
        let r: usize = rng.random_range(1..=n) - 1;
        let mut l: Vec<usize> = (0..n).map(|_| rng.random_range(0..=1)).collect_vec();
        (n, r, l)
    }

    #[test]
    fn test() {
        for _ in 0..1000 {
            let (n, r, l) = sampling();
            let sol = solve(n, r, l.clone());
            let nai = solve(n, r, l.clone());
            assert_eq!(sol, nai, "n: {} r: {} l: {:?}", n, r, l);
        }
    }
}
