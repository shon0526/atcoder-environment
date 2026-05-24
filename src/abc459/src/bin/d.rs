use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::HashMap;
fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      t: usize,
    }

    for _ in 0..t {
        let mut map = HashMap::new();
        input! {
            s: Chars,
        }
        for c in &s {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut cad_max = 0;
        let mut kind = Vec::new();
        for (key, v) in &map {
            if *v > cad_max {
                cad_max = *v;
            }
            kind.push(*key);
        }
        if cad_max > ((s.len() + 1) / 2) {
            println!("No");
            continue;
        }
        let mut kind_cnt = kind.len();
        let mut ans_vec = Vec::new();
        let mut idx = 0;
        while kind_cnt > 0 {
            let now_idx = idx % kind.len();
            let cnt = *map.get(&kind[now_idx]).unwrap();
            if cnt > 0 {
                ans_vec.push(*kind[now_idx]);
                *map.get_mut(&kind[now_idx]).unwrap() -= 1;
                if *map.get(&kind[now_idx]).unwrap() == 0 {
                    kind_cnt -= 1;
                }
            }
            idx += 1;
        }
        println!("Yes");
        println!("{}", ans_vec.iter().join(""));
    }
}

