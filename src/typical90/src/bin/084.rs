use itertools::Itertools;
use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let s = s.chars().collect_vec();
    let mut ans = 0;

    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];

    for i in 1..n + 1 {
        if s[i - 1] == 'o' {
            a[i] = i;
            b[i] = b[i - 1];
        } else {
            b[i] = i;
            a[i] = a[i - 1];
        }
    }

    for i in 1..n + 1 {
        ans += a[i].min(b[i]);
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
