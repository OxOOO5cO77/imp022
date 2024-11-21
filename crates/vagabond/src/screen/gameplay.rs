use crate::manager::DataManager;
use crate::network::client_gate::{GateCommand, GateIFace};
use crate::screen::compose::PlayerCache;
use crate::system::app_state::AppState;
use crate::system::ui::{font_size, font_size_color, screen_exit, text, text_centered, FontInfo, Screen, ScreenBundle, HUNDRED, ZERO};
use bevy::prelude::*;
use hall::data::game::GameMachinePlayerView;
use hall::data::player::player_state::PlayerStatePlayerView;
use hall::message::{AttrKind, CardIdxType, CardTarget};
use shared_data::build::BuildValueType;
use shared_data::card::{DelayType, ErgType};
use std::cmp::Ordering;
use std::collections::HashMap;
use vagabond::data::VagabondCard;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_event::<UiEvent>()
            .add_systems(OnEnter(AppState::Gameplay), gameplay_enter)
            .add_systems(Update, (gameplay_update, button_next_ui_update, local_ui_update, roll_ui_update, remote_ui_update, machine_ui_update).run_if(in_state(AppState::Gameplay)))
            .add_systems(Update, (button_next_update, button_attribute_update).after(button_next_ui_update).run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), gameplay_exit);
    }
}

#[derive(Clone, Debug, PartialEq)]
enum WaitKind {
    One,
    All,
}

#[derive(Default, Clone, Debug, PartialEq)]
enum GameplayState {
    #[default]
    Start,
    Pick,
    Play,
    Draw,
    Wait(WaitKind),
}

#[derive(Resource, Default)]
struct GameplayContext {
    state: GameplayState,
    attr_pick: Option<AttrKind>,
    card_picks: HashMap<CardIdxType, CardTarget>,
}

impl GameplayContext {
    fn reset(&mut self) {
        self.attr_pick = None;
        self.card_picks.clear();
    }
}

#[derive(Component)]
struct PhaseText;

#[derive(Component)]
struct RemoteAttrText(usize);

#[derive(Component)]
struct ErgText(usize);

#[derive(Component)]
struct RollText(usize);

#[derive(Component)]
struct AttributeText(usize, usize);

#[derive(Component)]
struct AttributeButton(AttrKind);

#[derive(Bundle)]
struct AttributeButtonBundle {
    node: NodeBundle,
    marker: AttributeButton,
    button: Button,
    interaction: Interaction,
}

impl AttributeButtonBundle {
    fn new(kind: AttrKind) -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: HUNDRED,
                    height: HUNDRED,
                    grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
                    column_gap: Val::Px(10.0),
                    grid_template_rows: GridTrack::flex(1.0),
                    ..default()
                },
                ..default()
            },
            marker: AttributeButton(kind),
            button: Default::default(),
            interaction: Default::default(),
        }
    }
    fn map_kind(kind: AttrKind) -> (&'static str, usize) {
        match kind {
            AttrKind::Analyze => ("A", 0),
            AttrKind::Breach => ("B", 1),
            AttrKind::Compute => ("C", 2),
            AttrKind::Disrupt => ("D", 3),
        }
    }
    fn spawn(parent: &mut ChildBuilder, kind: AttrKind, values: &[[BuildValueType; 4]; 4], font_info: &FontInfo) {
        parent.spawn(Self::new(kind)).with_children(|parent| {
            let (header, row_idx) = Self::map_kind(kind);
            parent.spawn(text_centered(header, font_info));
            for (idx, value) in values[row_idx].iter().enumerate() {
                parent.spawn((AttributeText(row_idx, idx), text_centered(value.to_string(), font_info)));
            }
        });
    }
}

#[derive(Component, Clone)]
struct CardLayout {
    slot: usize,
    title: Entity,
    cost: Entity,
    launch: Entity,
    run: Entity,
}

