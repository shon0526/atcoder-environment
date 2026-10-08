#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

const WAY: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const INF: usize = usize::MAX;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    let mut bom_x = HashSet::new();
    let mut bom_y = HashSet::new();

    // 爆弾の位置を把握
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                bom_x.insert(i);
                bom_y.insert(j);
            }
        }
    }

    let mut safe = Vec::new();

    //安全マスを把握
    for i in 0..h {
        for j in 0..w {
            if !bom_x.contains(&i) && !bom_y.contains(&j) {
                safe.push((i, j));
            }
        }
    }

    let mut dist: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    let mut queue = queue::Queue::new();
    for &(i, j) in &safe {
        queue.push((i, j));
        dist[i][j] = 0;
    }

    while let Some((x, y)) = queue.pop() {
        for &(dx, dy) in &WAY {
            let nx = x.wrapping_add_signed(dx);
            let ny = y.wrapping_add_signed(dy);

            if nx == usize::MAX || nx > h - 1 || ny == usize::MAX || ny > w - 1 || s[nx][ny] == '#'
            {
                continue;
            }

            if dist[nx][ny] != INF {
                continue;
            }
            dist[nx][ny] = dist[x][y] + 1;

            queue.push((nx, ny));
        }
    }

    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' && dist[i][j] <= k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

pub mod queue {
    use std::collections::{VecDeque, vec_deque};
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Queue<T> {
        data: VecDeque<T>,
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
    impl<T> From<Vec<T>> for Queue<T> {
        fn from(vec: Vec<T>) -> Self {
            Self {
                data: VecDeque::from(vec),
            }
        }
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Self {
                data: VecDeque::new(),
            }
        }
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                data: VecDeque::with_capacity(capacity),
            }
        }
        pub fn push(&mut self, value: T) {
            self.data.push_back(value);
        }
        pub fn pop(&mut self) -> Option<T> {
            self.data.pop_front()
        }
        pub fn front(&self) -> Option<&T> {
            self.data.front()
        }
        pub fn back(&self) -> Option<&T> {
            self.data.back()
        }
        pub fn len(&self) -> usize {
            self.data.len()
        }
        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
        pub fn clear(&mut self) {
            self.data.clear();
        }
        pub fn iter(&self) -> vec_deque::Iter<'_, T> {
            self.data.iter()
        }
    }
    impl<T> FromIterator<T> for Queue<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Queue<T> {
            Self {
                data: iter.into_iter().collect(),
            }
        }
    }
    impl<T> IntoIterator for Queue<T> {
        type Item = T;
        type IntoIter = vec_deque::IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }
    impl<'a, T> IntoIterator for &'a Queue<T> {
        type Item = &'a T;
        type IntoIter = vec_deque::Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }
}
