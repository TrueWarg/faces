use serde::Deserialize;

#[derive(Hash, PartialEq, Eq, Clone, Deserialize)]
pub enum MoveAnimationDirection {
    ForwardIdle,
    LeftIdle,
    BackwardIdle,
    RightIdle,
    ForwardMove,
    LeftMove,
    BackwardMove,
    RightMove,
}

impl MoveAnimationDirection {
    fn is_idle(&self) -> bool {
        matches!(
            self,
            MoveAnimationDirection::ForwardIdle
                | MoveAnimationDirection::BackwardIdle
                | MoveAnimationDirection::LeftIdle
                | MoveAnimationDirection::RightIdle
        )
    }
}
