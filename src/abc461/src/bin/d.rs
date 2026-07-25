use itertools::Itertools;
use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
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
        h: usize,
        w: usize,
        k: i64,
        s: [Chars; h],
    }

    let mut ans = 0;
    let mut prefix: Vec<Vec<i64>> = vec![vec![0; w + 1]; h + 1];

    let mut s_new: Vec<Vec<i64>> = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            s_new[i][j] = s[i][j].to_string().parse::<i64>().unwrap();
        }
    }

    for i in 1..h + 1 {
        for j in 1..w + 1 {
            prefix[i][j] = s_new[i - 1][j - 1];
        }
    }

    for i in 1..h + 1 {
        for j in 1..w + 1 {
            prefix[i][j] =
                prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1] + s_new[i - 1][j - 1];
        }
    }

    //  クロージャに切り出す
    let rect_sum = |i: usize, j: usize, left: usize, right: usize| -> i64 {
        prefix[i][right] + prefix[j][left] - prefix[j][right] - prefix[i][left]
    };
    // 以下で尺取法をやる
    for i in 1..h + 1 {
        for j in 0..i {
            let mut right1 = 1;
            let mut right2 = 1;
            for left in 0..w {
                if right1 < left + 1 {
                    right1 = left + 1;
                }
                if right2 < left + 1 {
                    right2 = left + 1;
                }
                while (right1 < w + 1) && (rect_sum(i, j, left, right1) < k) {
                    right1 += 1;
                }
                while (right2 < w + 1) && (rect_sum(i, j, left, right2) <= k) {
                    right2 += 1;
                }
                ans += right2 - right1;
            }
        }
    }

    println!("{}", ans);
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
