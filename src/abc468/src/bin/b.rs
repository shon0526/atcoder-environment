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
        m: usize,
        d: usize,
        s: Chars,
    }

    let mut ans = 0;

    for i in 0..m {
        let mut is_ok = true;

        for j in 0..m {
            if s[j] == '.' || (s[j] == 'G' && i.abs_diff(j) > d) {
                continue;
            }

            is_ok = false;
        }
        if is_ok {
            ans += 1;
        }
    }
    println!("{}", ans);

    //
    // let mut imos: Vec<i64> = vec![0; m + d + 2];
    //
    // for i in 0..m {
    //     if s[i] == 'G' {
    //         if i as i64 - d as i64 > 0 {
    //             imos[i - d] += 1;
    //         } else {
    //             imos[0] += 1;
    //         }
    //
    //         imos[i + d + 1] -= 1;
    //     }
    // }
    //
    // let imos: Vec<i64> = imos
    //     .iter()
    //     .scan(0i64, |prefix, x| {
    //         *prefix += *x;
    //         Some(*prefix)
    //     })
    //     .collect();
    // let ans = (0..m)
    //     .into_iter()
    //     .map(|i| imos[i])
    //     .filter(|possible| *possible == 0)
    //     .count();
    // println!("{}", ans);
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
