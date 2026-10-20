#![allow(unused_imports, dead_code)]
use itertools::{Itertools, izip};
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }

    let mut cnt = vec![0; n];

    for i in 0..m {
        let mut is_ok = false;
        let mut count = 0;
        let mut id = 0;
        for j in 0..n {
            if t[i] == s[j] {
                cnt[j] += 1;
                is_ok = true;
            } else if t[i].len() == s[j].len() {
                let xs = t[i].clone();
                let ys = s[j].clone();
                let mut diff = izip!(xs, ys).filter(|(x, y)| x != y).count();

                if diff == 1 {
                    count += 1;
                    id = j;
                }
            }
        }
        if !is_ok && count == 1 {
            cnt[id] += 1;
        }
    }
    for i in 0..n {
        println!("{}", cnt[i]);
    }
}
