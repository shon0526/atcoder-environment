use proconio::input;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    println!("{}", if a != b && b == c { "Yes" } else { "No" });
}
