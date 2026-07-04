use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::{
    collections::{BinaryHeap, HashMap},
    process,
};

const INF: u128 = 1_000_000_000_000_000_000;
fn main() {
    input! {
        a: u128,
        b: u128,
    }

    let ans = a * b / gcd(a, b);

    if ans > INF {
        println!("Large");
        process::exit(0);
    }
    println!("{}", ans);
}

fn gcd(a: u128, b: u128) -> u128 {
    if a % b == 0 {
        return b;
    }

    gcd(b, a % b)
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
