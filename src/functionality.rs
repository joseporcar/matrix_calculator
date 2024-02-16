use iced::{
    widget::{
        container, horizontal_space, row, rule, scrollable, text, text_input::{Id}, Column, Container, Row, Scrollable, text_input
    },
    Length,
};

use crate::{theme, Calculator, Message, Modes};

const TEXT_WIDTH: f32 = 8.;
pub trait MatrixVisualizing {
    fn update_matrix_size(&mut self, is_multiplication: bool);

    fn make_matrix(&self) -> Container<Message, iced::Renderer<theme::Theme>>;

    fn make_mult_matrix(&self) -> Container<Message, iced::Renderer<theme::Theme>>;

    fn make_output_matrix(&self) -> Container<Message, iced::Renderer<theme::Theme>>;

    fn make_matrices(&self) -> Container<Message, iced::Renderer<theme::Theme>>;

    fn longest_value(&self) -> f32;

    fn mult_longest_value(&self) -> f32;

    fn out_longest_value(&self) -> f32;
}

impl MatrixVisualizing for Calculator {
    fn update_matrix_size(&mut self, is_multiplication: bool) {
        let row;
        let col;
        let matrix;
        if is_multiplication {
            row = self.mult_row.parse().unwrap_or(0);
            col = self.mult_col.parse().unwrap_or(0);
            matrix = &mut self.mult_matrix;
        } else {
            row = self.row.parse().unwrap_or(0);
            col = self.col.parse().unwrap_or(0);
            matrix = &mut self.matrix;
        }

        matrix.resize(row, vec!["0".to_owned(); col]);
        matrix
            .iter_mut()
            .for_each(|row| row.resize(col, "0".to_owned()))
    }

    fn make_matrix(&self) -> Container<Message, iced::Renderer<theme::Theme>> {
        let mut content = Column::new();

        let width = self.longest_value();
        for row in 0..self.matrix.len() {
            let mut matrix_rows = Row::new();
            for col in 0..self.matrix[0].len() {
                matrix_rows = matrix_rows.push(
                    text_input("", &self.matrix[row][col])
                        .on_input(move |val| Message::UpdatedMatrix(val.clone(), row, col))
                        .on_submit(Message::SubmitEntry(row, col))
                        .id(Id::new(format!("{row}x{col}")))
                        .width(Length::Fixed(TEXT_WIDTH * width + 12.))
                        .line_height(1.)
                        .style(theme::TextInput::Borderless),
                );
            }
            content = content.push(matrix_rows.padding(5).spacing(10));
        }
        container(content)
    }

    fn make_mult_matrix(&self) -> Container<Message, iced::Renderer<theme::Theme>> {
        let mut content = Column::new();

        let width = self.mult_longest_value();
        for row in 0..self.mult_matrix.len() {
            let mut matrix_rows = Row::new();
            for col in 0..self.mult_matrix[0].len() {
                matrix_rows = matrix_rows.push(
                    text_input("", &self.mult_matrix[row][col])
                        .on_input(move |val| Message::UpdatedMultMatrix(val.clone(), row, col))
                        .on_submit(Message::SubmitMEntry(row, col))
                        .id(Id::new(format!("m{row}x{col}")))
                        .width(Length::Fixed(TEXT_WIDTH * width + 12.))
                        .line_height(1.)
                        .style(theme::TextInput::Borderless),
                );
            }
            content = content.push(matrix_rows.padding(5).spacing(10));
        }
        container(content)
    }

    fn make_output_matrix(&self) -> Container<Message, iced::Renderer<theme::Theme>> {
        let mut content = Column::new();

        let width = self.out_longest_value();
        for row in 0..self.output_matrix.len() {
            let mut matrix_rows = Row::new();
            for col in 0..self.output_matrix[0].len() {
                matrix_rows =
                    matrix_rows.push(text(&self.output_matrix[row][col]).width(TEXT_WIDTH * width));
            }
            content = content.push(matrix_rows.padding(7.5).spacing(22));
        }
        container(content)
    }

