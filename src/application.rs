
#[derive(Debug)]
struct Layout {
    stream: Stream,
    theme: Theme,
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
        keyboard::on_key_press(|key, _modifiers| match key {
            keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::Previous),
            keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
            _ => {
                println!("{:?}", key);
                None
            }
        })
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            button("+ Add")
                .padding([5, 10])
                .on_press(Message::AddSource),
            horizontal_space(),
            text(self.stream.title),
            horizontal_space(),
            button("← Previous")
                .padding([5, 10])
                .on_press(Message::Previous),
            button("Next →").padding([5, 10]).on_press(Message::Next),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let stream = container(self.stream.view())
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();

                container::Appearance::default().with_border(palette.background.strong.color, 4.0)
            })
            .padding(4)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        column![header, stream].spacing(10).padding(20).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
