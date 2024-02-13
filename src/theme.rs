
use iced::{
    application,
    executor::Default,
    theme::palette,
    widget::{button, container, text, text_input, rule},
    Application, BorderRadius, Color,
};

#[derive(Default, Clone, Debug, Copy)]
pub struct Theme;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: Color::from_rgb(
                0x20 as f32 / 255.0,
                0x22 as f32 / 255.0,
                0x25 as f32 / 255.0,
            ),
            text_color: Color::from_rgb(0.90, 0.90, 0.90),
        }
    }
}
#[derive(Default)]
pub enum TextInput {
    #[default]
    Default,
    Borderless,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = &palette::EXTENDED_DARK;

        let default = text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: palette.background.strong.color,
            icon_color: palette.background.weak.text,
        };
        match style {
            TextInput::Default => default,
            TextInput::Borderless => text_input::Appearance {
                border_width: 0.,
                border_radius: 0.0.into(),
                ..default
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = &palette::EXTENDED_DARK;

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: palette.background.base.text,
            icon_color: palette.background.weak.text,
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        let palette = &palette::EXTENDED_DARK;

        palette.background.strong.color
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        let palette = &palette::EXTENDED_DARK;

        palette.background.base.text
    }

    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        Color::BLACK
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        let palette = &palette::EXTENDED_DARK;

        palette.primary.weak.color
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = &palette::EXTENDED_DARK;

        text_input::Appearance {
            background: palette.background.weak.color.into(),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: palette.background.strong.color,
            icon_color: palette.background.strong.color,
        }
    }
}


#[derive(Clone, Copy, Default)]
pub enum Text {
    /// The default style.
    #[default]
    Default,
    /// Colored text.
    Color(Color),
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Text::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => text::Appearance::default(),
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Button {
    #[default]
    NotPressed,
    Pressed,
    Red,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let base = button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(100, 100, 200))),
            border_radius: BorderRadius::from(2.),
            border_width: 2.,
            border_color: Color::BLACK,
            text_color: Color::WHITE,
            ..button::Appearance::default()
        };

        match style {
            Button::NotPressed => base,
            Button::Pressed => button::Appearance {
                background: Some(iced::Background::Color(Color::BLACK)),
                border_color: Color::WHITE,
                text_color: Color::WHITE,
                ..base
            },
            Button::Red => button::Appearance {
                text_color: Color::from_rgb(1., 0., 0.),
                ..base
            },
        }
    }
}

impl container::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            ..container::Appearance::default()
        }
    }
}

impl rule::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> rule::Appearance {
        let palette = &palette::EXTENDED_DARK;
        rule::Appearance {
            color: palette.background.strong.color.into(),
            width: 1,
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
        }
    }
}
