# 手順書: range_max_segtree に MaxIdentity トレイトを導入し `From<Vec<T>>` を実装する

## この作業の目的

`mylib/src/range_max_segtree.rs` の `SegTree<T>` は、max 演算の単位元を
`element: T` フィールドに「実行時の値」として保持している。
そのため `From<Vec<T>>` を実装しようとすると、`from(vec: Vec<T>) -> Self` の
引数が `Vec<T>` ひとつしかなく、単位元を受け取る口がないので構築できない
（現状 82〜87 行の `impl From` は本体が空でコンパイルも通らない）。

### 解決方針

**単位元を「型の責務」に移す。**
`MaxIdentity` トレイトを定義し、`i64` などが「自分の最小値」を返せるようにする。
すると `SegTree` は単位元フィールドを持つ必要がなくなり、`T::min_identity()` から
いつでも単位元を取れるため `From<Vec<T>>` が素直に書ける。

あわせて、現在の `update` / `get_range` は「0-indexed・配列長 2n-1」と
「1-indexed 走査」が混在していてバグっているので、
**1-indexed・配列長 2n の定番反復セグ木**に内部を書き直す。

### 成果物

- `mylib/src/range_max_segtree.rs` を「完成形」で置き換え
- テストを通す（`cargo test`）
- スニペット `snippets/rust.json` を再生成
- `operation.md` に作業ログを追記

---

## 変更対象ファイル

| ファイル | 変更内容 |
|---|---|
| `mylib/src/range_max_segtree.rs` | 全面書き換え（本体＋テスト） |
| `mylib/src/lib.rs` | 変更不要（`pub mod range_max_segtree;` は既にある） |
| `snippets/rust.json` | `cargo snippet` で再生成 |
| `operation.md`（リポジトリ直下） | 作業ログを追記（最後に `/write-operation`） |

---

## 設計（頭に入れる3点）

1. **単位元は型が知っている**
   `MaxIdentity::min_identity()` が「その型で max の単位元になる値（＝最小値）」を返す。
   `i64` なら `i64::MIN`。`x.max(i64::MIN) == x` が常に成り立つのがルール。

2. **セグ木の配列レイアウト（1-indexed・長さ 2n）**
   - `tree[1]` が根
   - 葉は `tree[n]` 〜 `tree[2n-1]`（`tree[n + idx]` が `idx` 番目の要素）
   - 親 `i` の子は `tree[2*i]` と `tree[2*i + 1]`
   - この形は `n` が2の冪でなくても正しく動く（証明済みの定番アルゴリズム）

3. **区間クエリは半開区間 [l, r) に統一**
   `RangeBounds` を受け取り、内部で必ず `[l, r)` に正規化してから木をたどる。

---

## 手順

### Step 0: 現状確認

```bash
cd /Users/shon/Desktop/atcoder-environment/mylib
cargo test 2>&1 | head -30   # 今は range_max_segtree のせいで失敗するはず
```

失敗を確認したら、`src/range_max_segtree.rs` を丸ごと消して、以降のコードを
上から順に書いていく（一気に貼らず、ブロックごとに理解しながら進める）。

### Step 1: モジュールの外枠と `MaxIdentity` トレイト

```rust
use cargo_snippet::snippet;

#[snippet(name = "range-max-segtree")]
pub mod range_max_segtree {
    use std::ops::{Bound, RangeBounds};

    /// max 演算の単位元を型ごとに定めるトレイト。
    /// min_identity() は「その型で取りうる最小値」を返し、
    /// 任意の x について x.max(min_identity()) == x が成り立つ必要がある。
    pub trait MaxIdentity: Copy + Ord {
        fn min_identity() -> Self;
    }

    /// 整数型に対して MaxIdentity をまとめて実装するマクロ。
    /// <$t>::MIN は各整数型が持つ最小値の定数。
    macro_rules! impl_max_identity {
        ($($t:ty),*) => {
            $(
                impl MaxIdentity for $t {
                    fn min_identity() -> Self {
                        <$t>::MIN
                    }
                }
            )*
        };
    }
    impl_max_identity!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
```

- `Copy + Ord` を親トレイトに付けているので、`T: MaxIdentity` と書くだけで
  「コピーでき、比較できる」が保証される。
- マクロが難しければ、使う型だけ手書きでもよい
  （`impl MaxIdentity for i64 { fn min_identity() -> Self { i64::MIN } }` を必要な型ぶん並べる）。

### Step 2: `SegTree` 構造体と `new`

