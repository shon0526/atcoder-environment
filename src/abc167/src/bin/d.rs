use num_traits::pow;
use proconio::{input, marker::Usize1, source::once::OnceSource};
use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    process,
};

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
        mut k: usize,
        a: [Usize1; n],
    }

    let mut q = VecDeque::new();
    let mut is_rc_vec = vec![false; n];

    let mut cur = 0;

    loop {
        if is_rc_vec[cur] {
            while let Some(top) = q.front() {
                if *top == cur {
                    break;
                }
                k -= 1;
                q.pop_front();

                if k == 0 {
                    if let Some(ans) = q.pop_front() {
                        println!("{}", ans + 1);
                    }
                    process::exit(0);
                }
            }
            break;
        }
        q.push_back(cur);
        is_rc_vec[cur] = true;
        cur = a[cur];
    }

    let mut ans_vec = vec![];
    while let Some(v) = q.pop_front() {
        ans_vec.push(v);
    }

    println!("{}", ans_vec[k % ans_vec.len()] + 1);
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
