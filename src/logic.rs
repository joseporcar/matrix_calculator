use crate::{
    functionality::{GetCol, MatrixToNum},
    Calculator,
};
use rayon::prelude::*;

fn mini_multiply(first: &Vec<f64>, second: Vec<f64>) -> f64 {
    (0..first.len()).fold(0., |t, i| t + first[i] * second[i])

}
pub trait Operations {
    fn multiply(&mut self);

    fn determinant(&mut self);
}

impl Operations for Calculator {
    fn multiply(&mut self) {
        let (first, second) = (self.matrix.to_num(), self.mult_matrix.to_num());
        self.output_matrix = vec![vec![0.; second[0].len()]; first.len()];

        self.output_matrix.par_iter_mut().enumerate().for_each(|(r, row)| {
            row.par_iter_mut()
                .enumerate()
                .for_each(|(c, element)| *element = mini_multiply(&first[r], second.get_col(c)))
        });
    }

    fn determinant(&mut self) {
        todo!()
    }
}


