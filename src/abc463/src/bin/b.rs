use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        x: char,
        s: [Chars; n],
    }

    let x = x.to_ascii_lowercase();
    let row = x as usize - 'a' as usize;
    let mut ans = false;

    for i in 0..n {
        if s[i][row] == 'o' {
            ans = true;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
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
