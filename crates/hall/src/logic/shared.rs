use tracing::error;

use hall_lib::core::GameSubCommand;
use shared_net::{AuthType, GameIdType, UserIdType};

use crate::game::{GameState, GameUser};
use crate::HallGames;

pub(crate) fn update_user<T: Default>(games: &mut HallGames, game_id: GameIdType, user: UserIdType, auth: AuthType, command: GameSubCommand, update: impl Fn(&mut GameUser) -> T) -> T {
    if let Some(game) = games.get_mut(&game_id) {
        if let Some(user) = GameState::split_get_user_auth_mut(&mut game.users, user, auth) {
            match user.state.command.try_set(command) {
                Ok(_) => return update(user),
                Err(current) => error!(game_id, "Failed to set command: {:?} (currently: {:?})", command, current),
            }
        }
    }
    T::default()
}
