use itertools::Itertools;
use num_traits::pow;
use pathfinding::grid;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut grid = vec![vec![]; n];

    for (a, b) in ab {
        grid[a].push(b);
        grid[b].push(a);
    }

    let mut seen = vec![false; n];
    let mut is_ok = vec![false; n];

    dfs(&grid, &mut seen, &mut is_ok, 0);

    let mut ans = 0;
    for i in 0..n {
        if is_ok[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn dfs(grid: &Vec<Vec<usize>>, seen: &mut Vec<bool>, is_ok: &mut Vec<bool>, now: usize) {
    seen[now] = true;

    let mut cnt = 0;
    for &nx in &grid[now] {
        if now > nx {
            cnt += 1;
        }
        if seen[nx] {
            continue;
        }
        dfs(grid, seen, is_ok, nx);
    }
    if cnt == 1 {
        is_ok[now] = true;
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
