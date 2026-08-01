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
        s: Chars,
    }

    let n = s.len();
    let mut b_idx = 1;
    let mut c_idx: usize = 2;
    let mut ans = 0;

    if n < 3 {
        println!("{}", ans);
        return;
    }

    for i in 0..n - 2 {
        if s[i] != 'A' {
            continue;
        }
        let mut is_b = false;
        let mut is_c = false;

        if i > b_idx {
            b_idx = i + 1;
        }

        loop {
            if b_idx > n - 2 {
                break;
            }

            if s[b_idx] == 'B' {
                is_b = true;
                break;
            }

            b_idx += 1;
        }
        if b_idx > n - 2 {
            break;
        }

        if b_idx > c_idx {
            c_idx = b_idx + 1;
        }

        loop {
            if c_idx > n - 1 {
                break;
            }

            if s[c_idx] == 'C' {
                is_c = true;
                break;
            }

            c_idx += 1;
        }

        if c_idx > n - 1 {
            break;
        }

        if is_b && is_c {
            ans += 1;
        }
        b_idx += 1;
        c_idx += 1;
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
