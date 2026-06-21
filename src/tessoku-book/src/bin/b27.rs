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

    let mut vec = vec![a, b];
    vec.sort();
    let a = vec[1];
    let b = vec[0];
    let gcd = gcd(&a, &b);

    let ans = a * b / gcd;
    println!("{}", ans);
}

fn gcd(a: &usize, b: &usize) -> usize {
    let rem = a % b;
    if rem == 0 {
        *b
    } else {
        gcd(b, &rem)
    }
}
