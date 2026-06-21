use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};
use std::f64::consts::PI;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        es: [f64; q],
    }

    for e in es {
        println!("{}", solve(t, l, x, y, e));
    }
}

fn solve(t: f64, l: f64, x: f64, y: f64, e: f64) -> f64 {
    let theta = (e / t) * 2.0 * PI;
    let cy = -l / 2.0 * theta.sin();
    let cz = l / 2.0 - l / 2.0 * theta.cos();
    let d = (x * x + (y - cy) * (y - cy)).sqrt();
    let ans_theta = (cz / d).atan() * 180.0 / PI;
    ans_theta
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
