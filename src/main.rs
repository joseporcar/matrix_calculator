mod logic;
use logic::Matrix;
mod theme;
use iced::alignment::Horizontal;
use iced::futures::ready;
use iced::widget::text_input::Id;
use iced::widget::text_input::State;
use iced::widget::{
    button, column, container, horizontal_space, row, text, text_input, Column, Row, Container,
};
use iced::{
    gradient, Alignment, Application, Command, Element, Length, Padding, Settings, Theme,
};

// TODO
// add a clear button

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}


trait Numeric {
    fn is_numeric(&self) -> bool;
    fn is_float(&self) -> bool;
}
impl Numeric for String {
    fn is_numeric(&self) -> bool {
        self.trim().parse::<u8>().is_ok() || self == ""
    }
    fn is_float(&self) -> bool {
        self.trim().parse::<f64>().is_ok() || self == ""
    }
}

struct Calculator {
    temp_row: String,
    temp_col: String,
    mult_row: String,
    mult_col: String,
    mode: Modes,
    matrix: Vec<Vec<String>>,
    mult_matrix: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
enum Message {
    TempRow(String),
    TempCol(String),
    SubmitRow,
    MultRow(String),
    MultCol(String),
    SubmitMRow,
    UpdatedMatrix(String, usize, usize),
    UpdatedMultMatrix(String, usize, usize),
    SubmitEntry(usize, usize),
    SubmitMEntry(usize, usize),
    Inverse,
    Multiply,
    SysEq,
    Determinant,
    Clear,
    Calculate,
}

enum Modes {
    Input,
    Inverse,
    Multiply,
    SysEq,
    Determinant,
}

impl Calculator {
    fn update_matrix_size(&mut self, is_multiplication: bool) {
        let row;
        let col;
        let matrix;
        if is_multiplication {
            row = self.mult_row.parse().unwrap_or(0);
            col = self.mult_col.parse().unwrap_or(0);
            matrix = &mut self.mult_matrix;
        } else {
            row = self.temp_row.parse().unwrap_or(0);
            col = self.temp_col.parse().unwrap_or(0);
            matrix = &mut self.matrix;
        }

        matrix.resize(row, vec!["0".to_owned(); col]);
        matrix
            .iter_mut()
            .for_each(|row| row.resize(col, "0".to_owned()))
    }
    
    fn make_matrix(&self) -> Container<Message, iced::Renderer<theme::Theme>> {

        let mut content = Column::new();

        for row in 0..self.matrix.len() {
            let mut matrix_rows = Row::new();  
            for col in 0..self.matrix[0].len() {
                matrix_rows = matrix_rows.push(
                    text_input("", &self.matrix[row][col])
                        .on_input(move |val| Message::UpdatedMatrix(val.clone(), row, col))
                        .on_submit(Message::SubmitEntry(row, col))
                        .id(Id::new(format!("{row}x{col}")))
                        .width(Length::Fixed(50.))
                        .style(theme::TextInput::Borderless),
                );
            }
            content = content.push(matrix_rows.padding(5).spacing(10));
        }
        container(content)
    }

    fn make_mult_matrix(&self) -> Container<Message, iced::Renderer<theme::Theme>> {

        let mut content = Column::new();

        for row in 0..self.mult_matrix.len() {
            let mut matrix_rows = Row::new();  
            for col in 0..self.mult_matrix[0].len() {
                matrix_rows = matrix_rows.push(
                    text_input("", &self.mult_matrix[row][col])
                        .on_input(move |val| Message::UpdatedMultMatrix(val.clone(), row, col))
                        .on_submit(Message::SubmitMEntry(row, col))
                        .id(Id::new(format!("m{row}x{col}")))
                        .width(Length::Fixed(50.))
                        .style(theme::TextInput::Borderless)
                        ,
                );
            }
            content = content.push(matrix_rows.padding(5).spacing(10));
        }
        container(content)
    }

    fn make_matrices(&self) -> Container<Message, iced::Renderer<theme::Theme>> {
        let mut matrices = row!(self.make_matrix());

        if let Modes::Multiply = self.mode {
            
            matrices = matrices.push(text("x  ")).push(self.make_mult_matrix());
        }
        container(matrices.spacing(70))
    }