```rust
    /// 一点更新・区間最大クエリに答えるセグメント木。
    /// 1-indexed・配列長 2n の反復（ボトムアップ）実装。
    #[derive(Debug, Clone)]
    pub struct SegTree<T> {
        /// 葉の数（扱う要素数）
        n: usize,
        /// tree[1] が根。葉は tree[n..2*n] に並ぶ。
        tree: Vec<T>,
    }

    impl<T: MaxIdentity> SegTree<T> {
        /// すべて単位元で初期化した、長さ n のセグ木を作る。O(n)
        pub fn new(n: usize) -> Self {
            Self {
                n,
                tree: vec![T::min_identity(); 2 * n],
            }
        }
```

- ポイント：`element` フィールドが**消えた**。単位元は `T::min_identity()` で取れるから。
- `n == 0` でも `tree` が空になるだけで壊れない（クエリは単位元を返す）。

### Step 3: `update` と `get`

```rust
        /// idx 番目の値を v に更新する。O(log n)
        pub fn update(&mut self, idx: usize, v: T) {
            assert!(idx < self.n);
            let mut i = idx + self.n; // 葉の位置
            self.tree[i] = v;
            while i > 1 {
                i >>= 1; // 親へ
                self.tree[i] = self.tree[2 * i].max(self.tree[2 * i + 1]);
            }
        }

        /// idx 番目の現在値を返す。O(1)
        pub fn get(&self, idx: usize) -> T {
            assert!(idx < self.n);
            self.tree[idx + self.n]
        }
```

### Step 4: `get_range` と正規化ヘルパ

```rust
        /// range の区間最大値を返す。空区間なら単位元。O(log n)
        pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> T {
            let (mut l, mut r) = self.to_half_open(range); // [l, r)
            let mut res = T::min_identity();
            l += self.n;
            r += self.n;
            while l < r {
                if l & 1 == 1 {
                    res = res.max(self.tree[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    res = res.max(self.tree[r]);
                }
                l >>= 1;
                r >>= 1;
            }
            res
        }

        /// RangeBounds を半開区間 [l, r) に変換する。
        /// 範囲外の指定はパニックさせて誤用を弾く。
        fn to_half_open<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
            let l = match range.start_bound() {
                Bound::Included(&s) => s,
                Bound::Excluded(&s) => s + 1,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Included(&e) => e + 1,
                Bound::Excluded(&e) => e,
                Bound::Unbounded => self.n,
            };
            assert!(l <= r && r <= self.n);
            (l, r)
        }
    }
```

- `get_range(3..=5)` や `get_range(..)`、`get_range((Excluded(0), Included(2)))` が全部通る。
- 空区間（`l == r`）は while ループに入らず単位元が返る。

### Step 5: `From<Vec<T>>` ← 本題

```rust
    impl<T: MaxIdentity> From<Vec<T>> for SegTree<T> {
        /// 既存の列からセグ木を O(n) で構築する。
        /// 単位元は T::min_identity() から得られるので、Vec だけで構築できる。
        fn from(values: Vec<T>) -> Self {
            let n = values.len();
            let mut tree = vec![T::min_identity(); 2 * n];
            tree[n..2 * n].copy_from_slice(&values); // 葉を埋める
            for i in (1..n).rev() {
                tree[i] = tree[2 * i].max(tree[2 * i + 1]); // 下から親を計算
            }
            Self { n, tree }
        }
    }
```

- ここが「実装不可能では？」と悩んでいた箇所。
  単位元を**型から**取れるようになったので、`from` の引数が `Vec<T>` だけでも困らない。
- `copy_from_slice` は `T: Copy`（`MaxIdentity` の親トレイト）だから使える。
- `values` が空なら `n = 0`、`tree` も空、ループは回らない。安全。

### Step 6: `FromIterator<T>`（おまけ・既存モジュールに合わせる）

```rust
    impl<T: MaxIdentity> FromIterator<T> for SegTree<T> {
        /// イテレータからセグ木を構築する。O(n)
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            Self::from(iter.into_iter().collect::<Vec<T>>())
        }
    }
}
```

これで `(0..n).map(...).collect::<SegTree<i64>>()` が書ける
（`multiset.rs` が `FromIterator` を持っているのと同じ流儀）。

### Step 7: テスト（ファイル末尾）

`multiset.rs` のテストと同じスタイル。観点：`new` 直後 / `From` / 更新 / 各種
`RangeBounds` / 要素1個 / 空区間 / `FromIterator`。

