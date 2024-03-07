
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
        Self { title: "Left" },
        Self { title: "KKlog" },
        Self { title: "Right" },
    ];

    fn is_first(self) -> bool {
        Self::LIST.first() == Some(&self)
    }

    fn is_last(self) -> bool {
        Self::LIST.last() == Some(&self)
    }

    fn previous(self) -> Self {
        let Some(index) = Self::LIST.iter().position(|&example| example == self) else {
            return self;
        };

        Self::LIST
            .get(index.saturating_sub(1))
            .copied()
            .unwrap_or(self)
    }

    fn next(self) -> Self {
        let Some(index) = Self::LIST.iter().position(|&example| example == self) else {
            return self;
        };

        Self::LIST.get(index + 1).copied().unwrap_or(self)
    }

    fn sidebar(&self) -> Element<Message> {
        container(
            column![
                "Sidebar!", square(50), square(50)
            ]
            .spacing(40)
            .padding(10)
            .width(200)
            .align_items(Alignment::Center),
        )
        .style(theme::Container::Box)
        .height(Length::Fill)
        .center_y()
    }

    fn view(&self) -> Element<Message> {
        let sidebar = self.sidebar();
        let content = container(
            scrollable(
                column!["Content!", square(400), square(200), square(400), "The end"]
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
