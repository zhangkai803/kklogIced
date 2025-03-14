use crate::core::config::{Config, Pod};
use crate::core::stream::Stream;
use crate::message::Message;
use crate::style::{ButtonStyle, ContainerStyle}; // 引入自定义样式
use iced::executor;
use iced::futures::StreamExt;
use iced::widget::{button, column, container, horizontal_space, pick_list, row, text};
use iced::{Alignment, Application, Command, Element, Length, Subscription, Theme};
use serde_yaml::Error;
use std::hash::Hash;
use std::sync::Arc;

use async_tungstenite::tokio::connect_async;
use futures_util::SinkExt; // 引入 SinkExt
use home::home_dir; // 引入 home crate
use url::Url;

#[derive(Debug)]
pub struct Layout {
    pub stream: Stream,
    pub theme: Theme,
    pub config: Config,
    pub selected_env: Option<String>,
    pub selected_namespace: Option<String>,
    pub selected_deployment: Option<String>,
    pub selected_pod: Option<Pod>,
    pub available_pods: Vec<Pod>, // 新增
    pub cur_pod: Option<Pod>,
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
                selected_env: None,
                selected_namespace: None,
                selected_deployment: None,
                selected_pod: None,
                available_pods: Vec::new(), // 新增
                cur_pod: None,
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
            Message::YamlLoaded(Ok(config)) => {
                self.config = config;
            }
            Message::YamlLoaded(Err(err)) => {
                println!("load yaml err: {:?}", err)
            }
            Message::CloseConnection(_) => {
                // 处理关闭连接的消息，这里不需要做什么，
                // 因为connection_id不匹配的连接会自动关闭
            }
            Message::WssRead(msg) => {
                self.stream.buf.push(msg.to_string());
                if self.stream.buf.len() > 2000 {
                    let _ = self.stream.buf.drain(0..1000);
                }
            }

