use ac_library::{Max, Segtree};
use indexing::algorithms::lower_bound;
use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        hl: [(usize, usize); n],
        q: usize,
        ts: [usize; q],
    }

    let mut l_vec = Vec::new();
    let mut segtree: Segtree<Max<usize>> = Segtree::new(n);

    for i in 0..n {
        let (h, l) = hl[i];
        segtree.set(i, h);
        l_vec.push(l);
    }

    for &t in &ts {
        let idx = l_vec.lower_bound(&(t + 1));
        println!("{}", segtree.prod(idx..n));
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
