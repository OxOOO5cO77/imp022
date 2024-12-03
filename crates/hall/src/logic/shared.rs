use crate::HallGames;
use hall::data::game::{GameState, GameUser};
use shared_net::op;
use shared_net::types::{AuthType, GameIdType, UserIdType};

pub(crate) fn update_user<T: Default>(games: &mut HallGames, game_id: GameIdType, user: UserIdType, auth: AuthType, command: op::Command, update: impl Fn(&mut GameUser) -> T) -> T {
    if let Some(game) = games.get_mut(&game_id) {
        if let Some(user) = GameState::split_get_user_auth_mut(&mut game.users, user, auth) {
            match user.state.command.try_set(command) {
                Ok(_) => return update(user),
                Err(current) => println!("[Hall] Failed to set command: {:?} (currently: {:?})", command, current),
            }
        }
    }
    T::default()
}
