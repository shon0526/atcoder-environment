use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        cv: [(Usize1, u128); n],
    }

    let mut vec = vec![vec![]; n];
    for &(c, v) in &cv {
        vec[c].push(v);
    }

    let mut top = vec![];
    let mut res = vec![];
    let mut ans = 0;
    for i in 0..n {
        if vec[i].len() == 0 {
            continue;
        }
        vec[i].sort_by(|a, b| b.cmp(&a));
        for j in 0..vec[i].len() {
            if j == 0 {
                top.push(vec[i][j]);
            } else {
                res.push(vec[i][j]);
            }
        }
    }

    top.sort_by(|a, b| b.cmp(&a));
    for i in 0..top.len() {
        if i < m {
            ans += top[i];
        } else {
            res.push(top[i]);
        }
    }

    res.sort_by(|a, b| b.cmp(&a));

    for i in 0..k - m {
        ans += res[i];
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