```rust
#[cfg(test)]
mod tests {
    use crate::range_max_segtree::range_max_segtree::SegTree;
    use std::ops::Bound::{Excluded, Included};

    #[test]
    fn test_new() {
        let seg: SegTree<i64> = SegTree::new(5);
        assert_eq!(seg.get_range(..), i64::MIN);
        for i in 0..5 {
            assert_eq!(seg.get(i), i64::MIN);
        }
    }

    #[test]
    fn test_from_and_get_range() {
        let seg = SegTree::from(vec![3i64, 1, 4, 1, 5, 9, 2, 6]);
        assert_eq!(seg.get_range(..), 9);
        assert_eq!(seg.get_range(0..3), 4);
        assert_eq!(seg.get_range(1..2), 1);
        assert_eq!(seg.get_range(3..=5), 9);
        assert_eq!(seg.get_range(6..), 6);
    }

    #[test]
    fn test_update() {
        let mut seg = SegTree::from(vec![3i64, 1, 4, 1, 5]);
        seg.update(1, 10);
        assert_eq!(seg.get(1), 10);
        assert_eq!(seg.get_range(..), 10);
        assert_eq!(seg.get_range(0..1), 3);
        seg.update(1, -2);
        assert_eq!(seg.get_range(..), 5);
        assert_eq!(seg.get_range(0..2), 3);
    }

    #[test]
    fn test_range_bounds() {
        let seg = SegTree::from(vec![5i64, 3, 8, 1, 9, 2]);
        assert_eq!(seg.get_range(2..4), 8);
        assert_eq!(seg.get_range(2..=4), 9);
        assert_eq!(seg.get_range((Excluded(0), Included(2))), 8);
        assert_eq!(seg.get_range((Excluded(4), Excluded(5))), i64::MIN); // 空区間
    }

    #[test]
    fn test_single_element() {
        let mut seg = SegTree::from(vec![42i64]);
        assert_eq!(seg.get_range(..), 42);
        assert_eq!(seg.get(0), 42);
        seg.update(0, 7);
        assert_eq!(seg.get_range(0..1), 7);
    }

    #[test]
    fn test_from_iter() {
        // x*x - 5x  (x = 0..10): 最小 -6、最大は x=9 の 36
        let seg: SegTree<i64> = (0..10).map(|x| x * x - 5 * x).collect();
        assert_eq!(seg.get_range(..), 36);
        assert_eq!(seg.get_range(0..5), 0);
    }
}
```

### Step 8: テスト実行

```bash
cd /Users/shon/Desktop/atcoder-environment/mylib
cargo test range_max_segtree
cargo test          # 他のモジュールを壊していないことも確認
```

全部緑になればOK。

### Step 9: スニペット再生成

```bash
cd /Users/shon/Desktop/atcoder-environment/mylib
cargo snippet -t vscode > ../snippets/rust.json
```

再生成後の確認：

```bash
grep -c "range-max-segtree" ../snippets/rust.json   # 1 以上になる
grep -c "btree_multiset" ../snippets/rust.json      # 既存が消えていない
git diff --stat ../snippets/rust.json
```

### Step 10: 作業ログ

`/write-operation` を実行して `operation.md` に今回の作業を追記する。

---

## 完成形（`mylib/src/range_max_segtree.rs` 全文）

