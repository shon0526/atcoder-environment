use ac_library::ModInt1000000007;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut br = vec![0; n + 1];
    for i in 0..m {
        br[a[i]] = 1;
    }

    let mut dp = vec![ModInt1000000007::new(0); n + 1];
    dp[0] = ModInt1000000007::new(1);

    for i in 0..n {
        if br[i] == 1 {
            continue;
        }

        if br[i + 1] != 1 {
            dp[i + 1] = dp[i + 1] + dp[i];
        }
        if i + 2 <= n && br[i + 2] != 1 {
            dp[i + 2] = dp[i + 2] + dp[i];
        }
    }

    println!("{}", dp[n]);
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
