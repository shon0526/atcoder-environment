"""ストレステスト司令塔（汎用テンプレート）。

提出コード (target/release/<contest>-<problem>) と
愚直解 (target/release/<problem>_naive) に同じランダム入力を流し、出力一致を検証する。
不一致が出たら入力を ng_<problem>.txt に保存して終了する。

使い方:
    cargo build --release -p <contest>                          # 実行前に毎回ビルド
    uv run python src/<contest>/stress/stress.py <problem> <試行回数>

事前準備（問題ごと）:
    - 愚直解 src/<contest>/src/bin/<problem>_naive.rs を作成する
    - gen.py を gen_<problem>.py にコピーし、その問題の制約に合わせて実装する
"""

import os
import random
import subprocess
import sys

# このファイルが置かれているディレクトリ (= src/<contest>/stress)
HERE = os.path.dirname(os.path.abspath(__file__))
# 一つ上の src/<contest> の名前がコンテスト ID。
CONTEST = os.path.basename(os.path.dirname(HERE))
# repo ルート (stress から 3 つ上) の target/release。
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
    if len(sys.argv) < 2:
        print("usage: python stress.py <problem> [試行回数]")
        sys.exit(1)
    problem = sys.argv[1]
    trials = int(sys.argv[2]) if len(sys.argv) > 2 else 1000

    gen = os.path.join(HERE, f"gen_{problem}.py")
    ng = os.path.join(HERE, f"ng_{problem}.txt")
    main_name = f"{CONTEST}-{problem}"
    naive_name = f"{problem}_naive"
    main_bin = bin_path(main_name)
    naive_bin = bin_path(naive_name)

    # 事前準備の存在チェック。
    if not os.path.exists(gen):
        print(f"ジェネレータがありません: {gen}")
        print(f"gen.py をコピーして gen_{problem}.py を作成してください。")
        sys.exit(1)
    for name, path in [(main_name, main_bin), (naive_name, naive_bin)]:
        if not os.path.exists(path):
            print(f"バイナリが見つかりません: {path}")
            print(f"先に `cargo build --release -p {CONTEST}` を実行してください。")
            print(f"({naive_name} には src/{CONTEST}/src/bin/{problem}_naive.rs が必要)")
            sys.exit(1)

    for trial in range(1, trials + 1):
        seed = random.randint(0, 2**31 - 1)

        # ランダム seed で gen_<problem>.py を実行して入力を得る。
        g = subprocess.run(
            [sys.executable, gen, str(seed)], capture_output=True, text=True
        )
        # 生成器の失敗をソルバー異常と取り違えないよう、ここで切り分ける。
        if g.returncode != 0:
            raise RuntimeError(
                f"gen_{problem}.py が異常終了 (seed={seed}, code={g.returncode}):\n{g.stderr}"
            )
        input_text = g.stdout

        out_main = run_solver(main_bin, input_text)
        out_naive = run_solver(naive_bin, input_text)

        if out_main != out_naive:
            print("=" * 40)
            print(f"NG: problem={problem} trial={trial} seed={seed}")
            print("--- input ---")
            print(input_text.rstrip())
            print(f"--- {main_name} (提出) ---")
            print(out_main)
            print(f"--- {naive_name} (愚直) ---")
            print(out_naive)
            print("=" * 40)
            with open(ng, "w") as f:
                f.write(input_text)
            print(f"反例を {ng} に保存しました。")
            print(f"デバッグ: {main_bin} < {ng}")
            sys.exit(1)

        if trial % 100 == 0:
            print(f"{trial}/{trials} OK")

    print(f"全 {trials} 件一致しました。")


if __name__ == "__main__":
    main()
