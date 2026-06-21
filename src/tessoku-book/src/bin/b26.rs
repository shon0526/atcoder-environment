use proconio::input;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
    }

    let mut p_vec = vec![true; n + 1];
    p_vec[0] = false;
    p_vec[1] = false;

    for x in 2..=n.isqrt() {
        if !p_vec[x] {
            continue;
        }

        let mut y = 2 * x;

        while y <= n {
            p_vec[y] = false;
            y += x;
        }
    }

    for i in 0..n + 1 {
        if p_vec[i] {
            println!("{}", i);
        }
    }
}
