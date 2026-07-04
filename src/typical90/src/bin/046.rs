use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let bk = |vec: &Vec<usize>| {
        let mut vector: Vec<usize> = vec![0; 46];
        for &v in vec {
            vector[v % 46] += 1;
        }
        vector
    };

    let a = bk(&a);
    let b = bk(&b);
    let c = bk(&c);
    let mut ans = 0;

    for (i, &va) in a.iter().enumerate() {
        for (j, &vb) in b.iter().enumerate() {
            for (k, &vc) in c.iter().enumerate() {
                if (i + j + k) % 46 == 0 {
                    ans += va * vb * vc;
                }
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
