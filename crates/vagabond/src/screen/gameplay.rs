use crate::network::client_gate::{GateCommand, GateIFace};
use crate::system::app_state::AppState;
use crate::system::ui::{font_size, font_size_color, screen_exit, text, Screen, ScreenBundle, HUNDRED};
use bevy::prelude::*;
use hall::data::player::player_state::PlayerStatePlayerView;
use hall::message::AttrKind;
use shared_data::game::card::ErgType;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UiEvent>().add_systems(OnEnter(AppState::Gameplay), gameplay_enter).add_systems(Update, (gameplay_update, button_ui_update, player_ui_update, roll_ui_update).run_if(in_state(AppState::Gameplay))).add_systems(Update, button_next_update.after(button_ui_update).run_if(in_state(AppState::Gameplay))).add_systems(OnExit(AppState::Gameplay), gameplay_exit);
    }
}

#[derive(Clone, Debug)]
enum WaitKind {
    One,
    All,
}

#[derive(Default, Clone, Debug)]
enum GameplayState {
    #[default]
    Start,
    Pick,
    Play,
    Draw,
    Wait(WaitKind),
}

#[derive(Resource)]
struct GameplayContext {
    state: GameplayState,
}

#[derive(Component)]
struct PhaseText;

#[derive(Component)]
struct ErgText(usize);

#[derive(Component)]
struct RollText(usize);

#[derive(Event)]
enum UiEvent {
    PlayerState(PlayerStatePlayerView),
    Roll([ErgType; 4]),
}

#[derive(Component)]
struct ContinueButton;

fn spacer(color: Color) -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            ..default()
        },
        background_color: color.into(),
        ..default()
    }
}

fn gameplay_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_info_black = font_size(&asset_server, 16.0);
    let font_info_white = font_size_color(&asset_server, 16.0, bevy::color::palettes::basic::WHITE);
    let font_info_green = font_size_color(&asset_server, 16.0, bevy::color::palettes::basic::GREEN);

    let main_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: vec![GridTrack::px(272.0), GridTrack::flex(1.0), GridTrack::px(272.0)],
            grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::px(272.0)],
            ..default()
        },
        ..default()
    };
    let continue_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: vec![GridTrack::auto()],
            grid_template_rows: vec![GridTrack::auto(), GridTrack::px(72.0)],
            ..default()
        },
        ..default()
    };

    commands.spawn(ScreenBundle::default()).with_children(|parent| {
        parent.spawn(main_layout).with_children(|parent| {
            parent.spawn(spacer(Color::BLACK)).with_children(|parent| {
                parent.spawn((RollText(0), text("0", &font_info_white)));
                parent.spawn((RollText(1), text("0", &font_info_white)));
                parent.spawn((RollText(2), text("0", &font_info_white)));
                parent.spawn((RollText(3), text("0", &font_info_white)));
            });
            parent.spawn(spacer(Color::BLACK));
            parent.spawn(spacer(Color::BLACK));

            parent.spawn(spacer(Color::BLACK));
            parent.spawn(spacer(Color::BLACK)).with_children(|parent| {
                parent.spawn((ErgText(0), text("0", &font_info_white)));
                parent.spawn((ErgText(1), text("0", &font_info_white)));
                parent.spawn((ErgText(2), text("0", &font_info_white)));
                parent.spawn((ErgText(3), text("0", &font_info_white)));
            });
            parent.spawn(continue_layout).with_children(|parent| {
                parent.spawn((PhaseText, text("Phase", &font_info_green)));
                parent
                    .spawn((
                        ContinueButton,
                        ButtonBundle {
                            background_color: bevy::color::palettes::css::DARK_GRAY.into(),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(text("Next", &font_info_black));
                    });
            });
        });
    });
    commands.insert_resource(GameplayContext {
        state: GameplayState::default(),
    });
}

