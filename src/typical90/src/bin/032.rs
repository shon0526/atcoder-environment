use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::cmp::min;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut ps = (0..n).permutations(n).collect_vec();
    let mut ans = usize::MAX;

    let mut set = HashSet::new();
    for &(x, y) in &xy {
        set.insert((x, y));
    }

    for p in ps {
        let mut res = a[p[0]][0];
        let mut is_ok = true;

        for i in 1..n {
            let pre = p[i - 1];
            let now = p[i];
            if set.contains(&(pre, now)) || set.contains(&(now, pre)) {
                is_ok = false;
                continue;
            }
            res += a[p[i]][i];
        }
        if is_ok {
            ans = min(ans, res);
        }
    }

    println!("{}", if ans != usize::MAX { ans as i64 } else { -1 });
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
