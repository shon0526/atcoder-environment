use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        lrs: [(usize, usize, Usize1, Usize1); n],
    }

    let mut imos: Vec<Vec<i64>> = vec![vec![0; 1002]; 1002];

    for &(lx, ly, rx, ry) in &lrs {
        imos[lx][ly] += 1;
        imos[lx][ry + 1] -= 1;
        imos[rx + 1][ly] -= 1;
        imos[rx + 1][ry + 1] += 1;
    }

    for i in 0..1001 {
        for j in 0..1001 {
            imos[i][j + 1] += imos[i][j];
        }
    }

    for j in 0..1001 {
        for i in 0..1001 {
            imos[i + 1][j] += imos[i][j];
        }
    }

    let mut ans_vec = vec![0; n + 1];

    for i in 0..1002 {
        for j in 0..1002 {
            if imos[i][j] > 0 {
                ans_vec[imos[i][j] as usize] += 1;
            }
        }
    }

    for i in 1..n + 1 {
        println!("{}", ans_vec[i]);
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
