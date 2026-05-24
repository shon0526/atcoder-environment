use proconio::input;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      k: usize,
      fb: [(i64, i64); n],
    }

    let mut tol = fb.iter().map(|(af, b)| b).sum::<i64>();

    let mut cad_vec = Vec::new();
    for (i, (f, b)) in fb.iter().enumerate() {
        let diff = f - b;
        cad_vec.push((diff, i));
    }

    cad_vec.sort();
    for i in k..cad_vec.len() {
        tol += cad_vec[i].0;
    }
    println!("{}", tol);
}
