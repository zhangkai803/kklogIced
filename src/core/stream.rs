use iced::widget::{column, container, scrollable, text};
use iced::{Alignment, Element, Length};
use tokio::net::TcpStream;

use crate::message::Message;

use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use url::Url;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Default, Clone)]
pub struct Stream {
    pub title: String,
    pub url: String,
    pub buf: Vec<String>,
    pub wss: Option<Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>>
}

impl Stream {
    pub fn new(title: String, url: String) -> Self {
        Self {
            title,
            url,
            buf: Vec::<String>::default(),
            wss: None,
        }
    }

    pub async fn start(&mut self) {
        let url = Url::parse(self.url.as_str()).unwrap();
        println!("connect to: {:?}", url.as_str());
        let (wss, _) = connect_async(url).await.unwrap();
        self.wss = Some(Arc::new(Mutex::new(wss)));
        // match connect(url) {
        //     Ok(r) => {
        //         let (mut socket, response) = r;

        //         println!(
        //             "Connected to the server. Response HTTP code: {}",
        //             response.status()
        //         );
        //         // for (ref header, value) in response.headers() {
        //         //     println!("{}: {:?}", header, value);
        //         // }
        //         match socket.read() {
        //             Ok(msg) => {
        //                 println!("message received: {:?}", msg);
        //                 // let res = sender.try_send(msg);
        //                 // if res.is_err() {
        //                 //     println!("push msg err: {:?}", res.unwrap_err())
        //                 // }
        //                 // Command::perform(wss_msg_read(msg), Message::WssRead);
        //                 // Some(msg)
        //             }
        //             Err(err) => {
        //                 println!("wss read err: {:?}", err);
        //                 // None
        //             }
        //         }
        //     }
        //     Err(err) => {
        //         println!("wss connect err: {}", err);
        //     }
        // }
    }

    pub fn view(&self) -> Element<Message> {
        // println!("len of buf: {}", self.buf.len());
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