impl CardLayout {
    fn new(slot: usize) -> Self {
        Self {
            slot,
            title: Entity::PLACEHOLDER,
            cost: Entity::PLACEHOLDER,
            launch: Entity::PLACEHOLDER,
            run: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Component)]
struct CardText;

#[derive(Component)]
struct CardButton(usize);

#[derive(Bundle)]
struct CardBundle {
    node: NodeBundle,
    marker: CardButton,
    button: Button,
    interaction: Interaction,
}

impl CardBundle {
    fn new(slot: usize) -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: HUNDRED,
                    height: HUNDRED,
                    grid_template_columns: GridTrack::flex(1.0),
                    grid_template_rows: RepeatedGridTrack::flex(5, 1.0),
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },

            marker: CardButton(slot),
            button: Default::default(),
            interaction: Default::default(),
        }
    }
    fn spawn(parent: &mut ChildBuilder, slot: usize, card: Option<VagabondCard>, font_info: &FontInfo) {
        let mut layout = CardLayout::new(slot);
        parent
            .spawn(Self::new(slot))
            .with_children(|parent| {
                layout.title = parent.spawn((CardText, text_centered(card.as_ref().map_or("", |o| o.title.as_str()), font_info))).id();
                layout.cost = parent.spawn((CardText, text_centered(card.as_ref().map_or("-".to_string(), |o| o.cost.to_string()), font_info))).id();
                layout.launch = parent.spawn((CardText, text_centered(card.as_ref().map_or("", |o| o.launch_rules.as_str()), font_info))).id();
                layout.run = parent.spawn((CardText, text_centered(card.as_ref().map_or("", |o| o.run_rules.as_str()), font_info))).id();
            })
            .insert(layout);
    }
}

#[derive(Bundle)]
struct NextButtonBundle {
    marker: NextButton,
    button: ButtonBundle,
}

