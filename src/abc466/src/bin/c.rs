use num_traits::pow;
use proconio::{
    input, input_interactive, marker::Usize1, source::line::LineSource, source::once::OnceSource,
};
use std::collections::{BinaryHeap, HashMap};
use std::io::{stdin, stdout, BufReader, Write};

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
    input_interactive! {
        n: usize,
    }

    let mut ans = 0;
    let mut r = 2;

    for l in 1..=n {
        if l == r {
            r += 1;
        }
        while r <= n {
            println!("? {} {}", l, r);
            std::io::stdout().flush().unwrap();
            // interactive
            input_interactive! {
                s: String,
            }

            if s == "Yes" {
                r += 1;
            } else {
                break;
            }
        }

        ans += r - l - 1;
    }
    println!("! {}", ans);
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
