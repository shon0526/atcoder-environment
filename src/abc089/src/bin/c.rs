use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let mut map: HashMap<char, usize> = HashMap::new();
    let mut bk: Vec<usize> = vec![0; 5];

    map.insert('M', 0);
    map.insert('A', 1);
    map.insert('R', 2);
    map.insert('C', 3);
    map.insert('H', 4);

    for s in &ss {
        if let Some(&idx) = map.get(&s[0]) {
            bk[idx] += 1;
        }
    }

    let mut ans: usize = 0;

    for i in 0..3 {
        for j in i + 1..4 {
            for k in j + 1..5 {
                ans += bk[i] * bk[j] * bk[k];
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
