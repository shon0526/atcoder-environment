use itertools::Itertools;
use num_traits::{pow, Pow};
use proconio::{
    input,
    marker::{Bytes, Usize1},
};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: u128,
        k: u128,
    }

    let mut ans = n.to_string();
    for _ in 0..k {
        ans = eight_to_nine(&ans);
    }
    println!("{}", ans)
}

fn eight_to_nine(x: &String) -> String {
    let mut ten: u128 = 0;
    let mut len = x.len();

    // 8進数を10進数に
    for (i, c) in x.chars().enumerate() {
        ten += pow(8, len - 1 - i) * (c.to_string().parse::<u128>().unwrap());
    }

    //    dbg!(&ten);

    let mut nine: Vec<u128> = nine_digit(ten);
    //    dbg!(&nine);
    if nine.len() > 1 && nine[nine.len() - 1] == 0 {
        nine.pop();
    }

    let nine = nine
        .iter()
        .rev()
        .map(|&num| {
            if num == 8 {
                "5".to_string()
            } else {
                num.to_string()
            }
        })
        .collect::<String>();
    //   dbg!(&nine);
    if nine.len() > 0 {
        nine
    } else {
        0.to_string()
    }
}

fn nine_digit(num: u128) -> Vec<u128> {
    let mut vec = Vec::new();
    let mut x = num.clone();

    while x > 0 {
        vec.push(x % 9);
        x /= 9;
    }
    vec
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
