use std::process;

use itertools::Itertools;
use proconio::input;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      k: usize,
      mut a: [usize; n],
    }
    a.sort();
    let rlc = rlc(&a);

    if rlc.len() <= k {
        println!("{}", 0);
        process::exit(0);
    }

    let mut rlc = rlc.iter().cloned().map(|(x, y)| x * y).collect_vec();
    rlc.sort_by(|a, b| b.cmp(&a));
    let mut queue = queue::Queue::from(rlc);
    let mut count = 1;
    while count <= k {
        queue.pop();
        count += 1;
    }

    let mut ans = 0;
    while let Some(v) = queue.pop() {
        ans += v;
    }

    println!("{}", ans);
}
fn rlc(s: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut i = 0;
    let mut ctr = vec![];
    let mut cur = (s[0], 0);
    loop {
        while i < s.len() && s[i] == cur.0 {
            cur.1 += 1;
            i += 1;
        }
        ctr.push(cur);
        if i == s.len() {
            break;
        } else {
            cur = (s[i], 0);
        }
    }
    ctr
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