    fn make_matrices(&self) -> Container<Message, iced::Renderer<theme::Theme>> {
        //let rule = rule::Rule::vertical(100 //self.temp_row.parse::<u16>().unwr);
        let mut matrices = row!(self.make_matrix());

        if let Modes::Multiply = self.mode {
            matrices = matrices.push(text(" x ")).push(self.make_mult_matrix());
        }
        if self.output_matrix != Vec::<Vec<f64>>::default() {
            matrices = matrices.push(text(" = ")).push(self.make_output_matrix());
        }
        container(matrices.spacing(10)) //.height(u16::max(self.mult_row.parse::<u16>().unwrap_or(0), self.temp_row.parse::<u16>().unwrap_or(0)) * 30 + 50)
    }

    fn longest_value(&self) -> f32 {
        self.matrix.iter().fold(0., |l, v| {
            l.max(v.iter().fold(0., |l, v| l.max(v.len() as f32)))
        })
    }

    fn mult_longest_value(&self) -> f32 {
        self.mult_matrix.iter().fold(0., |l, v| {
            l.max(v.iter().fold(0., |l, v| l.max(v.len() as f32)))
        })
    }

    fn out_longest_value(&self) -> f32 {
        self.output_matrix.iter().fold(0., |l, v| {
            l.max(v.iter().fold(0., |l, v| l.max(v.to_string().len() as f32)))
        })
    }
}

pub trait MakeSizeInput {
    fn make_size_input(&self) -> Container<Message, iced::Renderer<theme::Theme>>;
}
impl MakeSizeInput for Calculator {
    fn make_size_input(&self) -> Container<Message, iced::Renderer<theme::Theme>> {
        let get_size = row![
            text_input("rows", &self.row)
                .on_input(Message::TempRow)
                .width(Length::Fixed(50.))
                .on_submit(Message::SubmitRow),
            text(" x "),
            text_input("cols", &self.col)
                .on_input(Message::TempCol)
                .width(Length::Fixed(50.))
                .id(Id::new("cols"))
        ];

        if let Modes::Multiply = self.mode {
            return container(
                get_size
                    .push(horizontal_space(Length::Fixed(40.)))
                    .push(
                        text_input("rows", &self.mult_row)
                            .on_input(Message::MultRow)
                            .width(Length::Fixed(50.))
                            .on_submit(Message::SubmitMRow),
                    )
                    .push(text(" x "))
                    .push(
                        text_input("cols", &self.mult_col)
                            .on_input(Message::MultCol)
                            .width(Length::Fixed(50.))
                            .id(Id::new("mcols")),
                    ),
            )
            .center_x();
        }

        container(get_size).center_x()
    }
}

pub trait MatrixToNum {
    fn to_num(&self) -> Vec<Vec<f64>>;
}
impl MatrixToNum for Vec<Vec<String>> {
    fn to_num(&self) -> Vec<Vec<f64>> {
        self.iter()
            .map(|row| row.iter().map(|s| s.parse::<f64>().unwrap_or(0.)).collect())
            .collect()
    }
}
pub trait GetCol<T> {
    fn get_col(&self, index: usize) -> Vec<T>;
}
impl GetCol<f64> for Vec<Vec<f64>> {
    fn get_col(&self, index: usize) -> Vec<f64> {
        self.iter().map(|x| x[index]).collect()
    }
}
pub trait BasicCalcFunctionality {
    fn get_error(&self) -> String;
}
impl BasicCalcFunctionality for Calculator {
    fn get_error(&self) -> String {
        match self.mode {
            Modes::Input => String::new(),
            Modes::Inverse => {
                if self.row != self.col {
                    "You need a square matrix to find inverse".to_owned()
                } else {
                    String::new()
                }
            }
            Modes::Multiply => {
                if self.col != self.mult_row {
                    "You need to have as many colums in the first matrix as rows in the second one"
                        .to_owned()
                } else {
                    String::new()
                }
            }
            Modes::SysEq => String::new(),
            Modes::Determinant => String::new(),
        }
    }
}


trait UpdateFunctionality {

}