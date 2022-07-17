use iced::pure::widget::{Button, Row};
use iced::pure::{column, row, text, text_input};
use iced::pure::{Element, Sandbox};
use iced::{alignment, Alignment, Color, Length, Settings};
use wingscore::{Message, Player};

pub mod util;

fn main() -> iced::Result {
    env_logger::init();
    let mut settings = Settings::default();
    settings.window.size = (480, 340);
    WingScore::run(settings)
}

pub struct WingScore {
    pub players: Vec<Player>,
    pub new_player_name: Option<String>,
}

impl Sandbox for WingScore {
    type Message = Message;

    fn new() -> Self {
        WingScore {
            players: Vec::new(),
            new_player_name: None,
        }
    }

    fn title(&self) -> String {
        "WingScore".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PlayerNameChanged(name) => {
                self.new_player_name = Some(name);
            }
            Message::AddPlayer => {
                if let Some(name) = self.new_player_name.take() {
                    let player_idx = self.players.len();
                    self.players.push(Player::new(player_idx, name));
                }
            }
            Message::SetScore(player_idx, score) => {
                self.players[player_idx].set_score(score);
            }
        }
    }

    fn view(&self) -> iced::pure::Element<'_, Self::Message> {
        let mut controls = column()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(20)
            .push(
                row()
                    .spacing(5)
                    .push(
                        text_input(
                            "Player Name",
                            match &self.new_player_name {
                                Some(name) => name,
                                None => "",
                            },
                            Message::PlayerNameChanged,
                        )
                        .on_submit(Message::AddPlayer)
                        .padding(10)
                        .size(24),
                    )
                    .push(button("Add Player").on_press(Message::AddPlayer)),
            );

        if self.players.len() > 0 {
            controls = controls.push(score_grid(&self.players));
        }

        let content: Element<_> = column().spacing(20).padding(20).push(controls).into();

        content
    }
}

fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    iced::pure::button(text(label).horizontal_alignment(alignment::Horizontal::Center))
        .padding(12)
        .width(Length::Units(150))
}

fn score_grid<'a>(players: &'a [Player]) -> Row<'a, Message> {
    let label_height = 29;
    let mut container = row().spacing(10).push({
        let mut col = column().push(text(" ").height(Length::Units(label_height)));

        let labels = players[0]
            .scores
            .iter()
            .map(|s| text(s.get_name()).height(Length::Units(label_height)));

        for label in labels {
            col = col.push(label);
        }

        col.push(text("Total").size(29).style(Color::from_rgb8(0, 0, 0xff)))
    });

    for player in players {
        container = container.push({
            let mut col = column().width(Length::Units(60)).push(
                text(&player.name)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
            );

            let scores = player.scores.iter().map(|s| {
                text_input("", &s.get_score().to_string(), |val| {
                    let val = if val.len() == 0 { "0" } else { &val };
                    Message::SetScore(
                        player.id,
                        util::parse_byte(val)
                            .map(|v| s.update(v))
                            .unwrap_or(s.clone()),
                    )
                })
                .padding(5)
            });

            for score in scores {
                col = col.push(score);
            }

            col.push(text(player.total_score().to_string()).size(30))
        });
    }

    container
}
