use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
    }

    let mut heap = BinaryHeap::new();

    for (a, b) in ab {
        heap.push(Reverse((a, b)));
    }

    let mut cnt = 0;
    let mut ans = 0;
    while cnt < m {
        if let Some(Reverse((a, b))) = heap.pop() {
            if cnt + b <= m {
                ans += a * b;
                cnt += b;
            } else {
                ans += a * (m - cnt);
                cnt = m;
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
