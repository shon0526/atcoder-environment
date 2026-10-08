use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

// aの候補は、1からk^{1/3}個ある
// 各aに対してb, cを求める -> b * c = k / a ( a <= b <= c)を求める
// o(k^{2/3})かな

fn main() {
    input! {
        k: usize,
    }

    let mut ans = 0;
    for a in (1..=10000).filter(|&x| x * x * x <= k) {
        if k % a != 0 {
            continue;
        }

        let div = k / a;

        for b in 1..=div.isqrt() {
            if div % b == 0 && a <= b {
                ans += 1;
            }
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
