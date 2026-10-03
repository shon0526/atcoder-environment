use cargo_snippet::snippet;

#[snippet(name = "scc")]
pub mod scc {
    // 有向グラフに対しKosaraju法で強連結成分分解を行う
    #[derive(Debug)]
    pub struct Scc {
        g: Vec<Vec<usize>>,
        g_rev: Vec<Vec<usize>>,
        stack: Vec<usize>,
        seen: Vec<bool>,
    }

    impl Scc {
        // 頂点数nと有向辺の一覧からグラフと逆辺グラフを構築する
        pub fn new(n: usize, edges: &Vec<(usize, usize)>) -> Self {
            let mut g = vec![vec![]; n];
            let mut g_rev = vec![vec![]; n];

            for &(u, v) in edges {
                g[u].push(v);
                g_rev[v].push(u);
            }

            Self {
                g,
                g_rev,
                stack: Vec::new(),
                seen: vec![false; n],
            }
        }

        // 帰りがけ順をstackに積む（Kosaraju法・第1回DFS）
        fn dfs(&mut self, v: usize) {
            self.seen[v] = true;

            for i in 0..self.g[v].len() {
                let w = self.g[v][i];
                if self.seen[w] {
                    continue;
                }
                self.dfs(w);
                self.stack.push(w);
            }
        }

        // 逆辺グラフ上でDFSし、到達範囲を1つの強連結成分として集める
        fn dfs_rv(&mut self, scc: &mut Vec<usize>, v: usize) {
            self.seen[v] = true;

            for i in 0..self.g_rev[v].len() {
                let w = self.g_rev[v][i];
                if self.seen[w] {
                    continue;
                }
                self.dfs_rv(scc, w);
                scc.push(w);
            }
        }

        // 強連結成分ごとの頂点集合を、縮約後のトポロジカル順で返す
        pub fn scc(&mut self) -> Vec<Vec<usize>> {
            for v in 0..self.g.len() {
                if !self.seen[v] {
                    self.dfs(v);
                    self.stack.push(v);
                }
            }

            self.seen = vec![false; self.g.len()];
            let mut sccs = Vec::new();

            while let Some(v) = self.stack.pop() {
                if !self.seen[v] {
                    let mut scc = Vec::new();

                    self.dfs_rv(&mut scc, v);
                    scc.push(v);
                    sccs.push(scc);
                }
            }

            sccs
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::scc;

    #[test]
    fn test_non_cycle() {
        let n = 3;
        let edges = vec![(0, 1), (1, 2)];
        let mut graph = scc::Scc::new(n, &edges);
        let scc = graph.scc();

        assert_eq!(scc, vec![vec![0], vec![1], vec![2]]);
    }

    #[test]
    fn test_cycle() {
        let n = 6;
        let edges = vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];

        let mut graph = scc::Scc::new(n, &edges);
        let scc = graph.scc();

        assert_eq!(scc.len(), 4);
        assert_eq!(scc, vec![vec![5], vec![4, 1], vec![2], vec![3, 0]]);
    }
}
