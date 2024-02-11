


use iced::alignment::Horizontal;
use iced::futures::ready;
use iced::{Theme, Element, Sandbox, Settings, gradient, Length, Alignment};
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
    menu: Menu,
    rows: u8,
    cols: u8,
}

#[derive(Debug, Clone)]
enum Message {
    TempRow(String),
    TempCol(String),
    HomeDown,
    InverseDown,
    MultiplyDown,
    SysEqDown,
}

enum Menu {
    Home,
    Inverse,
    Multiply,
    SysEq,
}
impl Menu {
    fn to_home(&mut self) {
        *self = Menu::Home
    }
    fn to_inverse(&mut self) {
        *self = Menu::Home
    }
    fn to_multiply(&mut self) {
        *self = Menu::Home
    }
    fn to_syseq(&mut self) {
        *self = Menu::Home
    }
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self {
            temp_row: String::new(),
            temp_col: String::new(),
            menu: Menu::Home,
            rows: 0,
            cols: 0,
            err_msg: String::new(),
        }
    }

    fn title(&self) -> String {
        "The Matrix Calculator".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TempRow(row) => if row.is_numeric() {self.temp_row = row},
            Message::TempCol(col) => if col.is_numeric() {self.temp_col = col},
            Message::HomeDown => self.menu.to_home(),
            Message::InverseDown => self.menu.to_inverse(),
            Message::MultiplyDown => self.menu.to_multiply(),
            Message::SysEqDown => self.menu.to_syseq(),

        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let txt = "hello";
        let err = "";
        let get_size = row![
            text_input("rows", &self.temp_row).on_input(Message::TempRow).width(Length::Fixed(50.)),
            text(" x ").horizontal_alignment(Horizontal::Center),
            text_input("cols", &self.temp_col).on_input(Message::TempCol).width(Length::Fixed(50.))
        ];
        container(column![
            text("Please input the dimensions of the matrix").horizontal_alignment(Horizontal::Center),
            get_size,
            text("\nWhat do you want to do with this matrix?\n"), 
            txt
            ].padding(5)
        ).width(Length::FillPortion(3)).center_x()
        .into()
            
        
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}