impl NextButtonBundle {
    fn new() -> Self {
        Self {
            marker: NextButton,
            button: ButtonBundle {
                background_color: bevy::color::palettes::css::DARK_GRAY.into(),
                style: Style {
                    width: Val::Percent(90.0),
                    height: Val::Percent(90.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
        }
    }
    fn spawn(parent: &mut ChildBuilder, font_info: &FontInfo) {
        parent.spawn(Self::new()).with_children(|parent| {
            parent.spawn(text_centered("Next", font_info));
        });
    }
}

#[derive(Component)]
struct MachineNameText;

#[derive(Component)]
struct MachineStatText(usize);

#[derive(Component)]
struct MachineCurrentProgramText;

#[derive(Component)]
struct MachineQueueItem(DelayType);

#[derive(Component)]
struct MachineProcessText(usize);

#[derive(Bundle)]
struct MachineBundle {
    node: NodeBundle,
    machine_kind: MachineKind,
}

impl MachineBundle {
    fn new(machine_kind: MachineKind, border_color: Srgba) -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: HUNDRED,
                    height: HUNDRED,
                    grid_template_columns: GridTrack::flex(1.0),
                    grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(4.0), GridTrack::flex(2.0), GridTrack::flex(6.0)],
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: bevy::color::palettes::basic::SILVER.into(),
                border_color: border_color.into(),
                ..default()
            },
            machine_kind,
        }
    }
    fn spawn(parent: &mut ChildBuilder, machine_kind: MachineKind, name: impl Into<String>, border_color: Srgba, font_info: &FontInfo) {
        let machine_layout = NodeBundle {
            style: Style {
                padding: UiRect::new(Val::Px(40.0), Val::Px(40.0), Val::Px(10.0), Val::Px(10.0)),
                ..default()
            },
            ..default()
        };

        let stats_layout = NodeBundle {
            style: Style {
                display: Display::Grid,
                width: HUNDRED,
                height: HUNDRED,
                grid_template_columns: vec![GridTrack::flex(2.0), GridTrack::flex(1.0), GridTrack::flex(1.0)],
                grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                ..default()
            },
            ..default()
        };

        let stats_graphic = NodeBundle {
            style: Style {
                width: Val::Percent(90.0),
                height: Val::Percent(90.0),
                grid_row: GridPlacement::span(2),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            background_color: border_color.into(),
            ..default()
        };

        let current_layout = NodeBundle {
            style: Style {
                display: Display::Grid,
                width: HUNDRED,
                height: HUNDRED,
                grid_template_columns: GridTrack::flex(1.0),
                grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                ..default()
            },
            ..default()
        };

        let queue_layout = NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(80.0),
                height: HUNDRED,
                grid_template_columns: RepeatedGridTrack::flex(10, 1.0),
                column_gap: Val::Px(10.0),
                grid_template_rows: GridTrack::flex(1.0),
                ..default()
            },
            ..default()
        };

        let queue_item = NodeBundle {
            style: Style {
                display: Display::Grid,
                width: HUNDRED,
                height: HUNDRED,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::WHITE.into(),
            border_color: Color::BLACK.into(),
            ..default()
        };

        let process_layout = NodeBundle {
            style: Style {
                display: Display::Grid,
                width: HUNDRED,
                height: Val::Percent(80.0),
                grid_template_columns: GridTrack::flex(1.0),
                grid_template_rows: vec![GridTrack::flex(1.5), GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::flex(1.0)],
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        };

        parent.spawn(machine_layout).with_children(|parent| {
            parent.spawn(Self::new(machine_kind, border_color)).with_children(|parent| {
                parent.spawn(NodeBundle::default()).with_children(|parent| {
                    parent.spawn((machine_kind, MachineNameText, text(name, AlignSelf::Center, JustifySelf::Start, font_info)));
                });

                parent.spawn(stats_layout).with_children(|parent| {
                    parent.spawn(stats_graphic);
                    parent.spawn((machine_kind, MachineStatText(0), text_centered("[1] meow", font_info)));
                    parent.spawn((machine_kind, MachineStatText(1), text_centered("[2] meow", font_info)));
                    parent.spawn((machine_kind, MachineStatText(2), text_centered("[3] meow", font_info)));
                    parent.spawn((machine_kind, MachineStatText(3), text_centered("[4] meow", font_info)));
                });
                parent.spawn(current_layout).with_children(|parent| {
                    parent.spawn(NodeBundle::default()).with_children(|parent| {
                        parent.spawn((machine_kind, MachineCurrentProgramText, text(" v- <idle>", AlignSelf::Start, JustifySelf::Center, font_info)));
                    });
                    parent.spawn(queue_layout).with_children(|parent| {
                        for i in 0..10 {
                            parent.spawn((machine_kind, MachineQueueItem(i), queue_item.clone()));
                        }
                    });
                });
                parent.spawn(process_layout).with_children(|parent| {
                    parent.spawn(text_centered("--Running Processes--", font_info));
                    parent.spawn(NodeBundle::default()).with_children(|parent| {
                        parent.spawn((machine_kind, MachineProcessText(0), text("?", AlignSelf::Start, JustifySelf::Center, font_info)));
                    });
                    parent.spawn(NodeBundle::default()).with_children(|parent| {
                        parent.spawn((machine_kind, MachineProcessText(1), text("?", AlignSelf::Start, JustifySelf::Center, font_info)));
                    });
                    parent.spawn(NodeBundle::default()).with_children(|parent| {
                        parent.spawn((machine_kind, MachineProcessText(2), text("?", AlignSelf::Start, JustifySelf::Center, font_info)));
                    });
                    parent.spawn(NodeBundle::default()).with_children(|parent| {
                        parent.spawn((machine_kind, MachineProcessText(3), text("?", AlignSelf::Start, JustifySelf::Center, font_info)));
                    });
                });
            });
        });
    }
}

#[derive(Component, Copy, Clone, PartialEq)]
enum MachineKind {
    Local,
    Remote,
}

#[derive(Event)]
enum UiEvent {
    PlayerState(PlayerStatePlayerView),
    ChooseAttr(Option<usize>),
    Roll([ErgType; 4]),
    Resources([ErgType; 4], [ErgType; 4], [BuildValueType; 4]),
    MachineUpdate(GameMachinePlayerView, GameMachinePlayerView),
}

