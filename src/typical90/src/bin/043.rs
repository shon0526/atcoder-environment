use itertools::Itertools;
use num_traits::pow;
use pathfinding::matrix::directions::W;
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{BinaryHeap, HashMap, VecDeque};

const WAY: [(i64, i64); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: usize,
        cs: usize,
        rl: usize,
        cl: usize,
        s: [Chars; h],
    }

    let mut grid = vec![vec!['#'; w + 2]; h + 2];
    for i in 1..=h {
        for j in 1..=w {
            if s[i - 1][j - 1] == '.' {
                grid[i][j] = '.';
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut way_count = vec![vec![vec![usize::MAX - 1; 4]; w + 2]; h + 2];
    way_count[rs][cs][0] = 0;
    way_count[rs][cs][1] = 0;
    way_count[rs][cs][2] = 0;
    way_count[rs][cs][3] = 0;
    for i in 0..4 {
        let (x, y) = WAY[i];
        let nx = rs as i64 + x;
        let ny = cs as i64 + y;

        if nx < 1 || nx > h as i64 || ny < 1 || ny > w as i64 {
            continue;
        }

        if grid[nx as usize][ny as usize] == '#' {
            continue;
        }

        queue.push_back((nx, ny, i));
        way_count[nx as usize][ny as usize][i] = 0;
    }

    while let Some((pre_x, pre_y, way)) = queue.pop_front() {
        for i in 0..4 {
            let (x, y) = WAY[i];
            let nx = pre_x + x;
            let ny = pre_y + y;

            let cost: usize = if way == i { 0 } else { 1 };
            if nx < 1 || nx > h as i64 || ny < 1 || ny > w as i64 {
                continue;
            }
            if grid[nx as usize][ny as usize] == '#' {
                continue;
            }
            if way_count[nx as usize][ny as usize][i]
                > way_count[pre_x as usize][pre_y as usize][way] + cost
            {
                way_count[nx as usize][ny as usize][i] =
                    way_count[pre_x as usize][pre_y as usize][way] + cost;
                if cost == 0 {
                    queue.push_front((nx, ny, i));
                } else {
                    queue.push_back((nx, ny, i));
                }
            }
        }
    }

    println!("{}", way_count[rl][cl].iter().min().unwrap());
    //
    // for i in 0..h + 2 {
    //     println!("{}", grid[i].iter().join(""));
    // }
}

pub mod queue {
    use std::collections::{vec_deque, VecDeque};
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
