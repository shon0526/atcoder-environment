use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let top = t[2];
    let s_len = s.len();
    let t_len = t.len();

    let mut point = 0;
    let mut is_ok = false;
    match top {
        'X' => {
            for i in 0..s_len {
                let s_up = s[i].to_ascii_uppercase();
                if s_up == t[point] {
                    point += 1;
                }

                if point == 2 {
                    break;
                }
            }
            is_ok = point == 2;
        }
        _ => {
            for i in 0..s_len {
                let s_up = s[i].to_ascii_uppercase();
                if s_up == t[point] {
                    point += 1;
                }

                if point == 3 {
                    break;
                }
            }
            is_ok = point == 3;
        }
    }

    println!("{}", if is_ok { "Yes" } else { "No" })
}

fn check(s: &Vec<char>, t: &Vec<char>) -> bool {
    let mut is_ok = true;
    for i in 0..2 {
        if s[i] != t[i] {
            is_ok = false;
        }
    }

    is_ok
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