#[derive(Component)]
struct NextButton;

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
    player_cache: Res<PlayerCache>,
    dm: Res<DataManager>,
) {
    let font_info_black = font_size(&asset_server, 16.0);
    let font_info_gray = font_size_color(&asset_server, 48.0, bevy::color::palettes::basic::GRAY);
    let font_info_green = font_size_color(&asset_server, 16.0, bevy::color::palettes::basic::GREEN);
    let font_info_card = font_size_color(&asset_server, 16.0, bevy::color::palettes::basic::GRAY);

    let main_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: vec![GridTrack::px(422.0), GridTrack::flex(1.0), GridTrack::px(422.0)],
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
            grid_template_rows: vec![GridTrack::px(368.0), GridTrack::px(290.0), GridTrack::flex(1.0)],
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
            padding: UiRect::all(Val::Px(30.0)),
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
            grid_template_rows: vec![GridTrack::px(128.0), GridTrack::px(572.0), GridTrack::flex(1.0)],
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
    let roll_values_layout = NodeBundle {
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
    let player_erg_card_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
            column_gap: Val::Px(4.0),
            grid_template_rows: vec![GridTrack::px(86.0), GridTrack::flex(1.0)],
            ..default()
        },
        ..default()
    };
    let remote_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: GridTrack::flex(1.0),
            grid_template_rows: vec![GridTrack::px(130.0), GridTrack::px(290.0), GridTrack::px(312.0), GridTrack::flex(1.0)],
            ..default()
        },
        ..default()
    };
    let remote_attr_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
            column_gap: Val::Px(10.0),
            grid_template_rows: GridTrack::flex(1.0),
            padding: UiRect::new(Val::Px(66.0), Val::Px(66.0), Val::Px(32.0), Val::Px(32.0)),
            ..default()
        },
        ..default()
    };
    let turn_control_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: GridTrack::flex(1.0),
            grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::px(100.0)],
            padding: UiRect::new(Val::Px(28.0), Val::Px(28.0), Val::Px(18.0), Val::Px(18.0)),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
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
                    AttributeButtonBundle::spawn(parent, AttrKind::Analyze, &player_cache.attr, &font_info_gray);
                    AttributeButtonBundle::spawn(parent, AttrKind::Breach, &player_cache.attr, &font_info_gray);
                    AttributeButtonBundle::spawn(parent, AttrKind::Compute, &player_cache.attr, &font_info_gray);
                    AttributeButtonBundle::spawn(parent, AttrKind::Disrupt, &player_cache.attr, &font_info_gray);
                });
                let name = format!("{} [{}]", &player_cache.bio.name, &player_cache.bio.id);
                MachineBundle::spawn(parent, MachineKind::Local, name, bevy::color::palettes::basic::GREEN, &font_info_black);
                parent.spawn(spacer(Color::NONE));
            });
            parent.spawn(center_layout).with_children(|parent| {
                parent.spawn(roll_layout).with_children(|parent| {
                    parent.spawn(spacer(Color::NONE));
                    parent.spawn(roll_values_layout).with_children(|parent| {
                        for i in 0..4 {
                            parent.spawn((RollText(i), text_centered("-", &font_info_gray)));
                        }
                    });
                    parent.spawn(spacer(Color::NONE));
                });
                parent.spawn(game_map_layout).with_children(|parent| {
                    parent.spawn(spacer(Color::NONE));
                });
                parent.spawn(player_erg_card_layout).with_children(|parent| {
                    //erg
                    for i in 0..4 {
                        parent.spawn((ErgText(i), text_centered("00", &font_info_gray)));
                    }
                    parent.spawn(spacer(Color::NONE));

                    //cards
                    for i in 0..5 {
                        CardBundle::spawn(parent, i, None, &font_info_card);
                    }
                });
            });
            parent.spawn(remote_layout).with_children(|parent| {
                parent.spawn(remote_attr_layout).with_children(|parent| {
                    for i in 0..4 {
                        parent.spawn((RemoteAttrText(i), text_centered("?", &font_info_gray)));
                    }
                });
                MachineBundle::spawn(parent, MachineKind::Remote, dm.node_name(player_cache.mission, 1), bevy::color::palettes::basic::RED, &font_info_black);
                parent.spawn(spacer(Color::NONE));
                parent.spawn(turn_control_layout).with_children(|parent| {
                    parent.spawn((PhaseText, text_centered("Phase", &font_info_green)));
                    NextButtonBundle::spawn(parent, &font_info_black);
                });
            });
        });
    });
    commands.insert_resource(GameplayContext::default());
}

