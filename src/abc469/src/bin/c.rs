use itertools::Itertools;
use num_traits::pow;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
    source::once::OnceSource,
};
use std::collections::{BinaryHeap, HashMap};

// 入力文字列をパースして答えの文字列を返す。
// ランダムテスト時は同コンテストの stress/naive_test.rs をこのファイル末尾に
// 貼り付けて naive と比較する(詳細はリポジトリルートの README.md)。
fn solve(input_str: &str) {
    let mut source = OnceSource::from(input_str);
    input! {
        from &mut source,
        n: usize,
        s: Bytes,
    }

    let mut cur = 0;
    let mut ans_vec = vec![0; n];

    for k in 0..n {
        cur = (cur + 1).min(n);

        loop {
            if cur == n {
                break;
            }
            let cur_char = s[cur - 1];
            if cur_char == b'x' {
                break;
            } else {
                cur += 1;
            }
        }

        ans_vec[k] = cur;
    }

    for i in 0..n {
        println!("{}", ans_vec[i]);
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
