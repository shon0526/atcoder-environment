use itertools::Itertools;
use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{BinaryHeap, HashMap};

const NUM: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
fn main() {
    input! {
        s: Chars,
    }

    let mut ans_vec = vec![];

    for i in 0..s.len() {
        if s[i].is_ascii_digit() {
            ans_vec.push(s[i]);
        }
    }

    println!(
        "{}",
        if ans_vec.len() == 0 {
            "".to_owned()
        } else {
            ans_vec.iter().join("")
        }
    );
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
