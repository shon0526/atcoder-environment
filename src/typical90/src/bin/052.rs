use ac_library::{ModInt1000000007, StaticModInt};
use az::UnwrappedAs;
use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

type ModInt = ModInt1000000007;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }

    let a = a
        .iter()
        .map(|a| {
            let mut vec = Vec::new();
            for i in 0..6 {
                vec.push(ModInt1000000007::new(a[i]));
            }
            vec
        })
        .collect_vec();
    let ans = a
        .iter()
        .map(|a| a.iter().sum::<ModInt>())
        .product::<ModInt>();
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
