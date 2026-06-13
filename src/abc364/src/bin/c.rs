use num_traits::pow;
use petgraph::algo::min_spanning_tree;
use proconio::{input, marker::Usize1};
use std::cmp::min;
use std::{
    collections::{BinaryHeap, HashMap},
    usize,
};

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort_by(|&x, &y| y.cmp(&x));
    b.sort_by(|&x, &y| y.cmp(&x));
    dbg!(&a);
    dbg!(&b);
    let mut ans = usize::MAX;

    let mut sw = 0;
    let mut count = 0;

    for i in 0..n {
        sw += a[i];
        if sw > x {
            count += 1;
            break;
        }
        count += 1;
    }

    ans = min(ans, count);

    let mut so = 0;
    let mut count = 0;

    for i in 0..n {
        so += b[i];
        if so > y {
            count += 1;
            break;
        }
        count += 1;
    }
    ans = min(ans, count);
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
