use num_traits::pow;
use proconio::{
    input,
    marker::{Chars, Usize1},
    source::once::OnceSource,
};
use std::cmp::min;
use std::collections::{BinaryHeap, HashMap};

const INF: usize = usize::MAX;
const PROBLEM: &str = "c";

// 入力文字列をパースして答えの文字列を返す。
// ランダムテスト時は同コンテストの stress/naive_test.rs をこのファイル末尾に
// 貼り付けて naive と比較する(詳細はリポジトリルートの README.md)。
fn solve(input_str: &str) -> String {
    let mut source = OnceSource::from(input_str);
    input! {
        from &mut source,
        h: usize,
        w: usize,
        k: usize,
        s: [Chars;h],
    }

    let mut ans = INF;
    for bit in 0..1 << h {
        let mut vec_size = 0;
        for i in 0..h {
            if (bit & (1 << i)) != 0 {
                vec_size += 1;
            }
        }

        let mut cnt_vec = vec![0; vec_size + 1];

        let mut cnt_v = 0;
        for j in 0..w {
            let mut now_vec = vec![0; vec_size + 1];
            let mut is_ok = false;

            let mut idx = 0;
            for i in 0..h {
                if (bit & (1 << i)) != 0 {
                    if s[i][j] == '1' {
                        now_vec[idx] += 1;
                        if cnt_vec[idx] + now_vec[idx] > k {
                            is_ok = true;
                            cnt_v += 1;
                        }
                    }
                    idx += 1;
                } else {
                    if s[i][j] == '1' {
                        now_vec[idx] += 1;
                        if cnt_vec[idx] + now_vec[idx] > k {
                            is_ok = true;
                            cnt_v += 1;
                        }
                    }
                }
            }
            if is_ok {
                cnt_vec = now_vec.clone();
            } else {
                for i in 0..vec_size + 1 {
                    cnt_vec[i] += now_vec[i];
                }
            }
        }

        ans = min(ans, vec_size + cnt_v);
    }
    ans.to_string()
}

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
    println!("{}", solve(&input));
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

#[cfg(test)]
mod random_tests {
    use super::*;
    use proconio::{input, source::once::OnceSource};
    use std::process::Command;

    /// 問題名。gen_<PROBLEM>.py / ng_<PROBLEM>.txt の名前解決に使う。
    const PROBLEM: &str = "a";
    /// 試行回数。seed は 0..TRIALS の連番なので毎回同じ入力列になる。
    const TRIALS: u64 = 500;

    // 愚直解。solve と同じシグネチャで、正しさ優先で実装する。
    fn naive(input_str: &str) -> String {
        let mut source = OnceSource::from(input_str);
        input! {
            from &mut source,
            n: usize,
            a: [usize; n],
        }
        todo!()
    }

    // CARGO_MANIFEST_DIR (= src/<contest>) 基準で stress/ 内のパスを返す。
    fn stress_path(name: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("stress")
            .join(name)
    }

    // gen_<PROBLEM>.py に seed を渡して実行し、生成された入力文字列を返す。
    fn generate(seed: u64) -> String {
        let gen = stress_path(&format!("gen_{PROBLEM}.py"));
        assert!(
            gen.exists(),
            "ジェネレータがありません: {}\nstress/gen.py をコピーして gen_{PROBLEM}.py を作成してください。",
            gen.display(),
        );
        let output = Command::new("python3")
            .arg(&gen)
            .arg(seed.to_string())
            .output()
            .expect("python3 の起動に失敗");
        assert!(
            output.status.success(),
            "{} が異常終了 (seed={seed}):\n{}",
            gen.display(),
            String::from_utf8_lossy(&output.stderr),
        );
        String::from_utf8(output.stdout).expect("gen の出力が UTF-8 でない")
    }

    // solve と naive の出力を全 seed で比較する。
    // 不一致なら入力を ng_<PROBLEM>.txt に保存して失敗する。
    #[test]
    fn stress() {
        for seed in 0..TRIALS {
            let input_str = generate(seed);
            let main_out = solve(&input_str);
            let naive_out = naive(&input_str);
            if main_out.trim() != naive_out.trim() {
                let ng = stress_path(&format!("ng_{PROBLEM}.txt"));
                std::fs::write(&ng, &input_str).expect("ng ファイルの書き込みに失敗");
                panic!(
                    "NG: seed={seed}\n--- input ({} に保存) ---\n{}--- solve ---\n{}\n--- naive ---\n{}",
                    ng.display(),
                    input_str,
                    main_out.trim(),
                    naive_out.trim(),
                );
            }
        }
    }
}
