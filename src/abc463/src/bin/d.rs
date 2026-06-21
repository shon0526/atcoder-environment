use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort_by(|a, b| a.1.cmp(&b.1));

    let mut left = 0;
    let mut right = 1_000_000_000;

    while left < right {
        let mid = (left + right + 1) / 2;
        if check(&lr, mid, &k) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    println!("{}", if left == 0 { -1 } else { left as i64 });
}

fn check(lr: &Vec<(usize, usize)>, x: usize, k: &usize) -> bool {
    let mut fin = lr[0].1 as i64;
    let mut cnt = 1;

    for i in 1..lr.len() {
        let (l, r) = lr[i];
        if l as i64 - fin >= x as i64 {
            fin = r as i64;
            cnt += 1;
        }
    }
    cnt >= *k
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
