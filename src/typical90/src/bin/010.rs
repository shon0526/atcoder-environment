use ac_library::FenwickTree;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(Usize1, Usize1); q]
    }

    let mut bit_1 = FenwickTree::new(n, 0);
    let mut bit_2 = FenwickTree::new(n, 0);

    for (i, &(c, p)) in cp.iter().enumerate() {
        if c == 1 {
            bit_1.add(i, p);
        } else {
            bit_2.add(i, p);
        }
    }

    for &(l, r) in &lr {
        let first = bit_1.sum(l..=r);
        let second = bit_2.sum(l..=r);
        println!("{} {}", first, second);
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
