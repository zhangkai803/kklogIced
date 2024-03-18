use crate::core::config::Config;
use crate::core::node::Node;
use crate::core::stream::Stream;
use crate::message::Message;
use iced::executor;
use iced::futures::StreamExt;
use iced::theme;
use iced::widget::Column;
use iced::widget::{button, column, container, horizontal_space, pick_list, row, scrollable, text};
use iced::{Alignment, Application, Command, Element, Length, Subscription, Theme};
use serde_yaml::Error;
use std::hash::Hash;
use std::sync::Arc;

use async_tungstenite::tokio::connect_async;

use url::Url;

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
                if self.selected_node.is_some()
                    && node.source == self.selected_node.clone().unwrap().source
                {
                    return Command::none();
                }
                self.selected_node = Some(node.clone());
                self.stream = Stream::new(
                    node.source.clone(),
                    node.url("dev", self.config.user.token.as_str()).clone(),
                );
            }
            Message::WssRead(msg) => {
                self.stream
                    .buf
                    .push(format!("{}: {:?}", self.stream.buf.len(), msg.to_string()));
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut s_vec = vec![];
        if self.stream.url.len() > 0 {
            let my = Subscription::from_recipe(MyRecipe::new(self.stream.url.clone()));
            s_vec.push(my);
        }
        Subscription::batch(s_vec)
        // keyboard::on_key_press(|key, _modifiers| match key {
        //     _ => {
        //         // println!("{:?}", key);
        //         None
        //     }
        // })
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

struct MyRecipe {
    url: String,
}

impl MyRecipe {
    fn new(url: String) -> Self {
        Self { url }
    }
}

impl iced::advanced::subscription::Recipe for MyRecipe {
    type Output = Message;

    fn hash(&self, state: &mut iced::advanced::Hasher) {
        self.url.hash(state)
    }

    fn stream(
        self: Box<Self>,
        input: iced::advanced::subscription::EventStream,
    ) -> iced::advanced::graphics::futures::BoxStream<Self::Output> {
        let (sender, receiver) = tokio::sync::mpsc::channel::<String>(1000);

        // 开启新线程发送消息
        tokio::spawn(async move {
            let url = Url::parse(self.url.as_str()).expect("wss url incorrect");
            let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

            let (_, mut read) = ws_stream.split();

            loop {
                let message = read.next().await;
                if message.is_none() {
                    continue;
                }
                let message = message.unwrap();
                if message.is_err() {
                    continue;
                }
                let message = message.unwrap();
                match message {
                    async_tungstenite::tungstenite::Message::Text(msg) => {
                        if let Err(_) = sender.send(msg).await {
                            break; // 如果发送出错（例如，接收器已被丢弃），则退出
                        }
                    }
                    _ => {}
                }
            }
        });

        // 将 mpsc 接收器转换为流
        iced::futures::stream::unfold(receiver, |mut receiver| async move {
            if let Some(received) = receiver.recv().await {
                Some((Message::WssRead(received), receiver))
            } else {
                None
            }
        })
        .boxed()
    }
}
