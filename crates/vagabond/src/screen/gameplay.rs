use crate::network::client_gate::{GateCommand, GateIFace};
use crate::system::app_state::AppState;
use crate::system::ui::{font_size, font_size_color, screen_exit, text, Screen, ScreenBundle, HUNDRED, ZERO};
use bevy::prelude::*;
use hall::data::player::player_state::PlayerStatePlayerView;
use hall::message::AttrKind;
use shared_data::game::card::ErgType;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_event::<UiEvent>()
            .add_systems(OnEnter(AppState::Gameplay), gameplay_enter)
            .add_systems(Update, (gameplay_update, button_ui_update, player_ui_update, roll_ui_update).run_if(in_state(AppState::Gameplay)))
            .add_systems(Update, button_next_update.after(button_ui_update).run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), gameplay_exit);
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

fn gameplay_enter(
    // bevy system
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font_info_black = font_size(&asset_server, 16.0);
    let font_info_erg = font_size_color(&asset_server, 48.0, bevy::color::palettes::basic::YELLOW);
    let font_info_green = font_size_color(&asset_server, 16.0, bevy::color::palettes::basic::GREEN);

    let main_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: vec![GridTrack::px(338.0), GridTrack::flex(1.0), GridTrack::px(338.0)],
            grid_template_rows: GridTrack::auto(),
            ..default()
        },
        ..default()
    };
    let attr_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: GridTrack::flex(1.0),
            grid_template_rows: vec![GridTrack::px(372.0), GridTrack::flex(1.0)],
            ..default()
        },
        ..default()
    };
    let attr_values_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
            column_gap: Val::Px(10.0),
            grid_template_rows: GridTrack::flex(1.0),
            ..default()
        },
        ..default()
    };
    let attr_player_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: GridTrack::flex(1.0),
            grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
            row_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(34.0)),
            ..default()
        },
        ..default()
    };
    let center_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: GridTrack::flex(1.0),
            grid_template_rows: vec![GridTrack::px(128.0), GridTrack::px(572.0), GridTrack::px(86.0), GridTrack::flex(1.0)],
            ..default()
        },
        ..default()
    };
    let roll_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::px(290.0), GridTrack::flex(1.0)],
            grid_template_rows: GridTrack::flex(1.0),
            ..default()
        },
        ..default()
    };
    let game_map_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: GridTrack::flex(1.0),
            grid_template_rows: GridTrack::flex(1.0),
            ..default()
        },
        ..default()
    };
    let player_erg_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: vec![GridTrack::px(80.0), GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::px(80.0)],
            grid_template_rows: GridTrack::flex(1.0),
            ..default()
        },
        ..default()
    };
    let player_card_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
            grid_template_rows: GridTrack::flex(1.0),
            ..default()
        },
        ..default()
    };
    let machine_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: GridTrack::flex(1.0),
            grid_template_rows: vec![GridTrack::px(86.0), GridTrack::px(308.0), GridTrack::px(450.0), GridTrack::px(100.0), GridTrack::flex(1.0)],
            ..default()
        },
        ..default()
    };

    let screen = ScreenBundle {
        screen: Screen,
        base: ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: ZERO,
                top: ZERO,
                width: HUNDRED,
                height: HUNDRED,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            image: asset_server.load("image/gameplay.png").into(),
            ..Default::default()
        },
    };

    commands.spawn(screen).with_children(|parent| {
        parent.spawn(main_layout).with_children(|parent| {
            parent.spawn(attr_layout).with_children(|parent| {
                parent.spawn(attr_player_layout).with_children(|parent| {
                    parent.spawn(spacer(Color::NONE));
                });
                parent.spawn(spacer(Color::NONE));
                parent.spawn(spacer(Color::NONE));
            });
            parent.spawn(center_layout).with_children(|parent| {
                parent.spawn(roll_layout).with_children(|parent| {
                    parent.spawn(spacer(Color::NONE));
                    parent.spawn(attr_values_layout.clone()).with_children(|parent| {
                        for i in 0..=3 {
                            parent.spawn((RollText(i), text("-", &font_info_erg)));
                        }
                    });
                    parent.spawn(spacer(Color::NONE));
                });
                parent.spawn(game_map_layout).with_children(|parent| {
                    parent.spawn(spacer(Color::NONE));
                });
                parent.spawn(player_erg_layout).with_children(|parent| {
                    parent.spawn(spacer(Color::NONE));
                    for i in 0..=3 {
                        parent.spawn((ErgText(i), text("00", &font_info_erg)));
                    }
                    parent.spawn(spacer(Color::NONE));
                    parent.spawn(spacer(Color::NONE));
                });
                parent.spawn(player_card_layout).with_children(|parent| {
                    parent.spawn(spacer(Color::NONE));
                });
            });
            parent.spawn(machine_layout).with_children(|parent| {
                parent.spawn(attr_values_layout).with_children(|parent| {
                    parent.spawn(spacer(Color::NONE));
                });
                parent.spawn(spacer(Color::NONE));
                parent.spawn(spacer(Color::NONE));
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

fn button_next_update(
    // bevy system
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
    mut context: ResMut<GameplayContext>,
    gate: Res<GateIFace>,
) {
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

fn button_ui_update(
    // bevy system
    mut interaction_query: Query<ButtonQuery, (Changed<Interaction>, With<Button>)>,
    context: Res<GameplayContext>,
) {
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

fn roll_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut roll_q: Query<(&mut Text, &RollText)>,
) {
    for ui_event in receive.read() {
        if let UiEvent::Roll(roll) = ui_event {
            for (mut roll_text, RollText(index)) in roll_q.iter_mut() {
                roll_text.sections[0].value = format!("{}", roll[*index])
            }
        }
    }
}

fn player_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut erg_q: Query<(&mut Text, &ErgText)>,
) {
    for ui_event in receive.read() {
        if let UiEvent::PlayerState(player_state) = ui_event {
            for (mut erg_text, ErgText(index)) in erg_q.iter_mut() {
                erg_text.sections[0].value = format!("{:02}", player_state.erg[*index])
            }
        }
    }
}

fn gameplay_update(
    // bevy system
    mut gate: ResMut<GateIFace>,
    mut context: ResMut<GameplayContext>,
    mut send: EventWriter<UiEvent>,
) {
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

pub fn gameplay_exit(
    // bevy system
    mut commands: Commands,
    screen_q: Query<Entity, With<Screen>>,
) {
    commands.remove_resource::<GameplayContext>();
    screen_exit(commands, screen_q);
}
