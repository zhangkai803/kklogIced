use iced::Element;

use crate::message::Message;

#[derive(Debug, Clone, Default)]
pub struct Node {
    name: String,
    project: String,
    deployment: String,
    r#type: String,
    pod: String,
    namespace: String,
}

impl Node {
    fn new() -> Self {
        Self {
            name: todo!(),
            project: todo!(),
            deployment: todo!(),
            r#type: todo!(),
            pod: todo!(),
            namespace: todo!(),
        }
    }
    fn view(&self) -> Element<Message>{

    }
}