fn button_next_update(
    // bevy system
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<NextButton>)>,
    mut context: ResMut<GameplayContext>,
    gate: Res<GateIFace>,
) {
    for &interaction in &interaction_q {
        if interaction == Interaction::Pressed {
            match context.state {
                GameplayState::Start => gate.send_game_start_turn(),
                GameplayState::Pick => {
                    if let Some(kind) = context.attr_pick {
                        gate.send_game_choose_attr(kind);
                    } else {
                        continue;
                    }
                }
                GameplayState::Play => gate.send_game_play_cards(&context.card_picks),
                GameplayState::Draw => gate.send_game_end_turn(),
                GameplayState::Wait(_) => {}
            };
            context.state = GameplayState::Wait(WaitKind::One);
        }
    }
}

fn button_attribute_update(
    // bevy system
    interaction_q: Query<(&AttributeButton, &Interaction), Changed<Interaction>>,
    mut context: ResMut<GameplayContext>,
    mut send: EventWriter<UiEvent>,
) {
    if context.state != GameplayState::Pick {
        return;
    }
    for (AttributeButton(kind), &interaction) in &interaction_q {
        if interaction == Interaction::Pressed {
            context.attr_pick = Some(*kind);
            send.send(UiEvent::ChooseAttr(Some(map_kind_to_row(*kind))));
        }
    }
}

fn map_kind_to_row(kind: AttrKind) -> usize {
    match kind {
        AttrKind::Analyze => 0,
        AttrKind::Breach => 1,
        AttrKind::Compute => 2,
        AttrKind::Disrupt => 3,
    }
}

type ButtonQueryParams<'a> = (&'a Interaction, &'a mut BackgroundColor, &'a mut BorderColor);
type ButtonQueryConditions = (Changed<Interaction>, With<NextButton>);
fn button_next_ui_update(
    // bevy system
    mut interaction_query: Query<ButtonQueryParams, ButtonQueryConditions>,
    context: Res<GameplayContext>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = match context.state {
                    GameplayState::Wait(WaitKind::One) => bevy::color::palettes::basic::RED.into(),
                    GameplayState::Wait(WaitKind::All) => bevy::color::palettes::basic::YELLOW.into(),
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
        match ui_event {
            UiEvent::Roll(roll) => {
                for (mut roll_text, RollText(index)) in roll_q.iter_mut() {
                    roll_text.sections[0].value = format!("{}", roll[*index]);
                    roll_text.sections[0].style.color = bevy::color::palettes::basic::GRAY.into();
                }
            }
            UiEvent::Resources(local_erg, remote_erg, _) => {
                for (mut roll_text, RollText(index)) in roll_q.iter_mut() {
                    roll_text.sections[0].style.color = match local_erg[*index].cmp(&remote_erg[*index]) {
                        Ordering::Less => bevy::color::palettes::basic::RED,
                        Ordering::Equal => bevy::color::palettes::basic::YELLOW,
                        Ordering::Greater => bevy::color::palettes::basic::GREEN,
                    }
                        .into();
                }
            }
            _ => {}
        }
    }
}

type ErgTextQuery<'a> = Query<'a, 'a, (&'a mut Text, &'a ErgText)>;
type AttributeTextQuery<'a> = Query<'a, 'a, (&'a mut Text, &'a AttributeText)>;
type CardLayoutQuery<'a> = Query<'a, 'a, &'a CardLayout>;
type CardTextQuery<'a> = Query<'a, 'a, &'a mut Text, With<CardText>>;

