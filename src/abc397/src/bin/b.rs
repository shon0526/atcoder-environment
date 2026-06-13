use num_traits::pow;
use proconio::{input, marker::Chars};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;

    if s[0] == 'o' {
        ans += 1;
    }

    for i in 0..s.len() - 1 {
        if s[i] == s[i + 1] {
            ans += 1;
        }
    }

    if s[s.len() - 1] == 'i' {
        ans += 1;
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
