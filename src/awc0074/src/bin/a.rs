use proconio::input;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      lt: [(usize, usize); n],
    }

    for &(l, t) in &lt {
        let mut ans = 0;
        let r = l % t;
        if r > 0 && r >= t / 2 {
            ans += l / t + 1;
        } else {
            ans += l / t;
        }
        println!("{}", ans);
    }
}
