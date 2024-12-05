mod shared;
mod server;
mod client;

pub(crate) use client::game_activate::recv_game_activate;
pub(crate) use client::game_build::recv_game_build;
pub(crate) use client::game_choose_attr::recv_game_choose_attr;
pub(crate) use client::game_end_game::recv_game_end_game;
pub(crate) use client::game_end_turn::recv_game_end_turn;
pub(crate) use client::game_play_card::recv_game_play_card;
pub(crate) use client::game_start_turn::recv_game_start_turn;
pub(crate) use client::game_update_state::recv_game_update_state;
pub(crate) use server::handle_phase_complete;