fn local_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut text_q: ParamSet<(AttributeTextQuery, ErgTextQuery, CardLayoutQuery, CardTextQuery)>,
    dm: Res<DataManager>,
) {
    for ui_event in receive.read() {
        match ui_event {
            UiEvent::PlayerState(player_state) => {
                for (mut attr_text, AttributeText(row, col)) in text_q.p0().iter_mut() {
                    attr_text.sections[0].value = format!("{}", player_state.attr[*row][*col]);
                }
                for (mut erg_text, ErgText(index)) in text_q.p1().iter_mut() {
                    erg_text.sections[0].value = format!("{:02}", player_state.erg[*index])
                }

                let layouts = text_q.p2().iter().cloned().collect::<Vec<_>>();

                for layout in &layouts {
                    let card = player_state.hand.get(layout.slot).and_then(|o| dm.convert_card(o));

                    if let Ok(mut title_text) = text_q.p3().get_mut(layout.title) {
                        title_text.sections[0].value = card.as_ref().map_or("<Empty>".to_string(), |o| o.title.clone());
                    }
                    if let Ok(mut cost_text) = text_q.p3().get_mut(layout.cost) {
                        cost_text.sections[0].value = card.as_ref().map_or("<Empty>".to_string(), |o| o.cost.to_string());
                    }
                    if let Ok(mut launch_text) = text_q.p3().get_mut(layout.launch) {
                        launch_text.sections[0].value = card.as_ref().map_or("<Empty>".to_string(), |o| o.launch_rules.clone());
                    }
                    if let Ok(mut run_text) = text_q.p3().get_mut(layout.run) {
                        run_text.sections[0].value = card.as_ref().map_or("<Empty>".to_string(), |o| o.run_rules.clone());
                    }
                }
            }
            UiEvent::ChooseAttr(kind) => {
                for (mut attr_text, AttributeText(row, _)) in text_q.p0().iter_mut() {
                    attr_text.sections[0].style.color = if Some(row) == kind.into() {
                        bevy::color::palettes::basic::GREEN
                    } else {
                        bevy::color::palettes::basic::GRAY
                    }
                        .into();
                }
            }
            _ => {}
        }
    }
}

type RemoteAttrTextParams<'a> = (&'a mut Text, &'a RemoteAttrText);

fn remote_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut text_q: Query<RemoteAttrTextParams>,
) {
    for ui_event in receive.read() {
        match ui_event {
            UiEvent::Roll(_) => {
                for (mut attr_text, RemoteAttrText(_)) in text_q.iter_mut() {
                    attr_text.sections[0].value = "?".to_string();
                    attr_text.sections[0].style.color = bevy::color::palettes::basic::GRAY.into();
                }
            }
            UiEvent::Resources(_, _, remote_attr) => {
                for (mut attr_text, RemoteAttrText(index)) in text_q.iter_mut() {
                    attr_text.sections[0].value = remote_attr[*index].to_string();
                    attr_text.sections[0].style.color = bevy::color::palettes::basic::RED.into();
                }
            }
            _ => {}
        }
    }
}

type MachineStatTextQuery<'a> = Query<'a, 'a, (&'a MachineKind, &'a mut Text, &'a MachineStatText)>;
type MachineCurrentProgramTextQuery<'a> = Query<'a, 'a, (&'a MachineKind, &'a mut Text, &'a MachineCurrentProgramText)>;
type MachineQueueItemQuery<'a> = Query<'a, 'a, (&'a MachineKind, &'a mut BackgroundColor, &'a MachineQueueItem)>;
type MachineProcessTextQuery<'a> = Query<'a, 'a, (&'a MachineKind, &'a mut Text, &'a MachineProcessText)>;

