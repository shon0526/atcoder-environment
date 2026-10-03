#![allow(unused_imports, dead_code)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 畳をおくパターンは３種類 (一枚おく, 右方向に２枚おく, 下方向に２枚おく)
// (0,0)から順に畳を敷き詰めていく j方向で考えていく
// 全て埋めきると終了

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    }
    let mut res = 0;
    dfs(0, 0, a, b, h, w, &mut res);
    println!("{}", res);
}

// i: 今から確認するますの番号
// bit: 埋まっているますの情報
// a: 1 x 2 の畳の使用可能枚数
// b: 1 x 1 の畳の使用可能枚数
//
fn dfs(i: usize, bit: usize, a: usize, b: usize, h: usize, w: usize, ans: &mut usize) {
    if i == h * w {
        *ans += 1;
        return;
    }

    if bit & (1 << i) != 0 {
        dfs(i + 1, bit, a, b, h, w, ans);
    }

    if b >= 1 {
        dfs(i + 1, bit | 1 << i, a, b - 1, h, w, ans);
    }

    if a >= 1 {
        // グリッドの右端の場合は実行しない
        if i % w != w - 1 && (bit & (1 << (i + 1))) == 0 {
            dfs(i + 1, (bit | 1 << i) | 1 << (i + 1), a - 1, b, h, w, ans);
        }

        // グリッドの行がh-1を超える場合は実行しない
        if i + w < h * w && (bit & (1 << (i + w))) == 0 {
            dfs(i + 1, (bit | 1 << i) | (1 << (i + w)), a - 1, b, h, w, ans);
        }
    }
}

//
// fn dfs(
//     seen: &mut Vec<Vec<bool>>,
//     u: usize,
//     v: usize,
//     h: usize,
//     w: usize,
//     mut a: usize,
//     mut b: usize,
//     res: &mut usize,
// ) {
//     if u == h && v == 0 {
//         *res += 1;
//         return;
//     }
//
//     let (nx, ny) = if v == w - 1 { (u + 1, 0) } else { (u, v + 1) };
//
//     if seen[u][v] {
//         dfs(seen, nx, ny, h, w, a, b, res);
//     } else {
//         seen[u][v] = true;
//         if b >= 1 {
//             dfs(seen, nx, ny, h, w, a, b - 1, res);
//         }
//
//         if a >= 1 {
//             if v != w - 1 && !seen[u][v + 1] {
//                 seen[u][v + 1] = true;
//                 dfs(seen, nx, ny, h, w, a - 1, b, res);
//                 seen[u][v + 1] = false;
//             }
//             if u != h - 1 && !seen[u + 1][v] {
//                 seen[u + 1][v] = true;
//                 dfs(seen, nx, ny, h, w, a - 1, b, res);
//                 seen[u + 1][v] = false;
//             }
//         }
//         seen[u][v] = false;
//     }
// }
