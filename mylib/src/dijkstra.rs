use cargo_snippet::snippet;

#[snippet(name = "dijkstra")]
#[allow(clippy::module_inception)]
mod dijkstra {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    #[allow(dead_code)]
    pub fn dijkstra(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
        let n = graph.len();
        let mut dist = vec![i64::MAX; n];
        let mut hq = BinaryHeap::new();
        dist[s] = 0;
        hq.push(Reverse((dist[s], s)));

        while let Some(Reverse((c, now))) = hq.pop() {
            if dist[now] < c {
                continue;
            }

            for i in 0..graph[now].len() {
                let (nxt, w) = graph[now][i];
                if dist[nxt] > dist[now] + w {
                    dist[nxt] = dist[now] + w;
                    hq.push(Reverse((dist[nxt], nxt)))
                }
            }
        }
        dist
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let graph = vec![
            vec![(1, 2), (2, 3)],
            vec![(3, 1)],
            vec![(3, 5), (4, 4)],
            vec![(5, 3)],
            vec![(5, 2)],
            vec![],
        ];
        let dist = dijkstra::dijkstra(&graph, 0);
        let ans = vec![0, 2, 3, 3, 7, 6];
        assert_eq!(dist, ans);
    }
}
