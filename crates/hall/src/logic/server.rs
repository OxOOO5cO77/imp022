use shared_net::{op, GameIdType};

use crate::logic::server::choose_attr::handle_choose_attr;
use crate::logic::server::choose_intent::handle_choose_intent;
use crate::logic::server::end_turn::handle_end_turn;
use crate::logic::server::game_build::handle_game_build;
use crate::logic::server::play_card::handle_play_card;
use crate::HallContext;

mod choose_attr;
mod choose_intent;
mod end_turn;
mod game_build;
mod play_card;
mod update_mission;
mod update_state;

pub(crate) fn handle_phase_complete(context: HallContext, game_id: GameIdType) {
    let mut bx = context.bx.write().unwrap();
    let mut games = context.games.write().unwrap();
    if let Some(game) = games.get_mut(&game_id) {
        if let Some(last_command) = game.determine_last_command() {
            match last_command {
                op::Command::GameBuild => handle_game_build(game, &mut bx),
                op::Command::GameChooseIntent => handle_choose_intent(game, &mut bx),
                op::Command::GameChooseAttr => handle_choose_attr(game, &mut bx),
                op::Command::GamePlayCard => handle_play_card(game, &mut bx),
                op::Command::GameEndTurn => handle_end_turn(game, &mut bx),
                _ => {}
            }
        }
    }
}
