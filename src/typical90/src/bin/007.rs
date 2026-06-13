use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::cmp::min;
use std::collections::{BinaryHeap, HashMap};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        q: usize,
        bs: [i64; q],
    }

    a.sort();

    for &b in &bs {
        let idx = a.lower_bound(&b);
        let mut ans = u64::MAX;

        if idx < a.len() {
            ans = min(ans, a[idx].abs_diff(b.clone()));
        }

        if idx > 0 {
            ans = min(ans, a[idx - 1].abs_diff(b.clone()));
        }
        println!("{}", ans);
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
