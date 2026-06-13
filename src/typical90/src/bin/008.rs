use ac_library::ModInt1000000007;
use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{BinaryHeap, HashMap};
const T: [char; 7] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![vec![ModInt1000000007::new(0); 8]; n + 1];
    dp[0][0] = ModInt1000000007::new(1);

    for i in 0..n {
        for j in 0..8 {
            let current = dp[i][j].clone();
            if j < 7 && s[i] == T[j] {
                dp[i + 1][j + 1] += current;
            }
            dp[i + 1][j] += current;
        }
    }

    println!("{}", dp[n][7]);
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
