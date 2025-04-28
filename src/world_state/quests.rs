use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum EscapeFromHouse {
    #[default]
    Courier,
    GoSleep,
    CallDog,
    Escape,
    Completed,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum EnterTheCourt {
    #[default]
    None,
    Go,
    StopDrevnira,
    Completed,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum InCourHall {
    #[default]
    None,
    TalkWithManager,
    Wait,
    Completed,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum Court {
    #[default]
    None,
    TalkWithGuardian,
    StopDrevnira,
    DrevniraStopped,
    Completed,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum House {
    #[default]
    GoSleep,
    TalkWithPolice,
    Completed,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum StrangeOldWoman {
    #[default]
    None,
    GiveMeFeather,
    Beaten,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum BlondAndGopniks {
    #[default]
    None,
    TalkWithBlond,
    TalkWithGopniks,
    GiveDumplingsToBlond,
    TakeDumplingsFromBlond,
    Completed,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GoIntoCourt {
    #[default]
    None,
    Wait,
    CanGo,
    Go,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum Trial {
    #[default]
    None,
    SpeakWithJudges,
    Wait,
    FormidableFaceWon,
    FormidableFaceFailed,
    GoAtHome,
}
