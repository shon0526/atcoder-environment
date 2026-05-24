use proconio::input;
use std::collections::HashSet;
fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      k: usize,
      m: usize,
      a: [usize; k],
      b: [usize; m],
    }

    let mut set = HashSet::new();
    for i in 0..k {
        set.insert(a[i]);
    }

    let mut count = 0;

    for i in 0..m {
        if set.contains(&b[i]) {
            count += 1;
        }
    }

    println!("{}", count);
}
