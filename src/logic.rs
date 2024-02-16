use crate::{
    functionality::{GetCol, MatrixToNum},
    Calculator,
};
use rand::Rng;
use rayon::prelude::*;

fn print_matrix(matrix: &Vec<Vec<f64>>) {
    for row in matrix {
        for el in row {
            print!("{:.5} ", el);
        }
        println!()
    }
}

pub fn big_tester(i: usize, magnitude: u32) {
    let mut rng = rand::thread_rng();
    let mag = 10_u32.pow(magnitude) as f64;

    let first = (0..i).map(|_| (0..i).map(|_| (rng.gen::<f64>() * mag).floor() / 1000.).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>();
    println!("first matrix");
    print_matrix(&first);
    let second = (0..i).map(|_| (0..i).map(|_| (rng.gen::<f64>() * mag).floor() / 1000.).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>();
    println!("second matrix");
    print_matrix(&second);
    
    let output = pure_multiply(first, second);
    println!("output: ");
    print_matrix(&output);
    // get output
    // print output


}

fn mini_multiply(first: &Vec<f64>, second: Vec<f64>) -> f64 {
    (0..first.len()).fold(0., |t, i| t + first[i] * second[i])

}
fn pure_multiply(first: Vec<Vec<f64>>, second: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut output_matrix = vec![vec![0.; second[0].len()]; first.len()];
    output_matrix.par_iter_mut().enumerate().for_each(|(r, row)| {
        row.par_iter_mut()
            .enumerate()
            .for_each(|(c, element)| *element = mini_multiply(&first[r], second.get_col(c)))
    });
    output_matrix
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


