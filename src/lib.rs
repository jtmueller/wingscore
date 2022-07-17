pub mod util;

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
    pub fn get_name(&self) -> &str {
        match self {
            Self::Birds(_) => "Birds",
            Self::BonusCards(_) => "Bonus Cards",
            Self::RoundEndGoals(_) => "Round End Goals",
            Self::Eggs(_) => "Eggs",
            Self::StashedFood(_) => "Stashed Food",
            Self::TuckedCards(_) => "Tucked Cards",
        }
    }

    pub fn get_score(&self) -> &u8 {
        match self {
            Self::Birds(x) => x,
            Self::BonusCards(x) => x,
            Self::RoundEndGoals(x) => x,
            Self::Eggs(x) => x,
            Self::StashedFood(x) => x,
            Self::TuckedCards(x) => x,
        }
    }

    pub fn update(&self, new_score: u8) -> Self {
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
    pub id: usize,
    pub name: String,
    pub scores: [Score; 6],
}

impl Player {
    pub fn new(id: usize, name: String) -> Self {
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

    pub fn set_score(&mut self, new_score: Score) {
        for score in self.scores.iter_mut() {
            if util::variant_eq(score, &new_score) {
                *score = new_score;
                break;
            }
        }
    }

    pub fn total_score(&self) -> u16 {
        self.scores.iter().map(|s| *s.get_score() as u16).sum()
    }
}
