use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap, HashSet};

// https://atcoder.jp/contests/typical90/editorial
// 発電所の座標を(a, b)で置いておく
// |x_1 - a| + |y_1 - b| + |x_2 - a| + |y_2 - b| + ...
// xとyを独立に考えることができる -> (|x_1 - a | * |x_2 - a| + ...) + (|y_1 - b| + |y_2 - b| + ...)
//
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut xs = xy.iter().map(|(x, _)| x).collect_vec();
    let mut ys = xy.iter().map(|(_, y)| y).collect_vec();

    xs.sort();
    ys.sort();

    let mx = xs[n / 2];
    let my = ys[n / 2];

    let mut ans = 0;
    for &(x, y) in &xy {
        ans += (x - mx).abs() + (y - my).abs();
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
