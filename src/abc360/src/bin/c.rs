use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        w: [usize; n],
    }

    let mut heap = BinaryHeap::new();
    let mut ans = 0;

    let mut a = a
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .into_iter()
        .collect_vec();

    a.sort();
    let mut now = 0;
    for i in 0..n {
        if a[i].0 != now {
            if heap.len() <= 1 {
                heap.clear();
                heap.push(Reverse(w[a[i].1]));
                now = a[i].0;
            } else {
                while let Some((Reverse(weight))) = heap.pop() {
                    ans += weight;
                    if heap.len() == 1 {
                        break;
                    }
                }
                heap.clear();
                heap.push(Reverse(w[a[i].1]));
                now = a[i].0
            }
        } else {
            heap.push(Reverse(w[a[i].1]));
        }
    }

    if heap.len() > 1 {
        while let Some(Reverse(v)) = heap.pop() {
            ans += v;
            if heap.len() == 1 {
                break;
            }
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

