mod logic;
use iced::Color;
use logic::Operations;
mod functionality;
mod theme;
use functionality::{MakeSizeInput, MatrixVisualizing, BasicCalcFunctionality};
use iced::alignment::Horizontal;
use iced::futures::ready;
use iced::widget;
use iced::widget::text_input::Id;
use iced::widget::text_input::State;
use iced::widget::{
    button, column, container, horizontal_space, row, text, text_input, Column, Container, Row,
};
use iced::{gradient, Alignment, Application, Command, Element, Length, Padding, Settings, Theme};


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

pub struct Calculator {
    temp_row: String,
    temp_col: String,
    mult_row: String,
    mult_col: String,
    mode: Modes,
    pub matrix: Vec<Vec<String>>,
    pub mult_matrix: Vec<Vec<String>>,
    output_matrix: Vec<Vec<f64>>,
    error: String,
}

#[derive(Debug, Clone)]
pub enum Message {
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
                output_matrix: Vec::default(),
                error: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "The Matrix Calculator".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        self.get_error();
        match message {
            Message::TempRow(row) => {
                if row.is_numeric() {
                    self.temp_row = row;
                    self.update_matrix_size(false);
                    self.output_matrix.clear();
                };
                Command::none()
            }
            Message::TempCol(col) => {
                if col.is_numeric() {
                    self.temp_col = col;
                    self.update_matrix_size(false);
                    self.output_matrix.clear();

                };
                Command::none()
            }
            Message::MultRow(row) => {
                if row.is_numeric() {
                    self.mult_row = row;
                    self.update_matrix_size(true);
                    self.output_matrix.clear();

                };
                Command::none()
            }
            Message::MultCol(col) => {
                if col.is_numeric() {
                    self.mult_col = col;
                    self.update_matrix_size(true);
                    self.output_matrix.clear();

                };
                Command::none()
            }
            Message::SubmitRow => text_input::focus(Id::new("cols")),
            Message::SubmitMRow => text_input::focus(Id::new("mcols")),
            Message::UpdatedMatrix(val, row, col) => {
                if val.is_float() {
                    self.matrix[row][col] = val
                };
                Command::none()
            }
            Message::UpdatedMultMatrix(val, row, col) => {
                if val.is_float() {
                    self.mult_matrix[row][col] = val
                };
                Command::none()
            }
            Message::SubmitEntry(row, col) => {
                if col >= self.temp_col.parse::<usize>().unwrap() - 1 {
                    if row >= self.temp_row.parse::<usize>().unwrap() - 1 {
                        return Command::none();
                    } else {
                        return text_input::focus(Id::new(format!("{}x0", row + 1)));
                    }
                }
                text_input::focus(Id::new(format!("{row}x{}", col + 1)))
            }
            Message::SubmitMEntry(row, col) => {
                if col >= self.mult_col.parse::<usize>().unwrap() - 1 {
                    if row >= self.temp_row.parse::<usize>().unwrap() - 1 {
                        return Command::none();
                    } else {
                        return text_input::focus(Id::new(format!("m{}x0", row + 1)));
                    }
                }
                text_input::focus(Id::new(format!("m{row}x{}", col + 1)))
            }
            Message::Inverse => {
                self.mode = Modes::Inverse;
                Command::none()
            }
            Message::Multiply => {
                self.mode = Modes::Multiply;
                Command::none()
            }
            Message::SysEq => {
                self.mode = Modes::SysEq;
                Command::none()
            }
            Message::Determinant => {
                self.mode = Modes::Determinant;
                Command::none()
            }
            Message::Clear => {
                self.matrix
                    .fill(vec!["0".to_owned(); self.temp_col.parse().unwrap_or(0)]);
                self.mult_matrix
                    .fill(vec!["0".to_owned(); self.mult_col.parse().unwrap_or(0)]);
                self.output_matrix.clear();
                Command::none()
            }
            Message::Calculate => match self.mode {
                Modes::Input => Command::none(),
                Modes::Inverse => todo!(),
                Modes::Multiply => {
                    self.output_matrix = self.multiply();
                    Command::none()
                }
                Modes::SysEq => todo!(),
                Modes::Determinant => todo!(),
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<theme::Theme>> {

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
            button("Multiply")
                .on_press(Message::Multiply)
                .style(button_styles[0]),
            button("Inverse")
                .on_press(Message::Inverse)
                .style(button_styles[1]),
            button("Sys. of Equations")
                .on_press(Message::SysEq)
                .style(button_styles[2]),
            button("Determinant")
                .on_press(Message::Determinant)
                .style(button_styles[3])
        ]
        .spacing(10)
        .padding(30);

        let util_buttons = row!(
            button("Clear")
                .on_press(Message::Clear)
                .style(theme::Button::Red),
            horizontal_space(Length::Fixed(30.)),
            button("Calculate").on_press(Message::Calculate)
        );

        let content = column![
            text("What do you want to do?").size(30),
            functions,
            text("Please input the dimensions of the matrix"),
            self.make_size_input().padding(20),
            text(&self.error).style(theme::Text::Color(Color::from_rgb(1.,0.,0.))),
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
