use cargo_snippet::snippet;

/// H×W の2次元グリッドを保持し、90度回転を提供する汎用構造体
#[snippet(name = "grid")]
pub struct Grid<T: Clone> {
    pub h: usize,
    pub w: usize,
    pub data: Vec<Vec<T>>,
}

#[snippet("grid")]
impl<T: Clone> Grid<T> {
    /// `data` から h, w を自動算出して Grid を構築する
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let h = data.len();
        let w = if h == 0 { 0 } else { data[0].len() };
        Self { h, w, data }
    }

    /// 時計回り90度回転した新しい Grid を返す（W行×H列）
    pub fn rotate90_cw(&self) -> Self {
        if self.h == 0 || self.w == 0 {
            return Self::new(vec![]);
        }
        let new_data = (0..self.w)
            .map(|c| (0..self.h).map(|r| self.data[self.h - 1 - r][c].clone()).collect())
            .collect();
        Self::new(new_data)
    }

    /// 反時計回り90度回転した新しい Grid を返す（W行×H列）
    pub fn rotate90_ccw(&self) -> Self {
        if self.h == 0 || self.w == 0 {
            return Self::new(vec![]);
        }
        let new_data = (0..self.w)
            .map(|c| (0..self.h).map(|r| self.data[r][self.w - 1 - c].clone()).collect())
            .collect();
        Self::new(new_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Grid<char> {
        Grid::new(vec![
            vec!['A', 'B', 'C', 'D'],
            vec!['E', 'F', 'G', 'H'],
            vec!['I', 'J', 'K', 'L'],
        ])
    }

    #[test]
    fn test_rotate90_cw() {
        let g = sample().rotate90_cw();
        assert_eq!(g.h, 4);
        assert_eq!(g.w, 3);
        assert_eq!(
            g.data,
            vec![
                vec!['I', 'E', 'A'],
                vec!['J', 'F', 'B'],
                vec!['K', 'G', 'C'],
                vec!['L', 'H', 'D'],
            ]
        );
    }

    #[test]
    fn test_rotate90_ccw() {
        let g = sample().rotate90_ccw();
        assert_eq!(g.h, 4);
        assert_eq!(g.w, 3);
        assert_eq!(
            g.data,
            vec![
                vec!['D', 'H', 'L'],
                vec!['C', 'G', 'K'],
                vec!['B', 'F', 'J'],
                vec!['A', 'E', 'I'],
            ]
        );
    }

    #[test]
    fn test_empty() {
        let g: Grid<i32> = Grid::new(vec![]);
        let cw = g.rotate90_cw();
        assert_eq!(cw.h, 0);
        assert_eq!(cw.w, 0);
    }
}
