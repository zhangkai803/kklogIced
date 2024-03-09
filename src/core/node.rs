use crate::message::Message;
use iced::{
    theme,
    widget::{button, text},
    Element,
};
use serde::Deserialize;

use tungstenite::connect;
use url::Url;

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct Node {
    pub source: String,
    pub project: String,
    pub deployment: String,
    pub r#type: String,
    pub name: String,
    pub namespace: String,
}

impl Node {
    pub fn view(&self) -> Element<Message> {
        let url = Url::parse(self.url("dev", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6ODksImV4cCI6MTcxMDUyMTUzNX0.o9xNwcT5IAZv23nKVQ_Ci7jILAR0b4-9VG_MHy3jM_0").as_str()).unwrap();
        println!("connect to: {:?}", url.as_str());
        match connect(url) {
            Ok(r) => {
                let (mut socket, response) = r;

                println!("Connected to the server");
                println!("Response HTTP code: {}", response.status());
                for (ref header, value) in response.headers() {
                    println!("{}: {:?}", header, value);
                }

                loop {
                    match socket.read() {
                        Ok(msg) => {
                            println!("{:?}", msg.);
                        }
                        Err(err) => {
                            println!("err: {:?}", err)
                        }
                    }
                }
            }
            Err(err) => {
                println!("wss connect err: {}", err);
            }
        }

        button(text(self.source.clone()))
            .on_press(Message::SourceSelected(self.to_owned()))
            .style(theme::Button::Text)
            .into()
    }

    pub fn url(&self, env: &str, token: &str) -> String {
        format!(
            "wss://value.weike.fm/ws/api/k8s/{}/pods/log?container=app&follow=true&previous=false&timestamps=true&prefix=false&tailLines=2000&proj_id=1&namespace={}&label=app={},cicd_env=stable,name={},type={},version=stable&token={}",
            env, self.namespace, self.deployment, self.name, self.r#type, token
        )
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
