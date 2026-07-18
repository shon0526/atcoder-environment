use proconio::{input, marker::Chars, source::once::OnceSource};
use std::cmp::min;

const INF: usize = usize::MAX;

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

    let mut res = 1 << 29;

    for bit in 0..(1 << (h - 1)) {
        let mut gok = true;
        let mut n = 0;
        let mut ord = vec![0; h];

        for i in 0..h - 1 {
            if (bit & 1 << i) != 0 {
                ord[i + 1] = ord[i] + 1;
                n += 1;
            } else {
                ord[i + 1] = ord[i];
            }
        }

        let mut add = 0;
        let mut nums = vec![0; n + 1];

        for j in 0..w {
            let mut ones = vec![0; n + 1];
            let mut is_ok = true;

            for i in 0..h {
                if s[i][j] == '1' {
                    ones[ord[i]] += 1;
                    nums[ord[i]] += 1;
                }
                if ones[ord[i]] > k {
                    gok = false;
                }
                if nums[ord[i]] > k {
                    is_ok = false;
                }
            }

            if !is_ok {
                nums = ones;
                add += 1;
            }
        }

        if gok {
            res = min(res, n + add);
        }
    }
    res.to_string()
}

fn main() {
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap();
    println!("{}", solve(&buf));
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
    const PROBLEM: &str = "e";
    /// 試行回数。seed は 0..TRIALS の連番なので毎回同じ入力列になる。
    const TRIALS: u64 = 500;

    fn naive(input_str: &str) -> String {
        let mut source = OnceSource::from(input_str);
        input! {
            from &mut source,
            h: usize,
            w: usize,
            k: usize,
            s: [Chars; h],
        }

        let mut ans = h + w;
        for hbit in 0u32..1 << (h - 1) {
            for vbit in 0u32..1 << (w - 1) {
                let cuts = (hbit.count_ones() + vbit.count_ones()) as usize;

                let mut row_bounds = vec![0];
                for i in 0..h - 1 {
                    if hbit & (1 << i) != 0 {
                        row_bounds.push(i + 1);
                    }
                }
                row_bounds.push(h);

                let mut col_bounds = vec![0];
                for j in 0..w - 1 {
                    if vbit & (1 << j) != 0 {
                        col_bounds.push(j + 1);
                    }
                }
                col_bounds.push(w);

                let mut valid = true;
                'outer: for ri in 0..row_bounds.len() - 1 {
                    for ci in 0..col_bounds.len() - 1 {
                        let mut count = 0;
                        for r in row_bounds[ri]..row_bounds[ri + 1] {
                            for c in col_bounds[ci]..col_bounds[ci + 1] {
                                if s[r][c] == '1' {
                                    count += 1;
                                }
                            }
                        }
                        if count > k {
                            valid = false;
                            break 'outer;
                        }
                    }
                }

                if valid {
                    ans = min(ans, cuts);
                }
            }
        }
        ans.to_string()
    }

    // CARGO_MANIFEST_DIR (= src/<contest>) 基準で stress/ 内のパスを返す。
    fn stress_path(name: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("stress")
            .join(name)
    }

    // gen_<PROBLEM>.py に seed を渡して実行し、生成された入力文字列を返す。
    fn generate(seed: u64) -> String {
        let gen_path = stress_path(&format!("gen_{PROBLEM}.py"));
        assert!(
            gen_path.exists(),
            "ジェネレータがありません: {}\nstress/gen.py をコピーして gen_{PROBLEM}.py を作成してください。",
            gen_path.display(),
        );
        let output = Command::new("python3")
            .arg(&gen_path)
            .arg(seed.to_string())
            .output()
            .expect("python3 の起動に失敗");
        assert!(
            output.status.success(),
            "{} が異常終了 (seed={seed}):\n{}",
            gen_path.display(),
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
