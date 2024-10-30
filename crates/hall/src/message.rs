mod game_activate;
mod game_build;
mod game_choose_attr;
mod game_end_game;
mod game_end_turn;
mod game_play_card;
mod game_resolve_cards;
mod game_resources;
mod game_roll;
mod game_start_game;
mod game_start_turn;
mod game_tick;
mod game_update_state;

pub use game_activate::{GameActivateRequest, GameActivateResponse};
pub use game_build::{GameBuildRequest, GameBuildResponse};
pub use game_choose_attr::{GameChooseAttrRequest, GameChooseAttrResponse, AttrIdxType};
pub use game_end_game::{GameEndGameRequest, GameEndGameResponse};
pub use game_end_turn::{GameEndTurnRequest, GameEndTurnResponse};
pub use game_play_card::{GamePlayCardRequest, GamePlayCardResponse, CardIdxType};
pub use game_start_turn::{GameStartTurnRequest, GameStartTurnResponse};

pub use game_resolve_cards::GameResolveCardsResponse;
pub use game_resources::GameResourcesResponse;
pub use game_roll::GameRollResponse;
pub use game_start_game::GameStartGameResponse;
pub use game_tick::GameTickResponse;
pub use game_update_state::GameUpdateStateResponse;

use shared_net::op;
use shared_net::sizedbuffers::Bufferable;

pub trait Request: Bufferable {
    const COMMAND: op::Command;
}