            Message::EnvSelected(env) => {
                self.selected_env = Some(env.clone());
                // self.available_namespaces = self
                //     .config
                //     .namespaces
                //     .get(env)
                //     .cloned()
                //     .unwrap_or_default();
                // self.selected_namespace = None;
                // self.available_deployments = Vec::new();
                // self.selected_deployment = None;
                // self.available_pods = Vec::new();
                // self.selected_pod = None;

                let _ = self.update_stream(self.config.user.token.clone());
            }
            Message::NamespaceSelected(namespace) => {
                self.selected_namespace = Some(namespace);

                let _ = self.update_stream(self.config.user.token.clone());
            }
            Message::DeploymentSelected(deployment) => {
                self.selected_deployment = Some(deployment.clone());
                self.available_pods = self
                    .config
                    .pods
                    .get(&deployment)
                    .cloned()
                    .unwrap_or_default();
                self.selected_pod = None;
                let _ = self.update_stream(self.config.user.token.clone());
            }
            Message::PodSelected(pod) => {
                self.selected_pod = Some(pod);
                let _ = self.update_stream(self.config.user.token.clone());
            }
            Message::CloneWindow => {
                // 调用系统命令克隆窗口 TOTEST
                if cfg!(target_os = "windows") {
                    // Windows
                    {
                        tokio::spawn(async {
                            std::process::Command::new("cmd")
                                .args(&["/c", "start", "kklogIced.exe"]) // 替换为你的程序名
                                .spawn()
                                .unwrap();
                        });
                    }
                } else if cfg!(target_os = "macos") {
                    // macOS TESTED
                    {
                        tokio::spawn(async {
                            std::process::Command::new("sh")
                                .args(&["-c", "kklogIced &"]) // 替换为你的程序名
                                .spawn()
                                .unwrap();
                        });
                        // tokio::spawn(async {
                        //     std::process::Command::new("open")
                        //         .args(&["-n", "/Applications/kklogIced.app"]) // 替换为你的程序路径
                        //         .spawn()
                        //         .unwrap();
                        // });
                    }
                } else {
                    // Linux TOTEST
                    {
                        tokio::spawn(async {
                            std::process::Command::new("sh")
                                .args(&["-c", "kklogIced &"]) // 替换为你的程序名
                                .spawn()
                                .unwrap();
                        });
                    }
                }
            },
            Message::ClearBuf => {
                self.stream.buf.clear();
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut s_vec = vec![];
        if self.stream.url.len() > 0 {
            let my = Subscription::from_recipe(MyRecipe::new(
                self.stream.url.clone(),
                self.stream.connection_id,
            ));
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
            horizontal_space(),
            text(if self.stream.pod.name.len() == 0 {
                "KKlog - Iced".to_string()
            } else {
                format!(
                    "KKlog - Iced - [{}][{}]",
                    self.stream.namespace,
                    self.stream.pod.to_string()
                )
            }),
            horizontal_space(),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
            button("Clone Window") // 新增
                .style(ButtonStyle::Primary) // 应用自定义样式
                .padding([5, 10])
                .on_press(Message::CloneWindow),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let env_list = pick_list(
            self.config.envs.clone(),
            self.selected_env.clone(),
            Message::EnvSelected,
        );

        let namespace_list = pick_list(
            self.config.namespaces.clone(),
            self.selected_namespace.clone(),
            Message::NamespaceSelected,
        );

        let deployment_list = pick_list(
            self.config.deployments.clone(),
            self.selected_deployment.clone(),
            Message::DeploymentSelected,
        );

        let pod_list = pick_list(
            self.available_pods.clone(), // 修改
            self.selected_pod.clone(),
            Message::PodSelected,
        );

        let selector_row = row![
            text("Env:"),
            env_list,
            text("Namespace:"),
            namespace_list,
            text("Project:"),
            deployment_list,
            text("Pod:"),
            pod_list,
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let functional_btn_row = row![
            button("Clear Buf") // 新增
                .style(ButtonStyle::Primary) // 应用自定义样式
                .padding([5, 10])
                .on_press(Message::ClearBuf),
        ]
        .spacing(10)
        .align_items(Alignment::End);

        let sub_header = row![selector_row, horizontal_space(), functional_btn_row];

        let stream = container(row![
            // self.sidebar(),
            self.stream.view()
        ])
        .style(ContainerStyle::Bordered) // 应用自定义样式
        .padding(4)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        column![header, sub_header, stream]
            .spacing(10)
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

impl Layout {
    // fn sidebar(&self) -> Element<Message> {
    //     container(scrollable(
    //         Column::with_children(self.config.envs.iter().map(Node::view))
    //             .spacing(40)
    //             .padding(10)
    //             .width(200)
    //             .align_items(Alignment::Start),
    //     ))
    //     .style(theme::Container::Box)
    //     .height(Length::Fill)
    //     .into()
    // }

    fn update_stream(&mut self, token: String) -> Command<Message> {
        if let (Some(env), Some(namespace), Some(deployment), Some(pod)) = (
            &self.selected_env,
            &self.selected_namespace,
            &self.selected_deployment,
            &self.selected_pod,
        ) {
            if self.cur_pod.is_some()
                && self.selected_pod.clone().unwrap() == self.cur_pod.clone().unwrap()
            {
                println!("same pod, ignore");
                return Command::none();
            }
            self.cur_pod = self.selected_pod.clone();

            // 如果有当前连接，先发送关闭命令
            let connection_id = self.stream.connection_id;
            let close_command = if connection_id != 0 {
                println!("close connection: {}", connection_id);
                Command::perform(async move { connection_id }, Message::CloseConnection)
            } else {
                Command::none()
            };

            self.stream = Stream::from_selected(
                env.clone(),
                namespace.clone(),
                deployment.clone(),
                pod.clone(),
                token,
            );
            Command::batch(vec![close_command, Command::none()])
        } else {
            Command::none()
        }
    }
}

async fn read_yaml() -> Option<String> {
    if let Some(home) = home_dir() {
        return Some(
            std::fs::read_to_string(format!("{}/.kkconf.yaml", home.display()))
                .expect("read yaml err"),
        );
    }
    None
}

async fn load_yaml() -> Result<Config, Arc<Error>> {
    let conf: Config = serde_yaml::from_str(read_yaml().await.unwrap().as_str())?;
    println!("{:?}", conf);
    Ok(conf)
}

struct MyRecipe {
    url: String,
    connection_id: u32,
}

impl MyRecipe {
    fn new(url: String, connection_id: u32) -> Self {
        Self { url, connection_id }
    }
}

impl iced::advanced::subscription::Recipe for MyRecipe {
    type Output = Message;

    fn hash(&self, state: &mut iced::advanced::Hasher) {
        self.url.hash(state)
    }

    fn stream(
        self: Box<Self>,
        _input: iced::advanced::subscription::EventStream,
    ) -> iced::advanced::graphics::futures::BoxStream<Self::Output> {
        let connection_id = self.connection_id;
        let (sender, receiver) = tokio::sync::mpsc::channel::<String>(1000);

        tokio::spawn(async move {
            let url = Url::parse(self.url.as_str()).expect("wss url incorrect");
            let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
            let (mut write, mut read) = ws_stream.split();

            loop {
                tokio::select! {
                    message = read.next() => {
                        match message {
                            Some(Ok(async_tungstenite::tungstenite::Message::Text(msg))) => {
                                if let Err(_) = sender.send(msg).await {
                                    break;
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }

            // 关闭连接
            let _ = write.close().await;
        });

        iced::futures::stream::unfold((receiver, connection_id), |(mut receiver, id)| async move {
            if let Some(received) = receiver.recv().await {
                Some((Message::WssRead(received), (receiver, id)))
            } else {
                Some((Message::CloseConnection(id), (receiver, id)))
            }
        })
        .boxed()
    }
}