fn machine_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut machine_q: ParamSet<(MachineStatTextQuery, MachineCurrentProgramTextQuery, MachineQueueItemQuery, MachineProcessTextQuery)>,
    dm: Res<DataManager>,
) {
    for ui_event in receive.read() {
        if let UiEvent::MachineUpdate(local, remote) = ui_event {
            for (machine_component, mut text, MachineStatText(index)) in machine_q.p0().iter_mut() {
                let machine = if *machine_component == MachineKind::Local {
                    local
                } else {
                    remote
                };
                text.sections[0].value = machine.stats[*index].to_string();
            }

            for (machine_component, mut text, MachineCurrentProgramText) in machine_q.p1().iter_mut() {
                let machine = if *machine_component == MachineKind::Local {
                    local
                } else {
                    remote
                };
                let mut result = " v- <idle>".to_string();
                if let Some(current) = machine.queue.iter().find(|item| item.delay == 0) {
                    if let Some(card) = dm.convert_card(&current.player_card) {
                        result = format!(" v- {}", card.title);
                    }
                }
                text.sections[0].value = result;
            }

            for (machine_component, mut color, MachineQueueItem(index)) in machine_q.p2().iter_mut() {
                let machine = if *machine_component == MachineKind::Local {
                    local
                } else {
                    remote
                };
                *color = if let Some(process) = machine.queue.iter().find(|item| item.delay == *index) {
                    if process.local {
                        bevy::color::palettes::basic::GREEN
                    } else {
                        bevy::color::palettes::basic::RED
                    }
                } else {
                    bevy::color::palettes::basic::WHITE
                }
                    .into();
            }

            for (machine_component, mut text, MachineProcessText(index)) in machine_q.p3().iter_mut() {
                let machine = if *machine_component == MachineKind::Local {
                    local
                } else {
                    remote
                };
                let mut result = "?".to_string();
                if let Some(process) = machine.running.get(*index) {
                    if let Some(card) = dm.convert_card(&process.player_card) {
                        result = format!("v- {}", card.title);
                    }
                }
                text.sections[0].value = result;
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
            println!("[RECV] GameStartTurn {}", if gate_response.success { "OK" } else { "ERROR" });
            if gate_response.success {
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameRoll(gate_response)) => {
            println!("[RECV] GameRoll => Pick");
            context.state = GameplayState::Pick;
            send.send(UiEvent::Roll(gate_response.roll));
            send.send(UiEvent::ChooseAttr(None));
        }
        Ok(GateCommand::GameChooseAttr(gate_response)) => {
            println!("[RECV] GameChooseAttr {}", if gate_response.success { "OK" } else { "ERROR" });
            if gate_response.success {
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameResources(gate_response)) => {
            println!("[RECV] GameResources => Play");
            send.send(UiEvent::PlayerState(gate_response.player_state_view));
            send.send(UiEvent::Resources(gate_response.local_erg, gate_response.remote_erg, gate_response.remote_attr));
            context.state = GameplayState::Play;
        }
        Ok(GateCommand::GamePlayCard(gate_response)) => {
            let success = gate_response.success.iter().all(|&success| success);
            println!("[RECV] GamePlayCard {}", if success { "OK" } else { "ERROR" });
            if success {
                println!("[RECV]  OK");
                context.card_picks.clear();
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameResolveCards(_gate_response)) => {
            println!("[RECV] GameResolveCards => Draw");
            context.state = GameplayState::Draw;
        }
        Ok(GateCommand::GameEndTurn(gate_response)) => {
            println!("[RECV] GameEndTurn {}", if gate_response.success { "OK" } else { "ERROR" });
            if gate_response.success {
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameTick(_gate_response)) => {
            println!("[RECV] GameTick");
            context.reset();
            context.state = GameplayState::Start;
        }
        Ok(GateCommand::GameEndGame(gate_response)) => {
            println!("[RECV] GameEndGame {}", if gate_response.success { "OK" } else { "ERROR" });
        }
        Ok(GateCommand::GameUpdateState(gate_response)) => {
            println!("[RECV] GameUpdateState");
            send.send(UiEvent::PlayerState(gate_response.player_state));
            send.send(UiEvent::MachineUpdate(gate_response.local_machine, gate_response.remote_machine));
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
    commands.remove_resource::<PlayerCache>();
    screen_exit(commands, screen_q);
}
