use num_traits::pow;
use proconio::{input, marker::Usize1, source::once::OnceSource};
use std::cmp::min;
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

const INF: usize = usize::MAX;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n-1],
    }

    let mut dp = vec![vec![INF; 2]; n];

    if a[0] == 0 {
        dp[0][0] = 0;
        dp[0][1] = 1;
    } else {
        dp[0][0] = 1;
        dp[0][1] = 0;
    }

    for i in 0..n - 1 {
        if b[i] == 0 {
            // a[i+1] == 0
            if a[i + 1] == 0 {
                dp[i + 1][0] = dp[i + 1][0].min(dp[i][0]);
                dp[i + 1][1] = dp[i + 1][1].min(dp[i][1] + 1);
            } else {
                dp[i + 1][0] = dp[i + 1][0].min(dp[i][0] + 1);
                dp[i + 1][1] = dp[i + 1][1].min(dp[i][1]);
            }

            // a[i+1] == 1
        } else {
            // a[i+1] == 0
            if a[i + 1] == 0 {
                dp[i + 1][0] = dp[i + 1][0].min(dp[i][1]);
                dp[i + 1][1] = dp[i + 1][1].min(dp[i][0] + 1);
            } else {
                dp[i + 1][0] = dp[i + 1][0].min(dp[i][1] + 1);
                dp[i + 1][1] = dp[i + 1][1].min(dp[i][0]);
            }
            // a[i+1] == 1
        }
    }
    println!("{:?}", dp[n - 1][0].min(dp[n - 1][1]));
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
