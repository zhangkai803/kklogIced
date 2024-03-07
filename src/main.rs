use iced::application;
use iced::executor;
use iced::keyboard;
use iced::mouse;
use iced::theme;
use iced::widget::{
    button, canvas, checkbox, column, container, horizontal_space, pick_list,
    row, scrollable, text,
};
use iced::{
    color, Alignment, Application, Command, Element, Font, Length, Point,
    Rectangle, Renderer, Settings, Subscription, Theme,
};

pub fn main() -> iced::Result {
    Layout::run(Settings::default())
}

#[derive(Debug)]
struct Layout {
    stream: Stream,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Previous,
    ThemeSelected(Theme),
    AddSource,
    Quit,
}

impl Application for Layout {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                stream: Stream::default(),
                theme: Theme::Light,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("KKlog - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Next => {
                self.stream = self.stream.next();
            }
            Message::Previous => {
                self.stream = self.stream.previous();
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
            Message::AddSource => {}
            Message::Quit => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;
        keyboard::on_key_press(|key, _modifiers| {
            match key {
                keyboard::Key::Named(key::Named::ArrowLeft) => {
                    Some(Message::Previous)
                }
                keyboard::Key::Named(key::Named::ArrowRight) => {
                    Some(Message::Next)
                }
                _ => {
                    println!("{:?}", key);
                    None
                }
            }
        })
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            button("+ Add").on_press(Message::AddSource).padding(10),
            horizontal_space(),
            text(self.stream.title),
            horizontal_space(),
            button("← Previous")
                .padding([5, 10])
                .on_press(Message::Previous),
            button("Next →")
                .padding([5, 10])
                .on_press(Message::Next),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let stream = container(self.stream.view())
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();

            container::Appearance::default()
                .with_border(palette.background.strong.color, 4.0)
        })
        .padding(4)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        column![header, stream]
            .spacing(10)
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Stream {
    title: &'static str,
}

impl Stream {
    const LIST: &'static [Self] = &[
        // Self {
        //     title: "Centered",
        //     view: centered,
        // },
        // Self {
        //     title: "Column",
        //     view: column_,
        // },
        Self {
            title: "Left",
        },
        Self {
            title: "KKlog",
            // view: application,
        },
        Self {
            title: "Right",
        },
    ];

    fn is_first(self) -> bool {
        Self::LIST.first() == Some(&self)
    }

    fn is_last(self) -> bool {
        Self::LIST.last() == Some(&self)
    }

    fn previous(self) -> Self {
        let Some(index) =
            Self::LIST.iter().position(|&example| example == self)
        else {
            return self;
        };

        Self::LIST
            .get(index.saturating_sub(1))
            .copied()
            .unwrap_or(self)
    }

    fn next(self) -> Self {
        let Some(index) =
            Self::LIST.iter().position(|&example| example == self)
        else {
            return self;
        };

        Self::LIST.get(index + 1).copied().unwrap_or(self)
    }

    fn view(&self) -> Element<Message> {
        let sidebar = container(
            column!["Sidebar!", square(50), square(50)]
                .spacing(40)
                .padding(10)
                .width(200)
                .align_items(Alignment::Center),
        )
        .style(theme::Container::Box)
        .height(Length::Fill)
        .center_y();

        let content = container(
            scrollable(
                column![
                    "Content!",
                    square(400),
                    square(200),
                    square(400),
                    "The end"
                ]
                .spacing(40)
                .align_items(Alignment::Center)
                .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10);

        row![sidebar, content].into()
    }
}

impl Default for Stream {
    fn default() -> Self {
        Self::LIST[0]
    }
}

fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}
