use iced::pure::widget::{Button, Row};
use iced::pure::{column, row, text, text_input};
use iced::pure::{Element, Sandbox};
use iced::{alignment, Alignment, Color, Length, Settings};

fn main() -> iced::Result {
    env_logger::init();
    let mut settings = Settings::default();
    settings.window.size = (480, 340);
    WingScore::run(settings)
}

pub struct WingScore {
    players: Vec<Player>,
    new_player_name: Option<String>,
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

#[derive(Debug, Clone)]
pub enum Message {
    PlayerNameChanged(String),
    AddPlayer,
    SetScore(usize, Score),
}

#[derive(Debug, Clone)]
pub enum Score {
    Birds(u8),
    BonusCards(u8),
    RoundEndGoals(u8),
    Eggs(u8),
    StashedFood(u8),
    TuckedCards(u8),
}

impl Score {
    fn get_name(&self) -> &str {
        match self {
            Self::Birds(_) => "Birds",
            Self::BonusCards(_) => "Bonus Cards",
            Self::RoundEndGoals(_) => "Round End Goals",
            Self::Eggs(_) => "Eggs",
            Self::StashedFood(_) => "Stashed Food",
            Self::TuckedCards(_) => "Tucked Cards",
        }
    }

    fn get_score(&self) -> &u8 {
        match self {
            Self::Birds(x) => x,
            Self::BonusCards(x) => x,
            Self::RoundEndGoals(x) => x,
            Self::Eggs(x) => x,
            Self::StashedFood(x) => x,
            Self::TuckedCards(x) => x,
        }
    }

    fn update(&self, new_score: u8) -> Self {
        match self {
            Self::Birds(_) => Self::Birds(new_score),
            Self::BonusCards(_) => Self::BonusCards(new_score),
            Self::RoundEndGoals(_) => Self::RoundEndGoals(new_score),
            Self::Eggs(_) => Self::Eggs(new_score),
            Self::StashedFood(_) => Self::StashedFood(new_score),
            Self::TuckedCards(_) => Self::TuckedCards(new_score),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    id: usize,
    name: String,
    scores: [Score; 6],
}

impl Player {
    fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            scores: [
                Score::Birds(0),
                Score::BonusCards(0),
                Score::RoundEndGoals(0),
                Score::Eggs(0),
                Score::StashedFood(0),
                Score::TuckedCards(0),
            ],
        }
    }

    fn set_score(&mut self, new_score: Score) {
        for score in self.scores.iter_mut() {
            if variant_eq(score, &new_score) {
                *score = new_score;
                break;
            }
        }
    }

    fn total_score(&self) -> u16 {
        self.scores.iter().map(|s| *s.get_score() as u16).sum()
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
                        parse_byte(val).map(|v| s.update(v)).unwrap_or(s.clone()),
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

fn parse_byte(val: &str) -> Option<u8> {
    val.parse().ok()
}

/// Returns true if two enums have the same variant.
fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
