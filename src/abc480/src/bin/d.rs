#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter;
use superslice::Ext;
// 必要な配列
// Aの累積和
// 長さNの配列Bを用意する (個数の累積和を管理)
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }

    let mut a_cumsum = iter::once(0)
        .chain(a.iter().scan(0, |sum, x| {
            *sum += *x;
            Some(*sum)
        }))
        .collect_vec();
    //  println!("cumsum {:?}", a_cumsum);

    let mut cnt = (0..=n)
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect_vec();
    //   println!("cnt {:?}", cnt);

    let b = a_cumsum
        .iter()
        .scan(0, |sum, x| {
            *sum += *x;
            Some(*sum)
        })
        .collect_vec();
    //    println!("b {:?}", b);

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        let low = cnt.lower_bound(&l);
        let up = cnt.lower_bound(&r);
        //      println!("low {}", low);
        //       println!("up {}", up);

        let cl = cnt[low] - cnt[low - 1];
        //     let cu = cnt[up] - cnt[up - 1];
        let mut ans = 0;
        //        println!("cl {} cu: {}", cl, cu);

        if low < up {
            ans += a_cumsum[cl] - a_cumsum[l - cnt[low - 1] - 1];
            ans += a_cumsum[r - cnt[up - 1]];
            ans += b[up - 1] - b[low];
        } else {
            ans += a_cumsum[r - cnt[up - 1]] - a_cumsum[l - cnt[low - 1] - 1];
        }

        println!("{}", ans);
    }
}
