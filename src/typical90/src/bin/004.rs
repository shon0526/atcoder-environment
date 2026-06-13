use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut line: Vec<usize> = vec![];
    let mut row: Vec<usize> = vec![];

    for i in 0..h {
        let mut tol = 0;
        for j in 0..w {
            tol += a[i][j];
        }
        line.push(tol);
    }

    for j in 0..w {
        let mut tol = 0;
        for i in 0..h {
            tol += a[i][j];
        }
        row.push(tol);
    }

    let mut ans_vec = vec![vec![]; h];

    for i in 0..h {
        for j in 0..w {
            ans_vec[i].push(line[i] + row[j] - a[i][j]);
        }
    }

    for i in 0..h {
        println!("{}", ans_vec[i].iter().join(" "));
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
