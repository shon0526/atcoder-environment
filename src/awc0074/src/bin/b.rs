use ac_library::{LazySegtree, LazySegtree::MAX};
use proconio::input;

use std::cmp::max;
fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      a: [i64; n],
    }

    let segtree = LazySegtree::from(a);
}
