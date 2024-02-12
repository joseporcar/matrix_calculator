pub struct Matrix {
    rows: u8,
    cols: u8,
    matrix: Vec<Vec<u8>>,
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            rows: 0,
            cols: 0,
            matrix: Vec::new(),
        }
    }

    pub fn set_size(&mut self, rows: u8, cols: u8) {
        self.rows = rows;
        self.cols = cols;
    }

    pub fn update_matrix(&mut self, s_matrix: Vec<Vec<String>>) {
        if s_matrix.len() != self.rows.into() || s_matrix[0].len() != self.cols.into() {
            panic!("The string matrix is not the same size as int matrix")
        }
        self.matrix = to_num(s_matrix)
    }
}
fn to_num(matrix: Vec<Vec<String>>) -> Vec<Vec<u8>> {
    matrix
        .iter()
        .map(|row| row.iter().map(|el| el.parse().unwrap()).collect())
        .collect()
}
