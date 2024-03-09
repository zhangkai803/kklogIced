use crate::core::config::Config;
use crate::core::node::Node;
use crate::core::stream::Stream;
use crate::message::Message;
use iced::executor;
use iced::keyboard;
use iced::theme;
use iced::widget::Column;
use iced::widget::{button, column, container, horizontal_space, pick_list, row, scrollable, text};
use iced::{Alignment, Application, Command, Element, Length, Subscription, Theme};
use serde_yaml::Error;
use std::sync::Arc;

#[derive(Debug)]
pub struct Layout {
    pub stream: Stream,
    pub theme: Theme,
    pub config: Config,
    pub selected_node: Option<Node>,
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
                config: Config::default(),
                selected_node: None,
            },
            Command::perform(load_yaml(), Message::YamlLoaded),
        )
    }

    fn title(&self) -> String {
        format!("KKlog - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
            Message::AddSource => {}
            Message::YamlLoaded(Ok(config)) => {
                self.config = config;
            }
            Message::YamlLoaded(Err(err)) => {
                println!("load yaml err: {:?}", err)
            }
            Message::SourceSelected(node) => {
                println!("selected: {:?}", node);
                self.selected_node = Some(node.clone());
                self.stream = Stream::new(
                    node.source.clone(),
                    format!("{}", node.url("dev", self.config.user.token.as_str())),
                );
                // return Command::perform(wss(node.url("dev", self.config.user.token.as_str())), Message::WssRead);
            }
            Message::WssRead(Some(msg)) => {
                self.stream.buf.push(msg.to_string());
                println!("len after push: {}", self.stream.buf.len());
            }
            Message::WssRead(None) => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, _modifiers| match key {
            _ => {
                // println!("{:?}", key);
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
            text(self.stream.title.as_str()),
            horizontal_space(),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let stream = container(row![self.sidebar(), self.stream.view()])
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

impl Layout {
    fn sidebar(&self) -> Element<Message> {
        container(scrollable(
            Column::with_children(self.config.envs.iter().map(Node::view))
                .spacing(40)
                .padding(10)
                .width(200)
                .align_items(Alignment::Start),
        ))
        .style(theme::Container::Box)
        .height(Length::Fill)
        .into()
    }
}

async fn read_yaml() -> Option<String> {
    if let Some(home) = std::env::home_dir() {
        return Some(
            std::fs::read_to_string(format!("{}/.kkconf.yaml", home.display()))
                .expect("read yaml err"),
        );
    }
    None
}

async fn load_yaml() -> Result<Config, Arc<Error>> {
    let conf: Config = serde_yaml::from_str(read_yaml().await.unwrap().as_str())?;
    Ok(conf)
}
