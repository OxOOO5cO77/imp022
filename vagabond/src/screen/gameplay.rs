use bevy::prelude::*;

use crate::system::app_state::AppState;
use crate::system::ui::{screen_exit, ScreenBundle};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameplay), gameplay_enter)
            .add_systems(Update, gameplay_update.run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), screen_exit)
        ;
    }
}

#[derive(Default)]
enum GameplayState {
    #[default] Draw,
    Roll,
    Pick,
    Assign,
    Play,
    Resolve,
    Update,
}

#[derive(Default, Component)]
struct Gameplay {
    state: GameplayState,
}

fn gameplay_enter(mut commands: Commands) {
    commands.spawn(ScreenBundle::default())
        .with_children( |parent| {
           parent.spawn(Gameplay::default());
        })
    ;
}

fn gameplay_update(mut gameplay_q: Query<&mut Gameplay>) {
    let gameplay = gameplay_q.single_mut();

    match gameplay.state {
        GameplayState::Draw => gameplay_state_draw(gameplay),
        GameplayState::Roll => gameplay_state_roll(gameplay),
        GameplayState::Pick => gameplay_state_pick(gameplay),
        GameplayState::Assign => gameplay_state_assign(gameplay),
        GameplayState::Play => gameplay_state_play(gameplay),
        GameplayState::Resolve => gameplay_state_resolve(gameplay),
        GameplayState::Update => gameplay_state_update(gameplay),
    }
}


fn gameplay_state_draw(mut gameplay: Mut<Gameplay>) {
    gameplay.state = GameplayState::Roll;
}

fn gameplay_state_roll(mut gameplay: Mut<Gameplay>) {
    gameplay.state = GameplayState::Pick;
}

fn gameplay_state_pick(mut gameplay: Mut<Gameplay>) {
    gameplay.state = GameplayState::Assign;
}

fn gameplay_state_assign(mut gameplay: Mut<Gameplay>) {
    gameplay.state = GameplayState::Play;
}

fn gameplay_state_play(mut gameplay: Mut<Gameplay>) {
    gameplay.state = GameplayState::Resolve;
}

fn gameplay_state_resolve(mut gameplay: Mut<Gameplay>) {
    gameplay.state = GameplayState::Update;
}

fn gameplay_state_update(mut gameplay: Mut<Gameplay>) {
    gameplay.state = GameplayState::Draw;
}
