use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;
use superslice::Ext;
fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      d: usize,
      mut xs: [(usize, usize); n],
    }

    let mut ans = 0;
    xs.sort();
    let mut x_vec = xs.iter().map(|(x, _)| *x).collect_vec();
    let mut s_vec = xs.iter().map(|(_, s)| *s).collect_vec();
    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + s_vec[i];
    }

    let mut ans = 0;
    let mut left = 0;
    for right in 0..n {
        while (left < right && (x_vec[right]).abs_diff(x_vec[left]) > d) {
            left += 1;
        }
        ans += (prefix_sum[right] - prefix_sum[left]) * s_vec[right];
    }
    println!("{}", ans);
}
