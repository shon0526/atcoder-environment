use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    }

    let mut a = a.iter().copied().collect_vec();
    a.push(l);
    let mut left = 0;
    let mut right = a.iter().sum::<usize>();

    while left < right {
        let mid = (left + right + 1) / 2;

        //println!("mid: {}", mid);
        if check(&a, mid) >= k + 1 {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    println!("{}", left);
}

fn check(vec: &Vec<usize>, x: usize) -> usize {
    let mut count = 0;
    let mut pre = 0;

    for i in 0..vec.len() {
        if vec[i] - pre >= x {
            //println!("{}", vec[i]);
            count += 1;
            pre = vec[i];
        }
    }

    count
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
