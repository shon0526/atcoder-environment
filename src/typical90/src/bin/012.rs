use ac_library::Dsu;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

define_queries! {
    enum Query: usize {
        1 => Red { r: usize, c: usize},
        2 => Search {ra: usize, ca: usize, rb: usize,  cb: usize},
    }
}

const X: [isize; 4] = [1, 0, -1, 0];
const Y: [isize; 4] = [0, -1, 0, 1];

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        queries: [Query; q],
    }

    let mut grid: Vec<Vec<usize>> = vec![vec![0; w + 1]; h + 1];
    let f = |r, c| -> usize { r * w + c };

    let mut uf = Dsu::new(h * w + w + 1);

    for query in queries {
        match query {
            Query::Red { r, c } => {
                grid[r][c] = 1;

                for i in 0..4 {
                    let nx = r.wrapping_add_signed(X[i]);
                    let ny = c.wrapping_add_signed(Y[i]);

                    if nx > h || ny > w {
                        continue;
                    }

                    if grid[nx][ny] == 0 {
                        continue;
                    }

                    uf.merge(f(r, c), f(nx, ny));
                }
            }
            Query::Search { ra, ca, rb, cb } => {
                let is_ok = grid[ra][ca] == 1 && grid[rb][cb] == 1 && uf.same(f(ra, ca), f(rb, cb));
                println!("{}", if is_ok { "Yes" } else { "No" });
            }
        }
    }
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
