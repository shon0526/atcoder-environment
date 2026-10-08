use itertools::Itertools;
use num_traits::pow;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
    source::once::OnceSource,
};
use std::collections::{BTreeSet, BinaryHeap, HashMap};

// https://atcoder.jp/contests/abc447/tasks/abc447_d
// あるAに対してそれより右側にある最短のBとCを見つける

fn solve(input_str: &str) {
    let mut source = OnceSource::from(input_str);
    input! {
        from &mut source,
        s: Bytes,
    }

    let mut n = s.len();
    let mut ans = 0;
    let mut set_a = s
        .iter()
        .copied()
        .positions(|c| c == b'A')
        .collect::<BTreeSet<_>>();

    let mut set_c = s
        .iter()
        .copied()
        .positions(|c| c == b'C')
        .collect::<BTreeSet<_>>();

    for i in 0..n {
        if s[i] == b'B' {
            let mut a_min = set_a.range(..i).min().copied();
            let mut c_min = set_c.range(i..).min().copied();

            if let Some(a) = a_min {
                if let Some(c) = c_min {
                    set_a.remove(&a);
                    set_c.remove(&c);
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}

fn main() {
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap();
    solve(&buf)
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
