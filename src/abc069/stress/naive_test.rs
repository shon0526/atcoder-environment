// ==== ランダムテスト(必要な問題にのみ、解答ファイルの末尾に貼り付ける) ====
// モジュール全体が #[cfg(test)] のため、提出ビルドには含まれない。
// 貼り付け後にやること:
//   1. PROBLEM 定数を問題名(a, b, c, ...)に合わせる
//   2. naive を実装する(遅くてよいので確実に正しい解法)
//   3. stress/gen.py を stress/gen_<problem>.py にコピーして制約に合わせて実装する
// 実行: コンテストディレクトリ(src/<contest>)で cargo test --bin <contest>-<problem>

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
