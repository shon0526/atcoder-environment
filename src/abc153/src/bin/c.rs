use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::{
    collections::{BinaryHeap, HashMap},
    process,
};

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [usize; n],
    }

    let mut ans = 0;
    if n <= k {
        println!("{}", ans);
        process::exit(0);
    }
    let mut h = h.iter().copied().collect_vec();
    h.sort();
    for i in 0..n {
        if i < n - k {
            ans += h[i];
        }
    }

    println!("{}", ans);
}

#[macro_export]
macro_rules! define_queries {
  ($( $(#[$attr:meta])* enum $enum_name:ident : $sig:ty { $( $pattern:pat => $variant:ident $( { $($name:ident : $marker:ty $(,)?),* } )? $(,)?),* } )*) => {
    $(
      $(#[$attr])*
      enum $enum_name {
        $(
          $variant $( {
            $( $name : <$marker as proconio::source::Readable>::Output ),*
          } )?
        ),*
      }

      impl proconio::source::Readable for $enum_name {
        type Output = Self;
        fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self {
          #![allow(unreachable_patterns)]
          match <$sig as proconio::source::Readable>::read(source) {
            $(
              $pattern => $enum_name::$variant $( {
                $( $name: <$marker as proconio::source::Readable>::read(source) ),*
              } )?
            ),*
            , _ => unreachable!()
          }
        }
      }
    )*
  }
}
