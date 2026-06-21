use proconio::input;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      q: usize,
      xs: [usize; q],
    }

    for &x in &xs {
        let mut x = x.clone();
        let sq_x = x.isqrt();
        let mut is_ok = true;

        for i in 2..=sq_x {
            if x % i == 0 {
                is_ok = false;
            }
        }
        println!("{}", if is_ok { "Yes" } else { "No" });
    }
}
