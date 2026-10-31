#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use num_integer::div_ceil;
use num_traits::pow;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// リポジトリのURL: https://github.com/shon0526/atcoder-environment

fn main() {
    input! {
        mut sx: i64,
        sy: i64,
        mut tx: i64,
        ty: i64,
    }

    let mut ans = 0;

    if sy != ty {
        ans += (sy - ty).abs();
    }
    //    println!("{}", ans);

    if sx > tx {
        if sy % 2 == 1 && sx % 2 == 0 || sy % 2 == 0 && sx % 2 == 1 {
            sx -= 1;
        }
        if ty % 2 == 0 && tx % 2 == 0 || ty % 2 == 1 && tx % 2 == 1 {
            tx += 1
        }
        let p = sx - (sy - ty).abs();
        if tx >= p {
            println!("{}", ans);
            return;
        } else {
            sx -= (sy - ty).abs();
            if ty % 2 == 0 && sx % 2 == 1 || ty % 2 == 1 && sx % 2 == 0 {
                sx -= 1;
            }
            ans += div_ceil((sx - tx).abs(), 2);
            println!("{}", ans);
            return;
        }
    }
    if sx < tx {
        if sy % 2 == 1 && sx % 2 == 1 || sy % 2 == 0 && sx % 2 == 0 {
            sx += 1;
        }
        if ty % 2 == 0 && tx % 2 == 1 || ty % 2 == 1 && tx % 2 == 0 {
            tx -= 1;
        }
        let p = sx + (sy - ty).abs();
        if tx <= p {
            println!("{}", ans);
        } else {
            sx += (sy - ty).abs();
            if ty % 2 == 0 && sx % 2 == 0 || ty % 2 == 1 && sx % 2 == 1 {
                sx += 1;
            }
            ans += div_ceil((sx - tx).abs(), 2);
            println!("{}", ans);
        }
        return;
    }
    println!("{}", ans);
}
