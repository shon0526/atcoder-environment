use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{BinaryHeap, HashMap};

const WAY: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// 制約が H x w <= 16 なので全ての経路で探索が可能
// バックトラックを使う
//  - 次いけるますそれぞれに対して再起呼び出しをする
//  - 行き止まりに当たったか、次行ける全部の方向についてしらば終わったら、一手戻る

fn main() {
    input! {
        h: usize,
        w: usize,
        cs: [Chars; h],
    }

    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if cs[i][j] == '#' {
                continue;
            }
            let mut seen = vec![vec![false; w]; h];
            let mut cad_vec: Vec<i64> = Vec::new();
            dfs(&cs, &mut seen, &mut cad_vec, i, j, (i, j), 1, h, w);
            //            println!("{:?}", cad_vec);
            if cad_vec.len() > 0 {
                ans = ans.max(*cad_vec.iter().max().unwrap());
            }
        }
    }

    println!("{}", if ans == 0 { -1 } else { ans });
}

fn dfs(
    graph: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    cad_vec: &mut Vec<i64>,
    u: usize,
    v: usize,
    st: (usize, usize),
    mut count: i64,
    h: usize,
    w: usize,
) {
    seen[u][v] = true;
    for &(x, y) in &WAY {
        let nx = u.wrapping_add_signed(x);
        let ny = v.wrapping_add_signed(y);

        if nx == usize::MAX || nx > h - 1 || ny == usize::MAX || ny > w - 1 {
            continue;
        }
        if graph[nx][ny] == '#' {
            continue;
        }
        if (nx as usize, ny as usize) == st {
            if count >= 4 {
                cad_vec.push(count);
            }
            continue;
        }
        if seen[nx][ny] {
            continue;
        }
        //       println!("u: {}, v: {} ", u, v);
        dfs(graph, seen, cad_vec, nx, ny, st, count + 1, h, w);
        seen[nx][ny] = false;
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
