use cargo_snippet::snippet;

// 区間をsetで管理するためのテクニックのモジュール化
#[allow(clippy::module_inception)]
#[snippet(name = "interval-set")]
pub mod interval_set {
    use std::collections::BTreeMap;

    // IntervalSetは整数の集合で隣り合わない半開区間の直和を管理するデータ構造
    // 例： [ [-3, -2), [-1, 2), [5, 7)] ]
    // イメージとしては [K, V) 半開区間の左側をkey, 右側をvalueで管理する意識

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct IntervalSet {
        map: BTreeMap<i64, i64>,
        length: i64,
    }

    impl Default for IntervalSet {
        fn default() -> Self {
            Self::new()
        }
    }

    impl IntervalSet {
        //からのIntervalSetを作成
        pub fn new() -> Self {
            Self {
                map: BTreeMap::new(),
                length: 0, // 全区間の長さの総和
            }
        }

        // xが区間の直和集合の中に含まれているか
        pub fn contains(&self, x: i64) -> bool {
            if let Some((&l, &r)) = self.map.range(..=x).last() {
                l <= x && x < r
            } else {
                false
            }
        }

        // [l, r) がいずれかの区間に内包されているか
        pub fn covered(&self, l: i64, r: i64) -> bool {
            assert!(l < r);
            if let Some((&p, &q)) = self.map.range(..=l).last() {
                p <= l && r <= q
            } else {
                false
            }
        }

        // p, qは同じ区間にいるのか
        pub fn same(&self, p: i64, q: i64) -> bool {
            if !self.covered(p, p + 1) || !self.covered(q, q + 1) {
                return false;
            }
            if let Some((&kp, _)) = self.map.range(..=p).last() {
                if let Some((&kq, _)) = self.map.range(..=q).last() {
                    kp == kq
                } else {
                    false
                }
            } else {
                false
            }
        }

        // [l, r)を挿入する
        pub fn insert_range(&mut self, l: i64, r: i64) {
            assert!(l <= r);
            if l == r {
                return;
            }
            let mut l_min = l;
            let mut r_max = r;
            let mut rm_key = Vec::new();

            let mut rm_length = 0;

            for (&left, &right) in self
                .map
                .range(..=r)
                .rev()
                .take_while(|(_, &right)| l <= right)
            {
                l_min = l_min.min(left);
                r_max = r_max.max(right);
                rm_key.push(left);
                rm_length += right - left;
            }

            for key in rm_key {
                self.map.remove(&key);
            }

            self.map.insert(l_min, r_max);

            let add_length = r_max - l_min;

            self.length += add_length - rm_length;
        }

        // [l, r)内の各値を集合から削除
        pub fn remove_range(&mut self, l: i64, r: i64) {
            assert!(l <= r);
            if l == r {
                return;
            }

            let mut add_v = Vec::new();
            let mut rem_v = Vec::new();
            let mut ch_length = 0;

            for (&left, &right) in self
                .map
                .range(..r)
                .rev()
                .take_while(|(_, &right)| l < right)
            {
                rem_v.push(left);
                ch_length += right - left;

                if left < l {
                    add_v.push((left, l));
                    ch_length += l - left;
                }

                if r < right {
                    add_v.push((r, right));
                    ch_length += right - r;
                }
            }

            for key in rem_v {
                self.map.remove(&key);
            }

            for (l_add, r_add) in add_v {
                self.map.insert(l_add, r_add);
            }

            self.length += ch_length;
        }

        pub fn insert(&mut self, x: i64) {
            self.insert_range(x, x + 1);
        }

        pub fn remove(&mut self, x: i64) {
            self.remove_range(x, x + 1)
        }

        // 各区間の長さの総和を返す
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> i64 {
            self.length
        }

        // 区間の数を返す
        pub fn count(&self) -> usize {
            self.map.len()
        }

        //空集合かどうか
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }

        pub fn ranges(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
            self.map.iter().map(|(&l, &r)| (l, r))
        }
    }
}

#[cfg(test)]
pub mod test {

    use super::interval_set::IntervalSet;

    #[test]
    fn test_contains() {
        let mut int_set = IntervalSet::new();
        int_set.insert_range(1, 4);
        int_set.insert_range(6, 8);

        let mut test_vec = Vec::new();
        for (l, r) in int_set.ranges() {
            test_vec.push((l, r));
        }

        assert_eq!(test_vec, vec![(1, 4), (6, 8)]);

        assert!(!int_set.contains(0));
        assert!(int_set.contains(1));
        assert!(int_set.contains(2));
        assert!(int_set.contains(3));
        assert!(!int_set.contains(4));
        assert!(int_set.contains(6));
        assert!(int_set.contains(7));
        assert!(!int_set.contains(8));
        assert!(!int_set.contains(9))
    }

    #[test]
    fn test_covered() {
        let mut interval_set = IntervalSet::new();
        interval_set.insert_range(1, 4);
        interval_set.insert_range(6, 8);

        assert!(!interval_set.covered(-1, 1));
        assert!(interval_set.covered(1, 3));
        assert!(interval_set.covered(1, 4));
        assert!(!interval_set.covered(4, 6));
        assert!(!interval_set.covered(7, 9));
    }

    #[test]
    #[should_panic]
    fn test_panic_covered() {
        let mut interval_set = IntervalSet::new();
        interval_set.insert_range(1, 4);
        assert!(interval_set.covered(3, 1));
    }

    #[test]
    fn test_same() {
        let mut interval_set = IntervalSet::new();
        interval_set.insert_range(1, 4);

        assert!(!interval_set.same(-1, 1));
        assert!(interval_set.same(1, 3));
        assert!(!interval_set.same(1, 4));
        assert!(!interval_set.same(4, 5));
    }

    #[test]
    fn test_ranges() {
        let mut interval_set = IntervalSet::new();
        interval_set.insert_range(1, 4);
        interval_set.insert_range(6, 8);
        assert_eq!(
            interval_set.ranges().collect::<Vec<(i64, i64)>>(),
            vec![(1, 4), (6, 8)]
        );
    }

    #[test]
    fn test_insert_range() {
        let mut interval_set = IntervalSet::new();

        interval_set.insert_range(1, 4);
        assert_eq!(interval_set.count(), 1);
        assert_eq!(interval_set.len(), 3);
        assert_eq!(
            interval_set.ranges().collect::<Vec<(i64, i64)>>(),
            vec![(1, 4)]
        );

        interval_set.insert_range(6, 8);

        assert_eq!(interval_set.count(), 2);
        assert_eq!(interval_set.len(), 5);
        assert_eq!(
            interval_set.ranges().collect::<Vec<(i64, i64)>>(),
            vec![(1, 4), (6, 8)]
        );

        interval_set.insert_range(-1, 2);
        assert_eq!(interval_set.count(), 2);
        assert_eq!(interval_set.len(), 7);
        assert_eq!(
            interval_set.ranges().collect::<Vec<(i64, i64)>>(),
            vec![(-1, 4), (6, 8)]
        );

        interval_set.insert_range(-3, -1);
        assert_eq!(interval_set.count(), 2);
        assert_eq!(interval_set.len(), 9);
        assert_eq!(
            interval_set.ranges().collect::<Vec<(i64, i64)>>(),
            vec![(-3, 4), (6, 8)]
        );

        interval_set.insert_range(4, 6);
        assert_eq!(interval_set.count(), 1);
        assert_eq!(interval_set.len(), 11);
        assert_eq!(
            interval_set.ranges().collect::<Vec<(i64, i64)>>(),
            vec![(-3, 8)]
        );
    }
}