```rust
use cargo_snippet::snippet;

#[snippet(name = "range-max-segtree")]
pub mod range_max_segtree {
    use std::ops::{Bound, RangeBounds};

    /// max 演算の単位元を型ごとに定めるトレイト。
    /// min_identity() は「その型で取りうる最小値」を返し、
    /// 任意の x について x.max(min_identity()) == x が成り立つ必要がある。
    pub trait MaxIdentity: Copy + Ord {
        fn min_identity() -> Self;
    }

    /// 整数型に対して MaxIdentity をまとめて実装するマクロ。
    macro_rules! impl_max_identity {
        ($($t:ty),*) => {
            $(
                impl MaxIdentity for $t {
                    fn min_identity() -> Self {
                        <$t>::MIN
                    }
                }
            )*
        };
    }
    impl_max_identity!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

    /// 一点更新・区間最大クエリに答えるセグメント木。
    /// 1-indexed・配列長 2n の反復（ボトムアップ）実装。
    #[derive(Debug, Clone)]
    pub struct SegTree<T> {
        /// 葉の数（扱う要素数）
        n: usize,
        /// tree[1] が根。葉は tree[n..2*n] に並ぶ。
        tree: Vec<T>,
    }

    impl<T: MaxIdentity> SegTree<T> {
        /// すべて単位元で初期化した、長さ n のセグ木を作る。O(n)
        pub fn new(n: usize) -> Self {
            Self {
                n,
                tree: vec![T::min_identity(); 2 * n],
            }
        }

        /// idx 番目の値を v に更新する。O(log n)
        pub fn update(&mut self, idx: usize, v: T) {
            assert!(idx < self.n);
            let mut i = idx + self.n;
            self.tree[i] = v;
            while i > 1 {
                i >>= 1;
                self.tree[i] = self.tree[2 * i].max(self.tree[2 * i + 1]);
            }
        }

        /// idx 番目の現在値を返す。O(1)
        pub fn get(&self, idx: usize) -> T {
            assert!(idx < self.n);
            self.tree[idx + self.n]
        }

        /// range の区間最大値を返す。空区間なら単位元。O(log n)
        pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> T {
            let (mut l, mut r) = self.to_half_open(range);
            let mut res = T::min_identity();
            l += self.n;
            r += self.n;
            while l < r {
                if l & 1 == 1 {
                    res = res.max(self.tree[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    res = res.max(self.tree[r]);
                }
                l >>= 1;
                r >>= 1;
            }
            res
        }

        /// RangeBounds を半開区間 [l, r) に変換する。範囲外はパニックさせる。
        fn to_half_open<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
            let l = match range.start_bound() {
                Bound::Included(&s) => s,
                Bound::Excluded(&s) => s + 1,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Included(&e) => e + 1,
                Bound::Excluded(&e) => e,
                Bound::Unbounded => self.n,
            };
            assert!(l <= r && r <= self.n);
            (l, r)
        }
    }

    impl<T: MaxIdentity> From<Vec<T>> for SegTree<T> {
        /// 既存の列からセグ木を O(n) で構築する。
        /// 単位元は T::min_identity() から得られるので Vec だけで構築できる。
        fn from(values: Vec<T>) -> Self {
            let n = values.len();
            let mut tree = vec![T::min_identity(); 2 * n];
            tree[n..2 * n].copy_from_slice(&values);
            for i in (1..n).rev() {
                tree[i] = tree[2 * i].max(tree[2 * i + 1]);
            }
            Self { n, tree }
        }
    }

    impl<T: MaxIdentity> FromIterator<T> for SegTree<T> {
        /// イテレータからセグ木を構築する。O(n)
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            Self::from(iter.into_iter().collect::<Vec<T>>())
        }
    }
}
```

（テストは Step 7 のブロックをこの下に置く）

---

## 検証（Done の条件）

1. `cd mylib && cargo test` が全モジュール緑。
2. `cargo test range_max_segtree` で Step 7 の 6 テストが通る。
3. `cargo snippet -t vscode > ../snippets/rust.json` 実行後、
   `rust.json` に `range-max-segtree` が含まれ、既存スニペットが消えていない。
4. （任意・推奨）`/verify-datastructure` で「愚直な `iter().max()`」との
   ランダム比較テストを追加し、区間最大の正当性をランダム入力で確認する。
5. `/write-operation` で `operation.md` にログ追記。

---

## ハマりどころ

- **`element` フィールドを消し忘れる**：単位元は型から取る設計なので、構造体に
  単位元を持たせない。持たせると `From` を作る意味が薄れる。
- **配列長を `2n-1` にしてしまう**（旧コードの名残）。1-indexed なら `2n`。
  `tree[1]` を根、`tree[n..2n]` を葉にすること。
- **`get_range` を閉区間で処理しようとする**。必ず半開 `[l, r)` に正規化してから
  木をたどる。旧コードの `l <= r` ループはバグの元。
- **`copy_from_slice` が使えない**と言われたら `T: Copy` 境界を確認
  （`MaxIdentity: Copy + Ord` になっているか）。
- **`macro_rules!` を `pub mod` 内で使う**のは問題ないが、
  マクロ定義は使用箇所より前に書くこと。

---

## 補足: なぜ「今の設計では From が実装不可能」だったのか

`From` トレイトのシグネチャは `fn from(value: T) -> Self` で、**引数はひとつだけ**。
旧 `SegTree` は `new(n, e)` で単位元 `e` を実行時に受け取り `element` に保存していたため、
`from(vec: Vec<T>)` の中で `e` を知る手段がなかった。

今回のように単位元を `MaxIdentity` トレイト（型に紐づく情報）へ移すと、
`from` の中で `T::min_identity()` を呼べば単位元が手に入るので、
引数が `Vec<T>` だけでも構築できるようになる。

他の選択肢（今回は採用しない）:
- `From<(Vec<T>, T)>`（タプルで単位元も渡す）
- `Option<T>` を内部表現にして単位元の概念自体を消す
- `From` を諦めて `from_vec(vec, e)` という専用コンストラクタにする
