use crate::core::config::Config;
use crate::core::node::Node;
use crate::core::stream::Stream;
use crate::message::Message;
use iced::executor;
use iced::futures::SinkExt;
use iced::futures::StreamExt;
use iced::futures::FutureExt;
use iced::keyboard;
use iced::subscription;
use iced::theme;
use iced::widget::Column;
use iced::widget::{button, column, container, horizontal_space, pick_list, row, scrollable, text};
use iced::{Alignment, Application, Command, Element, Length, Subscription, Theme};
use serde_yaml::Error;
use std::fmt::format;
use std::hash::Hash;
use std::sync::Arc;
use iced::time;
use std::time::{Duration, Instant};

use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use url::Url;
use tokio::sync::Mutex;

use iced::futures::stream::BoxStream;
use std::{sync::mpsc, thread};


pub struct TokioExecutor;

impl iced::executor::Executor for TokioExecutor {
    fn new() -> Result<Self, iced::futures::io::Error>
    where
        Self: Sized {
        Ok(Self)
    }

    fn spawn(&self, future: impl iced::futures::prelude::Future<Output = ()> + iced::advanced::graphics::futures::MaybeSend + 'static) {
        tokio::spawn(future);
    }
}

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
        // println!("update called {:?}", self);
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
                // println!("selected: {:?}", node);
                self.selected_node = Some(node.clone());
                self.stream = Stream::new(
                    node.source.clone(),
                    node.url("dev", self.config.user.token.as_str()).clone(),
                );
                // return Command::perform(wss(node.url("dev", self.config.user.token.as_str())), Message::WssRead);
            }
            Message::WssRead(Some(msg)) => {
                // println!("WssRead: {:?}", msg);
                self.stream.buf.push(format!("{}: {:?}", self.stream.buf.len(), msg.to_string()));
            }
            Message::WssRead(None) => {}
            Message::Tick(_ins) => {
                println!("Tick: {:?}", _ins);
                self.stream.buf.push(format!("{}: {:?}", self.stream.buf.len(), _ins));
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        // println!("subscription called {:?}", self.stream.url);
        // let tick = time::every(Duration::from_millis(1000)).map(Message::Tick);
        // let read = read_wss(self.stream.wss);

        let my = Subscription::from_recipe(MyRecipe::new(self.stream.url.clone()));
        let s_vec = vec![my];
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

// fn read_wss(wss: Option<Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>>) -> Subscription<Message> {
    // subscription::channel(
    //     std::any::TypeId::of::<String>(),
    //     100,
    //     |output| async move {
    //         if let Some(wss) = wss {
    //             while let Some(msg) = wss.lock().await.next().await{
    //                 println!("Received a message: {:?}", msg);
    //                 return output.send(Message::WssRead(Some(msg.unwrap().to_string()))).await;
    //             }
    //         }
    //         output.send(Message::WssRead(None)).await
    //     }
    // )
// }

struct MyRecipe {
    url: String,
}

impl MyRecipe {
    fn new(url: String) -> Self {
        Self{url}
    }
}

impl iced::advanced::subscription::Recipe for MyRecipe {
    type Output = Message;

    fn hash(&self, state: &mut iced::advanced::Hasher) {
        std::any::TypeId::of::<Self>().hash(state)
    }

    fn stream(self: Box<Self>, input: iced::advanced::subscription::EventStream) -> iced::advanced::graphics::futures::BoxStream<Self::Output> {
        println!("stream called {:?}", self.url);
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        // 开启新线程发送消息
        std::thread::spawn(move || {
            loop {
                if let Err(_) = sender.send(self.url.clone()) {
                    break; // 如果发送出错（例如，接收器已被丢弃），则退出循环
                }
                std::thread::sleep(tokio::time::Duration::from_secs(1));
            }
        });

        // 将 mpsc 接收器转换为流
        iced::futures::stream::unfold(receiver, |mut receiver| async move {
            receiver.recv().ok().map(|msg| (Message::WssRead(Some(msg)), receiver))
        }).boxed()
    }
}
