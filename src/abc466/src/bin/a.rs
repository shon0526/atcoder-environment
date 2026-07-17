use num_traits::pow;
use proconio::{input, marker::Usize1, source::once::OnceSource};
use std::collections::{BinaryHeap, HashMap};

// 入力文字列をパースして答えの文字列を返す。
// ランダムテスト時は同コンテストの stress/naive_test.rs をこのファイル末尾に
// 貼り付けて naive と比較する(詳細はリポジトリルートの README.md)。
fn solve(input_str: &str) -> String {
    let mut source = OnceSource::from(input_str);
    input! {
        from &mut source,
        n: usize,
        a: [usize; n],
    }
    todo!()
}

fn main() {
    input! {
        n: usize,
        xs: [i64; n],
    }

    let mut is_ok = true;

    for x in xs {
        if x >= 0 {
            is_ok = false;
        }
    }

    println!("{}", if is_ok { "Yes" } else { "No" });
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
