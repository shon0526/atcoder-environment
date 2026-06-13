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
提出コードと愚直解にランダム入力を流し、出力一致を検証する。
構成は `src/<contest>/stress/`（`gen.py`=入力生成 / `stress.py`=司令塔）に置く。
愚直解は `src/<contest>/src/bin/<problem>_naive.rs` に作成する。

### Python 環境（uv）
Python スクリプトは uv の仮想環境で実行する。初回のみ同期する。
```bash
uv sync
```

### 実行手順
```bash
# 提出コードと愚直解をビルド（実行前に毎回）
cargo build --release -p <contest>

# 試行回数を指定して実行
uv run python src/<contest>/stress/stress.py <試行回数>
```

不一致が出ると trial 番号・seed・入力・両出力を表示し、入力を
`src/<contest>/stress/ng.txt` に保存して終了する。反例は下記でデバッグできる。
```bash
target/release/<contest>-<problem> < src/<contest>/stress/ng.txt
```

例（abc461 C）:
```bash
cargo build --release -p abc461
uv run python src/abc461/stress/stress.py 2000
target/release/abc461-c < src/abc461/stress/ng.txt   # 反例のデバッグ
```
詳細は `src/abc461/stress/README.md` を参照。
