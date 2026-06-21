# abc461 ストレステスト

提出コードと愚直解にランダム入力を流し、出力一致を検証する。
（このディレクトリは `./compete-new.sh` が雛形として自動生成する）

## 構成

| ファイル | 役割 |
| --- | --- |
| `stress.py` | 司令塔（汎用）。`<problem>` を引数に取り使い回す |
| `gen.py` | ジェネレータ雛形。問題ごとに `gen_<problem>.py` にコピーして実装する |
| `gen_<problem>.py` | 各問題用のランダム入力生成（`argv[1]` を seed に再現可能） |
| `ng_<problem>.txt` | 反例が見つかったとき自動保存される入力 |

## 各問題でやること

1. 愚直解を作成: `src/abc461/src/bin/<problem>_naive.rs`
   提出コード `<problem>.rs` の入力読込をそのまま流用し、遅いが確実に正しい解法を書く。
2. ジェネレータを作成: `gen.py` を `gen_<problem>.py` にコピーし、その問題の制約・入出力形式に合わせて実装する。

## 実行

```bash
# 提出コードと愚直解をビルド（実行前に毎回）
cargo build --release -p abc461

# 問題と試行回数を指定して実行
uv run python src/abc461/stress/stress.py <problem> <試行回数>
```

不一致が出ると trial 番号・seed・入力・両出力を表示し、入力を `ng_<problem>.txt` に保存して終了する。

## デバッグ

```bash
target/release/abc461-<problem> < src/abc461/stress/ng_<problem>.txt
```

## メモ

- 提出バイナリ名は cargo-compete 設定に合わせて `abc461-<problem>`、
  愚直解は `src/bin/<problem>_naive.rs` から自動検出される `<problem>_naive`。
- 愚直解 `<problem>_naive` は検証専用であり、提出対象には含めない。
- 答えが一意でない問題は、naive を「main の出力を検証するチェッカー」にする方式にする。
