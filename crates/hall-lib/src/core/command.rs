use num_enum::{FromPrimitive, IntoPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, IntoPrimitive)]
pub enum GameSubCommand {
    #[num_enum(default)]
    Activate,
    Build,
    StartGame,
    ChooseIntent,
    Roll,
    ChooseAttr,
    Resources,
    PlayCard,
    ResolveCards,
    EndTurn,
    Tick,
    UpdateMission,
    UpdateTokens,
    UpdateState,
    EndGame,
}
