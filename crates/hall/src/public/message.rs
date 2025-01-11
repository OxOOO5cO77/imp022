mod game_activate;
mod game_build;
mod game_choose_attr;
mod game_choose_intent;
mod game_end_game;
mod game_end_turn;
mod game_play_card;
mod game_resolve_cards;
mod game_resources;
mod game_roll;
mod game_start_game;
mod game_tick;
mod game_update_mission;
mod game_update_state;

pub use game_activate::{GameActivateRequest, GameActivateResponse};
pub use game_build::{GameBuildRequest, GameBuildResponse};
pub use game_choose_attr::{GameChooseAttrRequest, GameChooseAttrResponse};
pub use game_choose_intent::{GameChooseIntentRequest, GameChooseIntentResponse};
pub use game_end_game::{GameEndGameRequest, GameEndGameResponse};
pub use game_end_turn::{GameEndTurnRequest, GameEndTurnResponse};
pub use game_play_card::{CardIdxType, CardTarget, GamePlayCardRequest, GamePlayCardResponse, PicksType};
pub use game_update_state::{GameUpdateStateRequest, GameUpdateStateResponse};

pub use game_resolve_cards::GameResolveCardsMessage;
pub use game_resources::GameResourcesMessage;
pub use game_roll::GameRollMessage;
pub use game_start_game::GameStartGameMessage;
pub use game_tick::GameTickMessage;
pub use game_update_mission::GameUpdateMissionMessage;

use shared_net::{op, Bufferable, GameIdType};

pub trait CommandMessage: Bufferable {
    const COMMAND: op::Command;
}

pub trait GameRequestMessage: CommandMessage {
    fn game_id(&self) -> GameIdType;
}

pub trait GameResponseMessage: CommandMessage {}
