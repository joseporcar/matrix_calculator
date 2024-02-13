// pub struct Matrix {
//     rows: u8,
//     cols: u8,
//     matrix: Vec<Vec<u8>>,
// }

// impl Matrix {
//     pub fn new() -> Matrix {
//         Matrix {
//             rows: 0,
//             cols: 0,
//             matrix: Vec::new(),
//         }
//     }

//     pub fn set_size(&mut self, rows: u8, cols: u8) {
//         self.rows = rows;
//         self.cols = cols;
//     }

//     pub fn update_matrix(&mut self, s_matrix: Vec<Vec<String>>) {
//         if s_matrix.len() != self.rows.into() || s_matrix[0].len() != self.cols.into() {
//             panic!("The string matrix is not the same size as int matrix")
//         }
//         self.matrix = to_num(s_matrix)
//     }
// }
// fn to_num(matrix: Vec<Vec<String>>) -> Vec<Vec<u8>> {
//     matrix
//         .iter()
//         .map(|row| row.iter().map(|el| el.parse().unwrap()).collect())
//         .collect()
// }

use crate::{
    functionality::{GetCol, MatrixToNum},
    Calculator,
};

fn mini_multiply(first: &Vec<f64>, second: Vec<f64>) -> f64 {
    let mut total = 0.;
    for i in 0..first.len() {
        total += first[i] * second[i]
    }
    total
}
pub trait Operations {
    fn multiply(&self) -> Vec<Vec<f64>>;
}

impl Operations for Calculator {
    fn multiply(&self) -> Vec<Vec<f64>> {
        let (first, second) = (self.matrix.to_num(), self.mult_matrix.to_num());
        let mut result = vec![vec![0.; second[0].len()]; first.len()];

        for row in 0..result.len() {
            for col in 0..result[0].len() {
                result[row][col] = mini_multiply(&first[row], second.get_col(col))
            }
        }
        result
    }
}
