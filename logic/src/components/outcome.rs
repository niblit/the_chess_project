use std::fmt;

use super::team::Team;

#[derive(Clone, Copy)]
pub enum Outcome {
    Checkmate(Team),
    Resignation(Team),
    Timeout(Team),

    Stalemate,

    InsufficientMaterial,

    FiftyMoveRule,
    EnforcedSeventyFiveMoveRule,

    ThreefoldRepetition,
    EnforcedFivefoldRepetition,

    DrawByAgreement,
}

const CHECKMATE: &str = "wins by checkmate";
const RESIGNATION: &str = "wins, opponent resigned";
const TIMEOUT: &str = "wins by timeout";

const STALEMATE: &str = "Draw by stalemate";

const INSUFFICIENT_MATERIAL: &str = "Draw by insufficient material";

const FIFTY_MOVE_RULE: &str = "Draw by fifty-move rule";
const SEVENTY_FIVE_MOVE_RULE: &str = "Draw by seventy-five-move rule";

const THREEFOLD_REPETITION: &str = "Draw by threefold repetition";
const FIVEFOLD_REPETITION: &str = "Draw by fivefold repetition";

const BY_AGREEMENT: &str = "Draw by agreement";

impl Outcome {
    pub fn as_sentence(&self) -> String {
        match self {
            Outcome::Checkmate(team) => format!("{} {}", team.as_sentence(), CHECKMATE),
            Outcome::Resignation(team) => format!("{} {}", team.as_sentence(), RESIGNATION),
            Outcome::Timeout(team) => format!("{} {}", team.as_sentence(), TIMEOUT),

            Outcome::Stalemate => String::from(STALEMATE),

            Outcome::InsufficientMaterial => String::from(INSUFFICIENT_MATERIAL),

            Outcome::FiftyMoveRule => String::from(FIFTY_MOVE_RULE),
            Outcome::EnforcedSeventyFiveMoveRule => String::from(SEVENTY_FIVE_MOVE_RULE),

            Outcome::ThreefoldRepetition => String::from(THREEFOLD_REPETITION),
            Outcome::EnforcedFivefoldRepetition => String::from(FIVEFOLD_REPETITION),

            Outcome::DrawByAgreement => String::from(BY_AGREEMENT),
        }
    }
}

const DRAW_SHORT: &str = "1/2-1/2";
const BLACK_SHORT: &str = "0-1";
const WHITE_SHORT: &str = "1-0";

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result_string = match self {
            Outcome::Checkmate(team) => match team {
                Team::Black => BLACK_SHORT,
                Team::White => WHITE_SHORT,
            },
            Outcome::Resignation(team) => match team {
                Team::Black => BLACK_SHORT,
                Team::White => WHITE_SHORT,
            },
            Outcome::Timeout(team) => match team {
                Team::Black => BLACK_SHORT,
                Team::White => WHITE_SHORT,
            },
            _ => DRAW_SHORT,
        };
        write!(f, "{}", result_string)
    }
}
