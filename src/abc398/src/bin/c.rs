use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BTreeMap, BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        xs: [usize; n],
    }

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();

    for &x in &xs {
        *map.entry(x).or_insert(0) += 1;
    }

    let mut ans: i64 = -1;
    for i in 0..n {
        if *map.get(&xs[i]).unwrap() == 1 {
            ans = 1 + i as i64;
        }
    }
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
