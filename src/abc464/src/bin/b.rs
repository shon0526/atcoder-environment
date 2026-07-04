use itertools::Itertools;
use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut top = h - 1;
    let mut buttom = 0;
    let mut left = w - 1;
    let mut right = 0;

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                top = min(top, i);
                buttom = max(buttom, i);
            }
        }
    }

    for j in 0..w {
        for i in 0..h {
            if c[i][j] == '#' {
                left = min(left, j);
                right = max(right, j);
            }
        }
    }

    for i in top..=buttom {
        println!("{}", (left..=right).into_iter().map(|j| c[i][j]).join(""));
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
