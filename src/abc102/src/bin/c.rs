#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// bがどんな値を取れば良いか考える
// |(a_1 - 1) - b| + |(a_2 - 2) - b| + ...
// c_i = a_i - (i+1)となる配列Cを作成して、その配列の中央値bを求める
// bに計算した値を代入して問題の定義通りに計算する

fn solve(n: &usize, a: &Vec<i64>) -> i64 {
    let mut a_ch = vec![0; *n];

    let mut ans = 0;
    for i in 0..*n {
        a_ch[i] = a[i] - (i + 1) as i64;
    }

    let mut a_ch2 = a_ch.clone();
    a_ch2.sort();
    let b = a_ch2[n / 2];

    for i in 0..*n {
        ans += (a_ch[i] - b).abs();
    }
    ans
}
fn naive(n: &usize, a: &Vec<i64>) -> i64 {
    let mut ans = 1_000_000_000_000_000;
    for b in -1_000_000..1_000_000 {
        let mut res = 0;
        for i in 0..*n {
            res += (a[i] - b - (i + 1) as i64).abs();
        }
        ans = ans.min(res);
    }
    ans
}
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    println!("{}", solve(&n, &a));
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use rand;
    use rand::prelude::*;

    use crate::{naive, solve};

    fn gen_input() -> (usize, Vec<i64>) {
        let n: usize = rand::random_range(1..=3);
        let a: Vec<i64> = (0..n)
            .into_iter()
            .map(|_| rand::random_range(1..=10000))
            .collect_vec();
        (n, a)
    }

    #[test]
    fn random_test() {
        for i in 0..100 {
            let (n, a) = gen_input();
            let sol = solve(&n, &a);
            let nai = naive(&n, &a);
            assert_eq!(
                sol, nai,
                "\n sol: {}, nai: {}, n: {}, a: {:?}, i: {}",
                sol, nai, n, a, i
            );
        }
    }
}
