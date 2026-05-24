use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;
const INF: usize = 10_000_000;
fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      q: usize,
    }
    let mut map = HashMap::new();
    for i in 1..n + 1 {
        map.insert(i, 0);
    }

    let mut prefix = vec![0; INF];
    let mut y_up = 0;
    for _ in 0..q {
        input! {
            num: usize,
        }

        if num == 1 {
            input! {
                x: usize,
            }

            *map.get_mut(&x).unwrap() += 1;
            let mut idx = *map.get(&x).unwrap();
            if prefix[idx] == n - 1 {
                prefix[idx] += 1;
                y_up += 1;
            } else {
                prefix[idx] += 1;
            }
        } else if num == 2 {
            input! {
                y: usize,
            }
            println!("{}", prefix[y + y_up]);
        }
    }
}
