use ac_library::FenwickTree;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::cmp::min;
use std::{
    collections::{BinaryHeap, HashMap},
    isize,
};

const INF: usize = 1_000_002;
fn main() {
    input! {
        n: usize,
        d: i64,
        st: [(i64, i64); n],
    }

    let mut bit = FenwickTree::new(INF, 0_i64);

    for &(s, t) in &st {
        let l: i64 = s;
        let r: i64 = t - d;

        if l <= r {
            bit.add(l as usize, 1);
            bit.add(r as usize + 1, -1);
        }
    }

    let mut ans: usize = 0;

    for i in 0..1_000_001 {
        let num = bit.sum(0..=i) as usize;
        if num >= 2 {
            ans += comp(num);
        }
    }

    println!("{}", ans);
}

fn comp(x: usize) -> usize {
    x * (x - 1) / 2
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
