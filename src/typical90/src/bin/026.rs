use itertools::Itertools;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::{
    collections::{BinaryHeap, HashMap},
    usize,
};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];

    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut q = queue::Queue::new();
    let mut dist: Vec<i64> = vec![-1; n];
    let mut cad1_vec = vec![];
    let mut cad2_vec = vec![];
    let mut start = usize::MAX;
    for i in 0..n {
        if graph[i].len() > 0 {
            start = i;
            break;
        }
    }

    q.push(start);
    dist[start] = 0;
    cad1_vec.push(start + 1);

    while let Some(v) = q.pop() {
        for &nv in &graph[v] {
            if dist[nv] != -1 {
                continue;
            }

            dist[nv] = dist[v] + 1;
            if dist[nv] % 2 == 0 {
                cad1_vec.push(nv + 1);
            } else {
                cad2_vec.push(nv + 1);
            }
            q.push(nv);
        }
    }

    let mut ans_vec = vec![];

    if cad1_vec.len() >= cad2_vec.len() {
        for i in 0..n / 2 {
            ans_vec.push(cad1_vec[i]);
        }
    } else {
        for i in 0..n / 2 {
            ans_vec.push(cad2_vec[i]);
        }
    }

    println!("{}", ans_vec.iter().join(" "));
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
