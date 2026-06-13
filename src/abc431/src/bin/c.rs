use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        hs: [usize; n],
        bs: [usize; m],
    }

    let mut hs = hs.iter().copied().collect_vec();
    let mut bs = bs.iter().copied().collect_vec();
    hs.sort();
    bs.sort();
    let mut cnt = 0;
    let mut x = 0;
    for i in 0..n {
        while x < m && hs[i] > bs[x] {
            x += 1;
        }

        if x < m {
            cnt += 1;
            x += 1;
        }
    }
    println!("{}", if cnt >= k { "Yes" } else { "No" });
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
