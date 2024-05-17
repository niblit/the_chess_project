use std::fmt;

use super::team::Team;

pub enum Outcome {
    Checkmate(Team),
    Resignation(Team),
    Timeout(Team),
    Stalemate,
    InsufficientMaterial,
    FiftyMoveRule,
    ThreefoldRepetition,
    DrawByAgreement,
}

const CHECKMATE: &str = "wins by checkmate";
const RESIGNATION: &str = "wins, opponent resigned";
const TIMEOUT: &str = "wins by timeout";

const STALEMATE: &str = "Draw by stalemate";
const INSUFFICIENT_MATERIAL: &str = "Draw by insufficient material";
const FIFTY_MOVE_RULE: &str = "Draw by fifty-move rule";
const THREEFOLD_REPETITION: &str = "Draw by repetition";
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
            Outcome::ThreefoldRepetition => String::from(THREEFOLD_REPETITION),
            Outcome::DrawByAgreement => String::from(BY_AGREEMENT)
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

mod tests {
    use super::*;

    #[test]
    fn test_as_sentence() {
        assert_eq!(Outcome::Checkmate(Team::Black).as_sentence(), format!("{} {}", Team::Black.as_sentence(), CHECKMATE));
        assert_eq!(Outcome::Checkmate(Team::White).as_sentence(), format!("{} {}", Team::White.as_sentence(), CHECKMATE));

        assert_eq!(Outcome::Resignation(Team::White).as_sentence(), format!("{} {}", Team::White.as_sentence(), RESIGNATION));
        assert_eq!(Outcome::Resignation(Team::Black).as_sentence(), format!("{} {}", Team::Black.as_sentence(), RESIGNATION));

        assert_eq!(Outcome::Timeout(Team::White).as_sentence(), format!("{} {}", Team::White.as_sentence(), TIMEOUT));
        assert_eq!(Outcome::Timeout(Team::Black).as_sentence(), format!("{} {}", Team::Black.as_sentence(), TIMEOUT));

        assert_eq!(Outcome::Stalemate.as_sentence(), STALEMATE);
        assert_eq!(Outcome::InsufficientMaterial.as_sentence(), INSUFFICIENT_MATERIAL);
        assert_eq!(Outcome::FiftyMoveRule.as_sentence(), FIFTY_MOVE_RULE);
        assert_eq!(Outcome::ThreefoldRepetition.as_sentence(), THREEFOLD_REPETITION);
        assert_eq!(Outcome::DrawByAgreement.as_sentence(), BY_AGREEMENT);
    }

    #[test]
    fn test_fmt_display() {
        assert_eq!(format!("{}", Outcome::Checkmate(Team::Black)), BLACK_SHORT);
        assert_eq!(format!("{}", Outcome::Checkmate(Team::White)), WHITE_SHORT);

        assert_eq!(format!("{}", Outcome::Resignation(Team::Black)), BLACK_SHORT);
        assert_eq!(format!("{}", Outcome::Resignation(Team::White)), WHITE_SHORT);

        assert_eq!(format!("{}", Outcome::Timeout(Team::Black)), BLACK_SHORT);
        assert_eq!(format!("{}", Outcome::Timeout(Team::White)), WHITE_SHORT);

        assert_eq!(format!("{}", Outcome::Stalemate), DRAW_SHORT);
        assert_eq!(format!("{}", Outcome::InsufficientMaterial), DRAW_SHORT);
        assert_eq!(format!("{}", Outcome::FiftyMoveRule), DRAW_SHORT);
        assert_eq!(format!("{}", Outcome::ThreefoldRepetition), DRAW_SHORT);
        assert_eq!(format!("{}", Outcome::DrawByAgreement), DRAW_SHORT);
    }
}
