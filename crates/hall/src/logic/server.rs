use hall_lib::core::GameSubCommand;
use shared_net::GameIdType;

use crate::HallContext;
use crate::logic::server::choose_attr::handle_choose_attr;
use crate::logic::server::choose_intent::handle_choose_intent;
use crate::logic::server::end_turn::handle_end_turn;
use crate::logic::server::game_build::handle_game_build;
use crate::logic::server::play_card::handle_play_card;

mod choose_attr;
mod choose_intent;
mod end_turn;
mod game_build;
mod play_card;
mod update_mission;
mod update_state;
mod update_tokens;

pub(crate) fn handle_phase_complete(context: HallContext, game_id: GameIdType) {
    let mut bx = context.bx.write().unwrap();
    let mut games = context.games.write().unwrap();
    let dm = context.data_manager.read().unwrap();
    if let Some(game) = games.get_mut(&game_id) {
        if let Some(last_command) = game.determine_last_command() {
            match last_command {
                GameSubCommand::Build => handle_game_build(game, &mut bx),
                GameSubCommand::ChooseIntent => handle_choose_intent(game, &mut bx, &dm),
                GameSubCommand::ChooseAttr => handle_choose_attr(game, &mut bx),
                GameSubCommand::PlayCard => handle_play_card(game, &mut bx),
                GameSubCommand::EndTurn => handle_end_turn(game, &mut bx),
                _ => {}
            }
        }
    }
}
