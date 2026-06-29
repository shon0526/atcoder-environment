use cargo_snippet::snippet;

/// H x W の2次元グリッドを保持する構造体。
#[snippet(name = "grid")]
pub struct Grid<T: Clone> {
    pub h: usize,
    pub w: usize,
    pub data: Vec<Vec<T>>,
}

#[snippet("grid")]
impl<T: Clone> Grid<T> {
    /// 2次元配列から高さと幅を自動計算してグリッドを作る。
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let h = data.len();
        let w = data.first().map_or(0, Vec::len);
        Self { h, w, data }
    }

    /// グリッドを時計回りに90度回転した新しいグリッドを返す。
    pub fn rotate90_cw(&self) -> Self {
        if self.h == 0 || self.w == 0 {
            return Self::new(Vec::new());
        }

        let mut data = vec![vec![self.data[0][0].clone(); self.h]; self.w];
        for row in 0..self.h {
            for col in 0..self.w {
                data[col][self.h - 1 - row] = self.data[row][col].clone();
            }
        }
        Self::new(data)
    }

    /// グリッドを反時計回りに90度回転した新しいグリッドを返す。
    pub fn rotate90_ccw(&self) -> Self {
        if self.h == 0 || self.w == 0 {
            return Self::new(Vec::new());
        }

        let mut data = vec![vec![self.data[0][0].clone(); self.h]; self.w];
        for row in 0..self.h {
            for col in 0..self.w {
                data[self.w - 1 - col][row] = self.data[row][col].clone();
            }
        }
        Self::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn rotate90_cw_matches_coordinate_rule() {
        let grid = Grid::new(vec![
            vec!['A', 'B', 'C', 'D'],
            vec!['E', 'F', 'G', 'H'],
            vec!['I', 'J', 'K', 'L'],
        ]);

        let rotated = grid.rotate90_cw();

        assert_eq!(rotated.h, 4);
        assert_eq!(rotated.w, 3);
        assert_eq!(
            rotated.data,
            vec![
                vec!['I', 'E', 'A'],
                vec!['J', 'F', 'B'],
                vec!['K', 'G', 'C'],
                vec!['L', 'H', 'D'],
            ]
        );
    }

    #[test]
    fn rotate90_ccw_matches_coordinate_rule() {
        let grid = Grid::new(vec![
            vec!['A', 'B', 'C', 'D'],
            vec!['E', 'F', 'G', 'H'],
            vec!['I', 'J', 'K', 'L'],
        ]);

        let rotated = grid.rotate90_ccw();

        assert_eq!(rotated.h, 4);
        assert_eq!(rotated.w, 3);
        assert_eq!(
            rotated.data,
            vec![
                vec!['D', 'H', 'L'],
                vec!['C', 'G', 'K'],
                vec!['B', 'F', 'J'],
                vec!['A', 'E', 'I'],
            ]
        );
    }

    #[test]
    fn empty_grid_stays_empty_after_rotation() {
        let grid: Grid<i32> = Grid::new(Vec::new());

        assert_eq!(grid.h, 0);
        assert_eq!(grid.w, 0);
        assert!(grid.data.is_empty());

        let rotated_cw = grid.rotate90_cw();
        assert_eq!(rotated_cw.h, 0);
        assert_eq!(rotated_cw.w, 0);
        assert!(rotated_cw.data.is_empty());

        let rotated_ccw = grid.rotate90_ccw();
        assert_eq!(rotated_ccw.h, 0);
        assert_eq!(rotated_ccw.w, 0);
        assert!(rotated_ccw.data.is_empty());
    }
}
