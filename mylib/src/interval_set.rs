use cargo_snippet::snippet;

// 区間をsetで管理するためのテクニックのモジュール化

#[snippet(name = "interval-set")]
pub mod interval_set {
    use std::collections::BTreeMap;

    // IntervalSetは整数の集合で隣り合わない半開区間の直和を管理するデータ構造
    // 例： [ [-3, -2), [-1, 2), [5, 7)] ]
    // イメージとしては [K, V) 半開区間の左側をkey, 右側をvalueで管理する意識

    #[derive(Debug)]
    pub struct IntervalSet {
        map: BTreeMap<i64, i64>,
        length: i64,
    }

    impl Default for IntervalSet {
        fn default() -> Self {
            Seld::new()
        }
    }

    impl IntervalSet {
        //からのIntervalSetを作成
        fn new() -> Self {
            Self {
                map: BTreeMap::new(),
                length: 0,
            }
        }
    }
}
