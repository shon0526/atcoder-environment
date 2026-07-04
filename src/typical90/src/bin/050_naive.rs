use ac_library::ModInt1000000007;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        l: usize,
    }

    let mut stairs = vec![ModInt1000000007::new(0); n + 1];

    stairs[1] += ModInt1000000007::new(1);
    if l <= n {
        stairs[l] += ModInt1000000007::new(1);
    }

    for i in 1..n {
        stairs[i + 1] = stairs[i + 1] + stairs[i];
        if i + l <= n {
            stairs[i + l] = stairs[i + l] + stairs[i];
        }
    }
    println!("{}", stairs[n]);
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
