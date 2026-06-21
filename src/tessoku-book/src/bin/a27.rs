use proconio::input;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      a: usize,
      b: usize,
    }

    let ans = gcd(a, b);

    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    let rem = a % b;
    if rem == 0 {
        b
    } else {
        gcd(b, rem)
    }
}
