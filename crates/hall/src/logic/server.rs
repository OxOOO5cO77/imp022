use crate::logic::server::choose_attr::handle_choose_attr;
use crate::logic::server::end_turn::handle_end_turn;
use crate::logic::server::game_build::handle_game_build;
use crate::logic::server::play_card::handle_play_card;
use crate::logic::server::start_turn::handle_start_turn;
use crate::HallContext;
use shared_net::op;
use shared_net::types::GameIdType;

mod choose_attr;
mod end_turn;
mod game_build;
mod play_card;
mod start_turn;

pub(crate) fn handle_phase_complete(context: HallContext, game_id: GameIdType) {
    let mut bx = context.bx.write().unwrap();
    let mut games = context.games.write().unwrap();
    if let Some(game) = games.get_mut(&game_id) {
        if let Some(last_command) = game.determine_last_command() {
            match last_command {
                op::Command::GameBuild => handle_game_build(game, &mut bx),
                op::Command::GameStartTurn => handle_start_turn(game, &mut bx),
                op::Command::GameChooseAttr => handle_choose_attr(game, &mut bx),
                op::Command::GamePlayCard => handle_play_card(game, &mut bx),
                op::Command::GameEndTurn => handle_end_turn(game, &mut bx),
                _ => {}
            }
        }
    }
}