    fn ask_size(&self) -> Container<Message, iced::Renderer<theme::Theme>> {
        let get_size = row![
            text_input("rows", &self.temp_row)
                .on_input(Message::TempRow)
                .width(Length::Fixed(50.))
                .on_submit(Message::SubmitRow),
            text(" x "),
            text_input("cols", &self.temp_col)
                .on_input(Message::TempCol)
                .width(Length::Fixed(50.))
                .id(Id::new("cols"))
        ];

        if let Modes::Multiply = self.mode {
            return container(get_size.push(horizontal_space(Length::Fixed(40.)))
                .push(text_input("rows", &self.mult_row)
                    .on_input(Message::MultRow)
                    .width(Length::Fixed(50.))
                    .on_submit(Message::SubmitMRow))
                .push(text(" x "))
                .push(text_input("cols", &self.mult_col)
                    .on_input(Message::MultCol)
                    .width(Length::Fixed(50.))
                    .id(Id::new("mcols")))).center_x()
        }

        container(get_size).center_x()
    }

}

impl Application for Calculator {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                temp_row: String::new(),
                temp_col: String::new(),
                mult_row: String::new(),
                mult_col: String::new(),
                mode: Modes::Input,
                matrix: Vec::default(),
                mult_matrix: Vec::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "The Matrix Calculator".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::TempRow(row) => {
                if row.is_numeric() {
                    self.temp_row = row;
                    self.update_matrix_size(false)
                };
                Command::none()
            }
            Message::TempCol(col) => {
                if col.is_numeric() {
                    self.temp_col = col;
                    self.update_matrix_size(false)
                };
                Command::none()
            }
            Message::MultRow(row) => {
                if row.is_numeric() {
                    self.mult_row = row;
                    self.update_matrix_size(true)
                };
                Command::none()
            }
            Message::MultCol(col) => {
                if col.is_numeric() {
                    self.mult_col = col;
                    self.update_matrix_size(true)
                };
                Command::none()
            }
            Message::SubmitRow => text_input::focus(Id::new("cols")),
            Message::SubmitMRow => text_input::focus(Id::new("mcols")),
            Message::UpdatedMatrix(val, row, col) => {
                if val.is_float() {self.matrix[row][col] = val};
                Command::none()
            }
            Message::UpdatedMultMatrix(val, row, col) => {
                if val.is_float() {self.mult_matrix[row][col] = val};
                Command::none()
            }
            Message::SubmitEntry(row, col) => {
                if col >= self.temp_col.parse::<usize>().unwrap() - 1 {
                    if row >= self.temp_row.parse::<usize>().unwrap() - 1 {return Command::none()}
                    else {return text_input::focus(Id::new(format!("{}x0", row+1)))}
                } 
                text_input::focus(Id::new(format!("{row}x{}", col+1)))
            },
            Message::SubmitMEntry(row, col) => {
                if col >= self.mult_col.parse::<usize>().unwrap() - 1 {
                    if row >= self.temp_row.parse::<usize>().unwrap() - 1 {return Command::none()}
                    else {return text_input::focus(Id::new(format!("m{}x0", row+1)))}
                } 
                text_input::focus(Id::new(format!("m{row}x{}", col+1)))
            },
            Message::Inverse => { self.mode = Modes::Inverse; Command::none() },
            Message::Multiply => { self.mode = Modes::Multiply; Command::none() },
            Message::SysEq => { self.mode = Modes::SysEq; Command::none() },
            Message::Determinant => { self.mode = Modes::Determinant; Command::none() },
            Message::Clear => {self.matrix.fill(vec!["0".to_owned();self.temp_col.parse().unwrap_or(0)]); self.mult_matrix.fill(vec!["0".to_owned();self.mult_col.parse().unwrap_or(0)]); Command::none()}
            Message::Calculate => { Command::none()}
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<theme::Theme>> {
        let err = "";
        let button_styles;
        {
            use theme::Button::{NotPressed, Pressed};
            button_styles = match self.mode {
                Modes::Input => [NotPressed; 4],
                Modes::Multiply => [Pressed, NotPressed, NotPressed, NotPressed],
                Modes::Inverse => [NotPressed, Pressed, NotPressed, NotPressed],
                Modes::SysEq => [NotPressed, NotPressed, Pressed, NotPressed],
                Modes::Determinant => [NotPressed, NotPressed, NotPressed, Pressed],
            };
        }
        let functions = row![
            button("Multiply").on_press(Message::Multiply).style(button_styles[0]),
            button("Inverse").on_press(Message::Inverse).style(button_styles[1]),
            button("Sys. of Equations").on_press(Message::SysEq).style(button_styles[2]),
            button("Determinant").on_press(Message::Determinant).style(button_styles[3])
        ]
        .spacing(10)
        .padding(30);
         
        let util_buttons = row!(
            button("Clear").on_press(Message::Clear).style(theme::Button::Red),
            horizontal_space(Length::Fixed(30.)),
            button("Calculate").on_press(Message::Calculate)
        );

        let content = column![
            text("What do you want to do?").size(30),
            functions,
            text("Please input the dimensions of the matrix"),
            self.ask_size().padding(20),
            self.make_matrices(),
            util_buttons,
        ]
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .padding(40)
            .center_x()
            .into()
    }
}
