use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut vs = vec![0; n + 1];

    let mut set = btree_multiset::MultiSet::new();

    for &x in &a {
        vs[x] += 1;
    }

    for (i, &v) in vs.iter().enumerate() {
        if v > 0 {
            set.insert(v);
        }
    }

    let mut ans: usize = 0;

    while set.size() > k {
        if let Some(v) = set.pop_first() {
            ans += v;
        }
    }

    println!("{}", ans);
}

pub mod btree_multiset {
    use std::{
        borrow::Borrow,
        collections::{
            btree_map::{self},
            BTreeMap,
        },
        ops::{Bound, RangeBounds},
    };
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MultiSet<T> {
        size: usize,
        map: BTreeMap<T, usize>,
    }
    impl<T: Ord> Default for MultiSet<T> {
        fn default() -> Self {
            Self::new()
        }
    }
    impl<T: Ord> From<Vec<T>> for MultiSet<T> {
        fn from(vec: Vec<T>) -> Self {
            let size = vec.len();
            let mut btree_map = BTreeMap::new();
            for key in vec {
                *btree_map.entry(key).or_insert(0) += 1;
            }
            Self {
                size: size,
                map: btree_map,
            }
        }
    }
    impl<T: Ord> MultiSet<T> {
        pub fn new() -> Self {
            Self {
                size: 0,
                map: BTreeMap::new(),
            }
        }
        pub fn clear(&mut self) {
            self.size = 0;
            self.map.clear();
        }
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.map.contains_key(value)
        }
        pub fn insert(&mut self, key: T) {
            self.size += 1;
            *self.map.entry(key).or_insert(0) += 1;
        }
        pub fn first(&self) -> Option<&T> {
            if let Some((key, _)) = self.map.iter().next() {
                Some(key)
            } else {
                None
            }
        }
        pub fn last(&self) -> Option<&T> {
            if let Some((key, _)) = self.map.iter().next_back() {
                Some(key)
            } else {
                None
            }
        }
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }
        pub fn len(&self) -> usize {
            self.map.len()
        }
        pub fn size(&self) -> usize {
            self.size
        }
        pub fn merge(&mut self, other: &mut MultiSet<T>)
        where
            T: Clone,
        {
            self.size += other.size;
            for (key, value) in other.map.iter() {
                if let Some(prev) = self.map.get_mut(key) {
                    *prev += value;
                } else {
                    self.map.insert(key.clone(), *value);
                }
            }
        }
        pub fn pop_first(&mut self) -> Option<T>
        where
            T: Ord + Clone,
        {
            let mut entry = self.map.first_entry()?;
            self.size -= 1;
            if *entry.get() > 1 {
                *entry.get_mut() -= 1;
                Some(entry.key().clone())
            } else {
                let (key, _) = entry.remove_entry();
                Some(key)
            }
        }
        pub fn pop_last(&mut self) -> Option<T>
        where
            T: Ord + Clone,
        {
            let mut entry = self.map.last_entry()?;
            self.size -= 1;
            if *entry.get() > 1 {
                *entry.get_mut() -= 1;
                Some(entry.key().clone())
            } else {
                let (key, _) = entry.remove_entry();
                Some(key)
            }
        }
        pub fn remove(&mut self, value: &T) -> bool
        where
            T: Clone,
        {
            self.map.entry(value.clone()).and_modify(|v| *v -= 1);
            if let Some(&cnt) = self.map.get(&value) {
                if cnt == 0 {
                    self.map.remove(&value);
                }
                self.size -= 1;
                true
            } else {
                false
            }
        }
        pub fn lower_bound<Q>(&self, bound: Bound<&Q>) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord,
        {
            match bound {
                Bound::Unbounded => unreachable!(),
                _ => {
                    if let Some((key, _)) = self.map.range((bound, Bound::Unbounded)).next() {
                        Some(key)
                    } else {
                        None
                    }
                }
            }
        }
        pub fn upper_bound<Q>(&self, bound: Bound<&Q>) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: Ord,
        {
            match bound {
                Bound::Unbounded => unreachable!(),
                _ => {
                    if let Some((key, _)) = self.map.range((Bound::Unbounded, bound)).next_back() {
                        Some(key)
                    } else {
                        None
                    }
                }
            }
        }
        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                range: self.range(..),
            }
        }
        pub fn range<U: ?Sized, R>(&self, range: R) -> Range<'_, T>
        where
            U: Ord,
            T: Borrow<U> + Ord,
            R: RangeBounds<U>,
        {
            Range {
                last: None,
                counter: 0,
                range: self.map.range(range),
            }
        }
        pub fn count<Q>(&self, key: &Q) -> usize
        where
            T: Borrow<Q>,
            Q: Ord,
        {
            if let Some(&cnt) = self.map.get(key) {
                cnt
            } else {
                0
            }
        }
    }
    impl<T: Ord> FromIterator<T> for MultiSet<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> MultiSet<T> {
            let mut set = MultiSet::new();
            for v in iter {
                set.insert(v);
            }
            set
        }
    }
    #[derive(Debug, Clone, Default)]
    pub struct Range<'a, T>
    where
        T: 'a,
    {
        last: Option<&'a T>,
        counter: usize,
        range: btree_map::Range<'a, T, usize>,
    }
    impl<'a, T> Iterator for Range<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.counter == 0 {
                if let Some((key, cnt)) = self.range.next() {
                    self.last = Some(key);
                    self.counter = cnt - 1;
                    Some(key)
                } else {
                    None
                }
            } else {
                self.counter -= 1;
                self.last
            }
        }
    }
    impl<'a, T> DoubleEndedIterator for Range<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.counter == 0 {
                if let Some((key, &cnt)) = self.range.next_back() {
                    self.last = Some(key);
                    self.counter = cnt - 1;
                    Some(key)
                } else {
                    None
                }
            } else {
                self.counter -= 1;
                self.last
            }
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Iter<'a, T>
    where
        T: 'a,
    {
        range: Range<'a, T>,
    }
    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.range.next()
        }
    }
    impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.range.next_back()
        }
    }
}

#[macro_export]
macro_rules! define_queries {
  ($( $(#[$attr:meta])* enum $enum_name:ident : $sig:ty { $( $pattern:pat => $variant:ident $( { $($name:ident : $marker:ty $(,)?),* } )? $(,)?),* } )*) => {
    $(
      $(#[$attr])*
      enum $enum_name {
        $(
          $variant $( {
            $( $name : <$marker as proconio::source::Readable>::Output ),*
          } )?
        ),*
      }

      impl proconio::source::Readable for $enum_name {
        type Output = Self;
        fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self {
          #![allow(unreachable_patterns)]
          match <$sig as proconio::source::Readable>::read(source) {
            $(
              $pattern => $enum_name::$variant $( {
                $( $name: <$marker as proconio::source::Readable>::read(source) ),*
              } )?
            ),*
            , _ => unreachable!()
          }
        }
      }
    )*
  }
}
