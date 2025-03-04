use iced::widget::{button, container};
use iced::{Color, Theme};

#[derive(Debug, Clone, Copy)]
pub enum ButtonStyle {
    Primary,
    Secondary,
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match self {
            ButtonStyle::Primary => button::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb8(0x3B, 0x76, 0xEF))),
                border: iced::Border {
                    radius: 5.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                text_color: Color::WHITE,
                ..button::Appearance::default()
            },
            ButtonStyle::Secondary => button::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb8(0x80, 0x80, 0x80))),
                border: iced::Border {
                    radius: 5.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                text_color: Color::WHITE,
                ..button::Appearance::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: active.shadow_offset + iced::Vector::new(0.0, 1.0),
            ..active
        }
    }
}

impl Into<iced::theme::Button> for ButtonStyle {
    fn into(self) -> iced::theme::Button {
        iced::theme::Button::Primary
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContainerStyle {
    Bordered,
}

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match self {
            ContainerStyle::Bordered => container::Appearance {
                border: iced::Border {
                    width: 1.0,
                    color: Color::from_rgb8(0xC0, 0xC0, 0xC0),
                    radius: 5.0.into(),
                },
                ..container::Appearance::default()
            },
        }
    }
}

impl Into<iced::theme::Container> for ContainerStyle {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Box
    }
}
