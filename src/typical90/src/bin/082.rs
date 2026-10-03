use num_traits::pow;
use proconio::input;
use std::collections::{BinaryHeap, HashMap};

const MOD: i128 = 1_000_000_007;

fn main() {
    input! {
        l: i128,
        r: i128,
    }

    let mut ans = 0;
    for i in 1..=19 {
        ans += cn(i, l, r);
    }

    println!("{}", ans % MOD);
}

fn cn(n: usize, l: i128, r: i128) -> i128 {
    let max = l.max(pow(10, n - 1));
    let min = r.min(pow(10, n) - 1);

    let mut count = (sigma(min) - sigma(max - 1)).rem_euclid(MOD);
    (n as i128 * count) % MOD
}

fn sigma(x: i128) -> i128 {
    (x * (x + 1) / 2) % MOD
}
