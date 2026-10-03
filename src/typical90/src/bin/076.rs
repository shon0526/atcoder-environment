use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

//円環なのでa = [1, 2, 3]のとき, a = [1, 2, 3, 1, 2, 3]とする

fn solve(n: usize, a: Vec<usize>) {
    let mut b = vec![0; 2 * n + 1];

    for i in 1..n + 1 {
        b[i] = b[i - 1] + a[i - 1];
    }
    for i in 1..n + 1 {
        b[i + n] = b[i + n - 1] + a[i - 1];
    }

    if b[n] % 10 != 0 {
        println!("No");
        return;
    }

    let target = b[n] / 10;
    let mut is_ok = false;
    for i in 0..=n {
        let goal = b[i] + target;
        // goalより小さい値の中での最大値に対応するインデックスを二分探索で探す
        let idx = b.partition_point(|&v| v < goal);
        if idx - i <= n && b[idx] == goal {
            is_ok = true;
            break;
        }
    }

    println!("{}", if is_ok { "Yes" } else { "No" });

    // let mut vec = a.clone();
    // vec.extend_from_slice(&a);
    // let mut is_ok = false;
    // let mut check = a.iter().sum::<usize>();
    // let mut tol = 0;
    // let mut r = 0;
    //
    // for l in 0..n {
    //     tol += a[l];
    //     while (r - l) < n && (tol + vec[l]) * 10 <= check {
    //         tol += vec[r];
    //         r += 1;
    //     }
    //     if tol * 10 == check {
    //         is_ok = true;
    //     }
    //     if l == r {
    //         r += 1;
    //     }
    //     tol -= vec[l];
    // }
    // if is_ok {
    //     "Yes".to_string()
    // } else {
    //     "No".to_string()
    // }
}

fn naive(n: usize, a: Vec<usize>) -> String {
    let mut is_ok = false;
    let mut check = a.iter().sum::<usize>();

    for i in 0..n {
        let mut tol = 0;
        for j in 0..n {
            tol += a[(i + j) % n];
            if tol * 10 == check {
                is_ok = true;
                break;
            }
        }
        if is_ok {
            break;
        }
    }
    if is_ok {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    solve(n, a);
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use rand;
    use rand::prelude::*;

    use crate::{naive, solve};

    fn gen_input() -> (usize, Vec<usize>) {
        let n = rand::random_range(1..=3);
        let a = (0..n).map(|_| rand::random_range(1..=10)).collect_vec();

        (n, a)
    }

    #[test]
    fn random_test() {
        for _ in 0..100 {
            let (n, a) = gen_input();
            eprintln!("trying n={} a={:?}", n, a);

            let sol = solve(n, a.clone());
            let nai = naive(n, a.clone());

            assert_eq!(
                sol, nai,
                "n: {}, a: {:?} \n sol:{}, nai: {}",
                n, a, sol, nai
            );
        }
    }
}
