use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::cmp::max;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        h: usize,
        w: usize,
        ps: [[usize; w]; h],
    }

    let mut grid = ps.iter().clone().collect_vec();
    let mut ans: usize = 0;

    for bit in 1..(1 << h) {
        let mut cnt_vec = vec![0 as usize; h * w + 1];
        let mut rs: Vec<usize> = vec![];
        let mut row_count = 0;
        for i in 0..h {
            if bit & (1 << i) != 0 {
                row_count += 1;
            }
        }
        for j in 0..w {
            let mut cad_vec: Vec<usize> = vec![];
            let mut is_ok = true;
            for i in 0..h {
                if bit & (1 << i) != 0 {
                    if !cad_vec.is_empty() && cad_vec[cad_vec.len() - 1] != grid[i][j] {
                        is_ok = false;
                        break;
                    } else {
                        cad_vec.push(grid[i][j]);
                    }
                }
            }
            if is_ok && !cad_vec.is_empty() {
                rs.push(cad_vec.pop().unwrap());
            }
        }
        for r in &rs {
            cnt_vec[*r] += 1;
        }

        ans = max(ans, cnt_vec.iter().copied().max().unwrap() * row_count);
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
