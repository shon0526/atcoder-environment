#![allow(unused_imports, dead_code)]
use ac_library::{Monoid, Segtree, segtree};
use bitvec::ptr::swap;
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::marker::PhantomData;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut p: [i64; n],
        lr: [(Usize1, Usize1); m],
    }
    let mut segtree_min: Segtree<MinPlusId<(usize, usize)>> = Segtree::new(n);
    let mut segtree_max: Segtree<MaxPlusId<(usize, usize)>> = Segtree::new(n);

    for (i, &v) in p.iter().enumerate() {
        segtree_min.set(i, (v, i as i64));
        segtree_max.set(i, (v, i as i64));
    }

    for (l, r) in lr {
        let (v_min, min_id) = segtree_min.prod(l..=r);
        let (v_max, max_id) = segtree_max.prod(l..=r);

        p.swap(min_id as usize, max_id as usize);
        segtree_min.set(min_id as usize, (v_max, min_id));
        segtree_min.set(max_id as usize, (v_min, max_id));
        segtree_max.set(min_id as usize, (v_max, min_id));
        segtree_max.set(max_id as usize, (v_min, max_id));
    }
    println!("{}", p.iter().join(" "));
}

pub struct MinPlusId<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for MinPlusId<S> {
    type S = (i64, i64);
    fn identity() -> Self::S {
        (200_001, -1)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.0 < b.0 { *a } else { *b }
    }
}

pub struct MaxPlusId<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for MaxPlusId<S> {
    type S = (i64, i64);
    fn identity() -> Self::S {
        (-1, -1)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.0 > b.0 { *a } else { *b }
    }
}
