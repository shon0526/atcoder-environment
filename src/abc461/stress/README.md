# abc461 C ストレステスト

提出コード (`src/abc461/src/bin/c.rs`) と愚直解 (`src/abc461/src/bin/c_naive.rs`) に
同じランダム入力を流し、出力一致を検証する。

## 構成

| ファイル | 役割 |
| --- | --- |
| `gen.py` | ランダム入力生成。`argv[1]` を seed にして再現可能 |
| `stress.py` | 司令塔。生成 → 両解実行 → 出力比較 |
| `ng.txt` | 反例が見つかったとき自動保存される入力（初回実行時に生成） |

## 実行手順

```bash
# 提出コードと愚直解をビルド（実行前に毎回）
cargo build --release -p abc461

# 試行回数を指定して実行
python3 src/abc461/stress/stress.py <試行回数>
```

不一致が出ると trial 番号・seed・入力・両出力を表示し、入力を `ng.txt` に保存して終了する。

## デバッグ

```bash
target/release/abc461-c < src/abc461/stress/ng.txt
```

## メモ

- 提出バイナリ名はこのリポジトリの cargo-compete 設定に合わせて `abc461-c`、
  愚直解は `src/bin/c_naive.rs` から自動検出される `c_naive`。
  バイナリ名は `stress.py` 冒頭の `MAIN_BIN` / `NAIVE_BIN` で変更できる。
- 愚直解 `c_naive` は検証専用であり、提出対象には含めない。
- 本問題は答えが一意（整数1つ）なので、愚直解の出力をそのまま正解として比較している。
