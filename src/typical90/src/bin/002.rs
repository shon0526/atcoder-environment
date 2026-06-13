use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};
fn main() {
    input! {
        n: usize,
    }

    let mut ans_vec = Vec::new();

    for bit in 0..(1 << n) {
        let mut s = Vec::new();
        for i in (0..n).into_iter().rev() {
            if bit & (1 << i) != 0 {
                s.push(')');
            } else {
                s.push('(');
            }
        }

        println!("{:?}", s);

        if valid(&s) {
            let s = s.iter().join("");
            ans_vec.push(s);
        }
    }
    for ans in ans_vec {
        println!("{}", ans);
    }
}

fn valid(vec: &Vec<char>) -> bool {
    let mut tol: i64 = 0;

    for &c in vec {
        if c == '(' {
            tol += 1;
        } else {
            tol -= 1;
        }

        if tol < 0 {
            break;
        }
    }

    tol == 0
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
