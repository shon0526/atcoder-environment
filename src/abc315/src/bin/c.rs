use glidesort::sort_by;
use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::cmp::max;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        fs: [(usize, usize); n],
    }

    let mut bk = vec![vec![]; n + 1];

    for &(f, s) in &fs {
        bk[f].push(s);
    }

    let mut best = vec![];
    let mut ans = 0;

    for i in 0..n + 1 {
        if bk[i].len() == 0 {
            continue;
        }
        bk[i].sort_by(|a, b| b.cmp(&a));
        if bk[i].len() >= 2 {
            ans = max(ans, bk[i][0] + bk[i][1] / 2);
        }

        best.push(bk[i][0]);
    }

    best.sort_by(|a, b| b.cmp(&a));
    if best.len() >= 2 {
        ans = max(ans, best[0] + best[1]);
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
