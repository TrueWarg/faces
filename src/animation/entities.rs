use serde::Deserialize;

#[derive(Hash, PartialEq, Eq, Clone, Deserialize)]
pub enum MoveDirection {
    ForwardIdle,
    LeftIdle,
    BackwardIdle,
    RightIdle,
    ForwardMove,
    LeftMove,
    BackwardMove,
    RightMove,
}

impl MoveDirection {
    pub fn is_idle(&self) -> bool {
        matches!(
            self,
            MoveDirection::ForwardIdle
                | MoveDirection::BackwardIdle
                | MoveDirection::LeftIdle
                | MoveDirection::RightIdle
        )
    }
}
