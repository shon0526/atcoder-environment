use num_traits::pow;
use proconio::{input, marker::Usize1, source::once::OnceSource};
use std::collections::{BinaryHeap, HashMap};

// 入力文字列をパースして答えの文字列を返す。
// ランダムテスト時は同コンテストの stress/naive_test.rs をこのファイル末尾に
// 貼り付けて naive と比較する(詳細はリポジトリルートの README.md)。

fn a(x1: i128, x2: i128) -> i128 {
    x2 - x1
}

fn b(y1: i128, y2: i128) -> i128 {
    y2 - y1
}

fn c(x1: i128, y1: i128, x2: i128, y2: i128) -> i128 {
    (x2 * x2) + (y2 * y2) - (x1 * x1) - (y1 * y1)
}

fn yesno(is_ok: bool) {
    println!("{}", if is_ok { "Yes" } else { "No" });
}

fn solve(input_str: &str) {
    let mut source = OnceSource::from(input_str);
    input! {
        from &mut source,
        t: usize,
        q: [(i128, i128, i128, i128, i128, i128, i128, i128); t],
    }

    for (px, py, qx, qy, rx, ry, sx, sy) in q {
        let a1 = a(px, qx);
        let b1 = b(py, qy);
        let a2 = a(rx, sx);
        let b2 = b(ry, sy);

        let c1 = c(px, py, qx, qy);
        let c2 = c(rx, ry, sx, sy);

        let denom = a1 * b2 - a2 * b1;

        let mut is_ok = false;
        if denom != 0 {
            is_ok = true;
        } else {
            let cx = b2 * c1 - b1 * c2;
            let cy = a2 * c1 - a1 * c2;

            if cx == 0 && cy == 0 {
                is_ok = true;
            }
        }
        yesno(is_ok);
    }
}

fn main() {
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap();
    solve(&buf)
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
