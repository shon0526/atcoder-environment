"""abc461 C のストレステスト司令塔。

提出コード (target/release/abc461-c) と愚直解 (target/release/c_naive) に
同じランダム入力を流し、出力が一致するか検証する。不一致が出たら入力を ng.txt に
保存して終了する。

使い方:
    cargo build --release -p abc461        # 実行前に毎回ビルド
    python3 src/abc461/stress/stress.py <試行回数>
"""

import os
import random
import subprocess
import sys

# このファイルが置かれているディレクトリ (= stress ディレクトリ)
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(HERE, "gen.py")
NG = os.path.join(HERE, "ng.txt")

# 提出コード / 愚直解の release バイナリ名。
# 提出コードは cargo-compete のターゲット名 (abc461-c)、
# 愚直解は src/bin/c_naive.rs から自動検出されるターゲット名 (c_naive)。
MAIN_BIN = "abc461-c"
NAIVE_BIN = "c_naive"

# repo ルート (stress ディレクトリから 3 つ上) の target/release。
REPO_ROOT = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
RELEASE_DIR = os.path.join(REPO_ROOT, "target", "release")


def bin_path(name: str) -> str:
    """OS に応じて .exe を付けた release バイナリの絶対パスを返す。"""
    exe = ".exe" if os.name == "nt" else ""
    return os.path.join(RELEASE_DIR, name + exe)


def run_solver(path: str, input_text: str) -> str:
    """バイナリに input_text を標準入力で渡し、標準出力を strip して返す。"""
    result = subprocess.run([path], input=input_text, capture_output=True, text=True)
    if result.returncode != 0:
        raise RuntimeError(f"{path} が異常終了 (code={result.returncode}):\n{result.stderr}")
    return result.stdout.strip()


def main() -> None:
    trials = int(sys.argv[1]) if len(sys.argv) > 1 else 1000

    main_bin = bin_path(MAIN_BIN)
    naive_bin = bin_path(NAIVE_BIN)
    for name, path in [(MAIN_BIN, main_bin), (NAIVE_BIN, naive_bin)]:
        if not os.path.exists(path):
            print(f"バイナリが見つかりません: {path}")
            print("先に `cargo build --release -p abc461` を実行してください。")
            sys.exit(1)

    for trial in range(1, trials + 1):
        seed = random.randint(0, 2**31 - 1)

        # ランダム seed で gen.py を実行して入力を得る。
        gen = subprocess.run(
            [sys.executable, GEN, str(seed)], capture_output=True, text=True
        )
        # 生成器の失敗をソルバー異常と取り違えないよう、ここで切り分ける。
        if gen.returncode != 0:
            raise RuntimeError(
                f"gen.py が異常終了 (seed={seed}, code={gen.returncode}):\n{gen.stderr}"
            )
        input_text = gen.stdout

        out_main = run_solver(main_bin, input_text)
        out_naive = run_solver(naive_bin, input_text)

        if out_main != out_naive:
            print("=" * 40)
            print(f"NG: trial={trial} seed={seed}")
            print("--- input ---")
            print(input_text.rstrip())
            print(f"--- {MAIN_BIN} (提出) ---")
            print(out_main)
            print(f"--- {NAIVE_BIN} (愚直) ---")
            print(out_naive)
            print("=" * 40)
            with open(NG, "w") as f:
                f.write(input_text)
            print(f"反例を {NG} に保存しました。")
            print(f"デバッグ: {main_bin} < {NG}")
            sys.exit(1)

        if trial % 100 == 0:
            print(f"{trial}/{trials} OK")

    print(f"全 {trials} 件一致しました。")


if __name__ == "__main__":
    main()
