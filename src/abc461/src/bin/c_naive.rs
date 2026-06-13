use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

// abc461 C の愚直解。
// N 個から K 個を全探索し、異なる色が M 種類以上になる選び方の中で
// 価値合計の最大値を求める（小さい N でのみ現実的）。
fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        cv: [(Usize1, u128); n],
    }

    let mut ans: u128 = 0;
    // K 個のインデックスの組み合わせを総当たりする。
    for combo in (0..n).combinations(k) {
        let colors: HashSet<usize> = combo.iter().map(|&i| cv[i].0).collect();
        if colors.len() >= m {
            let sum: u128 = combo.iter().map(|&i| cv[i].1).sum();
            ans = ans.max(sum);
        }
    }

    println!("{}", ans);
}
