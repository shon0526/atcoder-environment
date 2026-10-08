use ac_library::{Mod1000000007, ModInt1000000007, StaticModInt};
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

// 1番目はk種類の中から好きな色を選べる
// ２番目はk - 1種類の中から好きな色を選べる
// ３番目以降は i番目を基準とした時に i-2, i-1番目と被ってはいけないので、k-2種類の中から好きな色を選ぶ
//
fn solve(n: usize, k: usize) -> StaticModInt<Mod1000000007> {
    let mut ans = ModInt1000000007::new(1);

    ans = ans * ModInt1000000007::new(k);
    if n > 1 {
        ans = ans * ModInt1000000007::new(k - 1);
    }

    if n > 2 && k > 1 {
        ans = ans * mod_pow(k - 2, n - 2);
    }

    ans
}

fn naive(n: usize, k: usize) -> StaticModInt<Mod1000000007> {
    let mut ans = ModInt1000000007::new(1);
    ans = ans * ModInt1000000007::new(k);
    if n > 1 {
        ans = ans * ModInt1000000007::new(k - 1);
    }

    if n > 2 && k > 1 {
        for _ in 0..n - 2 {
            ans = ans * ModInt1000000007::new(k - 2);
        }
    }
    ans
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = ModInt1000000007::new(1);

    ans = ans * ModInt1000000007::new(k);
    if n > 1 {
        ans = ans * ModInt1000000007::new(k - 1);
    }

    if n > 2 && k > 1 {
        ans = ans * mod_pow(k - 2, n - 2);
    }

    println!("{}", ans);
}

// 繰り返し２乗法

fn mod_pow(x: usize, y: usize) -> StaticModInt<Mod1000000007> {
    let mut ret = ModInt1000000007::new(1);
    let mut mul: StaticModInt<Mod1000000007> = ModInt1000000007::new(x);

    for i in 0..60 {
        if 1 << i & y > 0 {
            ret = ret * mul;
        }
        mul = (mul * mul);
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    fn gen_input(rng: &mut StdRng) -> (usize, usize) {
        let n = rng.random_range(1..=10000);
        let k = rng.random_range(1..=10000000);
        (n, k)
    }

    #[test]
    fn random_test() {
        let itertions = 1000;
        for seed in 0..itertions {
            let mut rng = StdRng::seed_from_u64(seed);
            let (n, k) = gen_input(&mut rng);
            let expected = solve(n, k);
            let actual = naive(n, k);

            assert_eq!(
                expected, actual,
                "\n expected: {}, actual: {}, n: {}, k: {}",
                expected, actual, n, k
            );
        }
    }
}
