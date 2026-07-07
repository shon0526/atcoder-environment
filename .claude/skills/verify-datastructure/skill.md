---
name: verify-datastructure
description: mylibのデータ構造を愚直実装とのランダム比較テストで正当性検証します
---

## verify-datastructure

`mylib`のデータ構造・アルゴリズムに対し、`Vec`等を使った愚直実装（naive）を用意し、ランダムな操作列で結果を照合する`cargo test`を作成する。コンテスト解答のsolve/naive比較（`src/<contest>/stress/`）と同じ思想を、ライブラリ単体に適用するもの。

## 作業手順

### 1. 検証対象の操作を洗い出す

対象モジュールのpublicメソッドを列挙し、次の2種類に分類する。

- **変更操作**: insert, remove, push, pop, merge など状態を変えるもの
- **観測操作**: contains, count, len, first/last, lower_bound, range など状態を読むもの

観測操作の戻り値が愚直実装と一致すれば正しい、という形に落とし込む。

### 2. 愚直実装（naive）を書く

テストモジュール内に、`Vec`やソート済み`Vec`を使った自明に正しい実装を書く。

- 速度は不要。O(N)やO(N^2)でよいので**読んで明らかに正しい**ことを最優先にする
- 例: MultiSetのnaiveは「ソートせず持つ`Vec<T>`」で、`lower_bound`は`iter().filter(...).min()`で実装する

### 3. ランダム比較テストを書く

対象モジュールのファイル末尾の `#[cfg(test)] mod tests` に追加する。乱数は`compete.toml`のdependenciesにある`rand`系クレートが`mylib`には入っていないため、**外部クレートを追加せず、シード付きの自前XorShiftをテスト内に書く**（数行で済む）。

```rust
// テスト用の決定的な乱数生成器。シードから同じ列を再現できる。
struct XorShift(u64);
impl XorShift {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
```

テスト本体の構造:

```rust
#[test]
fn random_test_against_naive() {
    for seed in 1..=20 {
        let mut rng = XorShift(seed);
        let mut target = MultiSet::new();
        let mut naive: Vec<i64> = Vec::new();
        let mut history = Vec::new(); // 失敗時に再現できるよう操作列を記録

        for _ in 0..1000 {
            // 値域を狭くして（例: 0..20）重複・削除の衝突を意図的に起こす
            let value = (rng.next() % 20) as i64;
            match rng.next() % 3 {
                0 => { /* targetとnaive両方にinsert */ }
                1 => { /* targetとnaive両方からremove */ }
                _ => { /* 観測操作の結果をassert_eqで照合 */ }
            }
            history.push(/* 操作の記録 */);
            // 毎ステップ全観測操作を照合。失敗時はseedと操作列をメッセージに含める
            assert_eq!(target.size(), naive.len(), "seed={seed}, history={history:?}");
        }
    }
}
```

設計上の要点:

- **シードをループで複数回す**（20シード×1000操作程度）。失敗時にどのseedかをassertメッセージに必ず含め、再現可能にする
- **値域を意図的に狭くする**。値域が広いと重複や「存在しない要素の削除」が起きず、バグが露出しない
- 変更操作のたびに主要な観測操作（len/size/count等）を照合し、数十ステップごと（または最後）に全体照合（iter結果のVec比較など）を行う
- 操作の選択比率は偏らせてもよい（削除多めで空に戻るケースを踏ませる等）

### 4. 境界ケースの個別テスト

ランダムテストとは別に、狙い撃ちのテストを追加する。チェックリスト:

- [ ] 空の状態で観測操作・削除操作を呼ぶ（None/false/0が返る）
- [ ] 要素1個で全操作
- [ ] 同じ値を複数回挿入 → 1個ずつ削除（重複管理の整合性）
- [ ] 全要素削除して空に戻す → 再度挿入
- [ ] 型の境界値（i64::MIN / i64::MAX など）
- [ ] 範囲系操作で「範囲の端＝要素」「範囲内に要素なし」

### 5. 実行と確認

```bash
cd mylib && cargo test
```

- 全テストが通ることを確認する
- 失敗した場合は、assertメッセージのseedと操作列から最小再現手順を特定し、対象モジュール側のバグかnaive側のバグかを切り分けてから修正する（**勝手に対象モジュールを修正せず、原因を報告してから直す**）
- ランダムテストが遅すぎる場合（1テスト数秒以上）は操作回数を減らすか`--release`での実行を案内する

## 完了条件

- [ ] naive実装との照合テストが複数シードで通る
- [ ] 境界ケースの個別テストがある
- [ ] 失敗時にseedから再現できる形になっている
