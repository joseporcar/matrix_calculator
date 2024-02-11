
use iced::alignment::Horizontal;
use iced::futures::ready;
use iced::widget::text_input::Id;
use iced::{Theme, Element, Application, Settings, gradient, Length, Alignment, Padding, Command};
use iced::widget::{text, button, column, row, text_input, horizontal_space, container, Row};

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

trait Numeric {
    fn is_numeric(&self) -> bool;
}
impl Numeric for String {
    fn is_numeric(&self) -> bool {
        self.trim().parse::<u8>().is_ok() || self == ""
    }
}
struct Calculator {
    temp_row: String,
    temp_col: String,
    err_msg: String,
    action: Actions,
    rows: u8,
    cols: u8,
    matrix: Vec<Vec<u8>>,
}

#[derive(Debug, Clone)]
enum Message {
    TempRow(String),
    TempCol(String),
    SubmitRow,
    Inverse,
    Multiply,
    SysEq,
    Determinant,
}

enum Actions {
    Input,
    Inverse,
    Multiply,
    SysEq,
    Determinant,
}

impl Application for Calculator {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self {
            temp_row: String::new(),
            temp_col: String::new(),
            action: Actions::Input,
            rows: 0,
            cols: 0,
            err_msg: String::new(),
            matrix: Vec::default(),
        }, Command::none())
    }

    fn title(&self) -> String {
        "The Matrix Calculator".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::TempRow(row) => { if row.is_numeric() {self.rows = row.parse().unwrap(); self.temp_row = row}; Command::none()},
            Message::TempCol(col) => { if col.is_numeric() {self.cols = col.parse().unwrap(); self.temp_col = col}; Command::none()},
            Message::SubmitRow => text_input::focus(Id::new("cols")),
            Message::Inverse => Command::none(),
            Message::Multiply => Command::none(),
            Message::SysEq => Command::none(),
            Message::Determinant => Command::none(),

        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let err = "";

        let get_size = row![
            text_input("rows", &self.temp_row).on_input(Message::TempRow).width(Length::Fixed(50.)).on_submit(Message::SubmitRow),
            text(" x "),
            text_input("cols", &self.temp_col).on_input(Message::TempCol).width(Length::Fixed(50.)).id(Id::new("cols"))
        ];

        let size_container = container(get_size).center_x();

        let functions = row![
            button("Multiply").on_press(Message::Multiply),
            button("Inverse").on_press(Message::Inverse),
            button("Sys. of Equations").on_press(Message::SysEq),
            button("Determinant").on_press(Message::Determinant)
        ].spacing(10).padding(30);

        let content = column![
            text("What do you want to do?").size(30),
            functions,
            text("Please input the dimensions of the matrix"),
            size_container.padding(20),
            ].align_items(Alignment::Center);

        container(content).width(Length::Fill).padding(40).center_x().into()
            
        
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

}