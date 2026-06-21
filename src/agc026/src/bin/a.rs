use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let a = a.iter().copied().collect_vec();
    let a_rlc = rlc(&a);

    let mut ans = 0;

    for &(_, c) in &a_rlc {
        if c >= 2 {
            ans += c / 2;
        }
    }

    println!("{}", ans);
}

fn rlc(vec: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut rlc_vec: Vec<(usize, usize)> = vec![];
    let mut pre = vec[0];
    let mut cnt = 1;

    for i in 1..vec.len() {
        if pre == vec[i] {
            cnt += 1;
        } else {
            rlc_vec.push((pre, cnt));
            pre = vec[i];
            cnt = 1;
        }
    }

    rlc_vec.push((pre, cnt));
    rlc_vec
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
