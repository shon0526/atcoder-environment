use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::{
    collections::{BinaryHeap, HashMap},
    process,
};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut a = a.iter().copied().collect_vec();

    let mut now = 0;

    for i in 0..n {
        if a[i] == now + 1 {
            now += 1;
        }
    }

    println!("{}", if now != 0 { (a.len() - now) as i64 } else { -1 });
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
