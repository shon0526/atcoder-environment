use ac_library::dsu;
use num_traits::pow;
use proconio::{input, marker::Usize1};
use std::cmp::max;
use std::collections::{BinaryHeap, HashMap};
fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    let mut dist: Vec<i64> = vec![-1; n];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dist_pre: Vec<i64> = vec![-1; n];
    let mut dist_post: Vec<i64> = vec![-1; n];

    let mut queue = queue::Queue::new();

    queue.push(0);
    dist_pre[0] = 0;

    while queue.len() != 0 {
        if let Some(v) = queue.pop() {
            for &nv in &graph[v] {
                if dist_pre[nv] != -1 {
                    continue;
                }

                dist_pre[nv] = dist_pre[v] + 1;
                queue.push(nv);
            }
        }
    }

    let mut mu = 0;
    let mut md = 0;

    for (i, &v) in dist_pre.iter().enumerate() {
        if v > md {
            mu = i;
            md = v;
        }
    }

    queue.push(mu);
    dist_post[mu] = 0;

    while queue.len() != 0 {
        if let Some(v) = queue.pop() {
            for &nv in &graph[v] {
                if dist_post[nv] != -1 {
                    continue;
                }

                dist_post[nv] = dist_post[v] + 1;
                queue.push(nv);
            }
        }
    }

    let ans = dist_post.iter().max().unwrap() + 1;
    println!("{}", ans);
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
