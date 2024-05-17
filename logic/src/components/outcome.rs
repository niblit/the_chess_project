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
