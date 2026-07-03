# atcoder-environment
atcoderをrustで解くためのリポジトリ

ユーザー名：shonNMIXX

[https://atcoder.jp/users/shonNMIXX](https://atcoder.jp/users/shonNMIXX)

## atcoderにログインする
`.env`ファイルを作成する。webサイトのデベロッパーツールからセッションIDを取得後に
`REVEAL_SESSION`の値に追加する
```bash
REVEAL_SESSION="<session_id>"
```
その後に`setup.sh`を実行する

```
./setup.sh
```

## コンテスト用のファイルを取得
```bash
./compete-new.sh abc<hoge>
```

## テストコマンド
各コンテストのディレクトリ内で下記のコマンドを実行する
```bash
cargo compete test <problem>
```

## 提出コマンド
各コンテストのディレクトリ内で下記のコマンドを実行する
```bash
cargp compete submit <probelm>
```

## ランダムテスト（ストレステスト）
解答ファイル内の `solve` と愚直解 `naive` に同じランダム入力（Python 生成）を渡し、
`cargo test` で出力一致を検証する。
`src/<contest>/stress/`（`naive_test.rs`=貼り付け用雛形 / `gen.py`=ジェネレータ雛形）は
`./compete-new.sh` 実行時に `snippets/stress/` の雛形から自動生成される
（既存ディレクトリでも足りないファイルだけ補充される）。

### 各問題でやること（必要な問題だけ）
1. `src/<contest>/stress/naive_test.rs` の内容を解答ファイル `<problem>.rs` の末尾に貼り付ける
2. `PROBLEM` 定数を問題名に合わせ、`naive` を実装する（遅くてよいので確実に正しい解法）
3. `gen.py` を `gen_<problem>.py` にコピーして制約に合わせて実装する

### 実行手順
各コンテストのディレクトリ（`src/<contest>`）内で下記のコマンドを実行する。
```bash
cargo test --bin <contest>-<problem>
```

不一致が出ると seed・入力・両出力を panic メッセージに表示し、入力を
`stress/ng_<problem>.txt` に保存して失敗する。反例は下記でデバッグできる。
```bash
cargo run --bin <contest>-<problem> < stress/ng_<problem>.txt
```

例（abc461 C、`src/abc461` 内で実行）:
```bash
cargo test --bin abc461-c
cargo run --bin abc461-c < stress/ng_c.txt   # 反例のデバッグ
```

### メモ
- 貼り付けたテストブロックは全体が `#[cfg(test)]` なので提出ビルドに含まれず、そのまま提出してよい。
- seed は `0..TRIALS` の連番で、毎回同じ入力列になり再現性がある。
- `gen_<problem>.py` が未作成の場合はその旨のメッセージでテストが失敗する。
- 比較は出力文字列の完全一致（前後 trim のみ）。答えが一意でない問題はテスト内の比較部分を
  問題に合わせて書き換える。インタラクティブ問題は対象外。
- 旧方式（`stress.py` + `<problem>_naive.rs` の別バイナリ比較）はレガシーであり、
  過去コンテストのディレクトリに残っているものは履歴として保持するのみで新規には使わない。
- 設計の詳細は `docs/random-test-redesign.md` を参照。
