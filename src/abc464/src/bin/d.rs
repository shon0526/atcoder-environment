use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::cmp::max;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
            xs: [i64; n],
            ys: [i64; n-1],
        }

        let mut dp = vec![vec![0; 2]; n];
        if s[0] == 'S' {
            dp[0][0] = 0;
            dp[0][1] = -xs[0];
        } else {
            dp[0][0] = -xs[0];
            dp[0][1] = 0;
        }

        for i in 0..n - 1 {
            dp[i + 1][0] = max(dp[i][0], dp[i][1] + ys[i]);
            dp[i + 1][1] = max(dp[i][0], dp[i][1]);
            if s[i + 1] == 'S' {
                dp[i + 1][1] -= xs[i + 1];
            } else {
                dp[i + 1][0] -= xs[i + 1];
            }
        }

        println!("{}", max(dp[n - 1][0], dp[n - 1][1]));
    }
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
