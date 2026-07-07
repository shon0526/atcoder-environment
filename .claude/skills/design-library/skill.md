---
name: design-library
description: mylibに新しいデータ構造・アルゴリズムのモジュールを設計・実装するときの手順ガイド
---

## design-library

`mylib`に新しいデータ構造やアルゴリズムのモジュールを追加するときは、必ずこの手順に従う。

## 作業手順

### 1. 車輪の再発明チェック

実装を始める前に、同等の機能が既に使えるものに存在しないか確認する。

- `mylib/src/` の既存モジュール（`lib.rs`の`pub mod`一覧を見る）
- `compete.toml`のdependenciesにあるクレート。特に確認するもの:
  - `ac-library-rs`: FenwickTree, Segtree, LazySegtree, Dsu(UnionFind), 数論(modint等), maxflow, scc など
  - `pathfinding`: BFS/DFS/Dijkstra/トポロジカルソートなどのグラフアルゴリズム
  - `superslice`: lower_bound / upper_bound
  - `counter`, `hashbag`: 多重集合系
  - `primal`: 素数関連
- 存在する場合はその旨をユーザーに伝え、それでも自作するか確認する（学習目的の再実装はあり得るため、勝手に中止しない）

### 2. API設計

実装前にAPI（構造体名・メソッド一覧・シグネチャ）を提示してユーザーの合意を取る。勝手に実装を進めない。

- 既存モジュール（`multiset.rs`など）と流儀を揃える: `new()` / `Default` / `From<Vec<T>>` / `FromIterator` / `is_empty()` / `len()` など標準コレクション風のAPI
- 命名は`std::collections`のBTreeMap/BTreeSetの慣習に合わせる（`insert`, `remove`, `contains`, `iter`, `range`, ...）
- ジェネリクスにする場合、トレイト境界は必要最小限にしてメソッド単位で付ける（`multiset.rs`の`pop_first`が`where T: Ord + Clone`をメソッド側に付けているのと同じ方式）

### 3. 実装

- ファイルは `mylib/src/<module_name>.rs` に作成し、`mylib/src/lib.rs` に `pub mod <module_name>;` を追加する
- 先頭に `use cargo_snippet::snippet;` を書き、モジュール全体を `#[snippet(name = "<snippet名>")]` 付きの `pub mod` で包む（`multiset.rs`の`btree_multiset`と同じ構造）
- **各構造体・各メソッドに1〜2行の振る舞いコメントを付ける（CLAUDE.mdの規約）。加えて計算量を明記する**:

```rust
// x以上の最小の要素を返す。存在しなければNone。O(log N)
pub fn lower_bound(...) -> Option<&T> { ... }
```

- 償却計算量の場合は「償却O(1)」のように区別して書く
- 変数名はその場で適切な名前を使う（a, bなどにしない）

### 4. テスト

- 同ファイル末尾に `#[cfg(test)] mod tests` を書く（`multiset.rs`のテストと同じスタイル）
- 最低限カバーする観点:
  - 空の状態（`new`直後の各操作）
  - 基本操作の系列（挿入→検索→削除）
  - 境界（要素1個、重複要素、削除して空に戻る、範囲の端）
- ランダム入力での正当性検証が必要な規模のデータ構造なら、`/verify-datastructure` を続けて実行する
- `mylib`ディレクトリで `cargo test` を実行し、全テストが通ることを確認する

### 5. スニペット登録

`mylib`ディレクトリで以下を実行し、`snippets/rust.json` を再生成する。

```bash
cargo snippet -t vscode > ../snippets/rust.json
```

再生成後、`snippets/rust.json` に新しいスニペット名が含まれること、既存のスニペットが消えていないことを確認する。

## 完了条件

- [ ] 既存クレート・既存モジュールとの重複を確認済み
- [ ] 全メソッドに振る舞いコメント + 計算量コメントがある
- [ ] `cargo test` が全て通る
- [ ] `snippets/rust.json` が再生成されている
