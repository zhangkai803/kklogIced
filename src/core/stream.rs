use iced::widget::{column, container, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::message::Message;

use tungstenite::connect;
use url::Url;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Stream {
    pub title: String,
    pub url: String,
    pub buf: Vec<String>,
}

impl Stream {
    pub fn new(title: String, url: String) -> Self {
        Self {
            title,
            url,
            buf: Vec::<String>::default(),
        }
    }

    pub fn start(self) {
        let url = Url::parse(self.url.as_str()).unwrap();
        println!("connect to: {:?}", url.as_str());
        match connect(url) {
            Ok(r) => {
                let (mut socket, response) = r;

                println!(
                    "Connected to the server. Response HTTP code: {}",
                    response.status()
                );
                // for (ref header, value) in response.headers() {
                //     println!("{}: {:?}", header, value);
                // }
                match socket.read() {
                    Ok(msg) => {
                        println!("message received: {:?}", msg);
                        // let res = sender.try_send(msg);
                        // if res.is_err() {
                        //     println!("push msg err: {:?}", res.unwrap_err())
                        // }
                        // Command::perform(wss_msg_read(msg), Message::WssRead);
                        // Some(msg)
                    }
                    Err(err) => {
                        println!("wss read err: {:?}", err);
                        // None
                    }
                }
            }
            Err(err) => {
                println!("wss connect err: {}", err);
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        println!("len of buf: {}", self.buf.len());
        container(
            scrollable(
                column![text(self.buf.join("\n"))]
                    .spacing(40)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10)
        .into()
    }
}