fn button_next_update(interaction_q: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>, mut context: ResMut<GameplayContext>, gate: Res<GateIFace>) {
    for &interaction in &interaction_q {
        if interaction == Interaction::Pressed {
            match context.state {
                GameplayState::Start => gate.send_game_start_turn(),
                GameplayState::Pick => gate.send_game_choose_attr(AttrKind::Analyze),
                GameplayState::Play => gate.send_game_play_card(0),
                GameplayState::Draw => gate.send_game_end_turn(),
                GameplayState::Wait(_) => {}
            };
            context.state = GameplayState::Wait(WaitKind::One);
        }
    }
}
type ButtonQuery<'a> = (&'a Interaction, &'a mut BackgroundColor, &'a mut BorderColor);

fn button_ui_update(mut interaction_query: Query<ButtonQuery, (Changed<Interaction>, With<Button>)>, context: Res<GameplayContext>) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = match context.state {
                    GameplayState::Wait(_) => bevy::color::palettes::basic::RED.into(),
                    _ => bevy::color::palettes::basic::GREEN.into(),
                };
                *border_color = bevy::color::palettes::basic::RED.into();
            }
            Interaction::Hovered => {
                *background_color = bevy::color::palettes::basic::SILVER.into();
                *border_color = bevy::color::palettes::basic::WHITE.into();
            }
            Interaction::None => {
                *background_color = bevy::color::palettes::basic::GRAY.into();
                *border_color = bevy::color::palettes::basic::BLACK.into();
            }
        }
    }
}

fn roll_ui_update(mut receive: EventReader<UiEvent>, mut roll_q: Query<(&mut Text, &RollText)>) {
    for ui_event in receive.read() {
        if let UiEvent::Roll(roll) = ui_event {
            for (mut roll_text, RollText(index)) in roll_q.iter_mut() {
                roll_text.sections[0].value = format!("{}", roll[*index])
            }
        }
    }
}

fn player_ui_update(mut receive: EventReader<UiEvent>, mut erg_q: Query<(&mut Text, &ErgText)>) {
    for ui_event in receive.read() {
        if let UiEvent::PlayerState(player_state) = ui_event {
            for (mut erg_text, ErgText(index)) in erg_q.iter_mut() {
                erg_text.sections[0].value = format!("{}", player_state.erg[*index])
            }
        }
    }
}

fn gameplay_update(mut gate: ResMut<GateIFace>, mut context: ResMut<GameplayContext>, mut send: EventWriter<UiEvent>) {
    match gate.grx.try_recv() {
        Ok(GateCommand::GameStartTurn(gate_response)) => {
            if gate_response.success {
                println!("State: GameStartTurn");
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameRoll(gate_response)) => {
            println!("State: GameRoll");
            context.state = GameplayState::Pick;
            send.send(UiEvent::Roll(gate_response.roll));
        }
        Ok(GateCommand::GameChooseAttr(gate_response)) => {
            if gate_response.success {
                println!("State: GameChooseAttr");
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameResources(gate_response)) => {
            println!("State: GameResources");
            send.send(UiEvent::PlayerState(gate_response.player_state_view));
            context.state = GameplayState::Play;
        }
        Ok(GateCommand::GamePlayCard(gate_response)) => {
            if gate_response.success {
                println!("State: GamePlayCard");
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameResolveCards(gate_response)) => {
            println!("State: GameResolveCards");
            context.state = GameplayState::Draw;
        }
        Ok(GateCommand::GameEndTurn(gate_response)) => {
            if gate_response.success {
                println!("State: GameEndTurn");
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameTick(gate_response)) => {
            println!("State: GameTick");
            context.state = GameplayState::Start;
        }
        Ok(GateCommand::GameEndGame(gate_response)) => {
            if gate_response.success {
                println!("State: GameEndGame");
            }
        }
        Ok(GateCommand::GameUpdateState(gate_response)) => {
            println!("State: GameUpdateState");
            send.send(UiEvent::PlayerState(gate_response.player_state));
        }
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn gameplay_exit(mut commands: Commands, screen_q: Query<Entity, With<Screen>>) {
    commands.remove_resource::<GameplayContext>();
    screen_exit(commands, screen_q);
}
