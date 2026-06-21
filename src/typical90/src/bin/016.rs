use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::cmp::min;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let mut ans = i64::MAX;

    for x in 0..10000 {
        for y in 0..10000 {
            if x + y >= 10000 {
                continue;
            }

            let res: i64 = n - a * x - b * y;
            if res >= 0 && res % c == 0 {
                let z = res / c;
                ans = min(ans, x + y + z);
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
