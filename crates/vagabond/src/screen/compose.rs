use bevy::prelude::*;
use pyri_tooltip::TooltipContent;
use shared_data::player::attribute::AttributeValueType;
use vagabond::data::vagabond_part::VagabondPart;
use warehouse::data::player_bio::PlayerBio;
use crate::manager::{DataManager, WarehouseManager};
use crate::network::client_gate::{GateCommand, GateIFace};
use crate::screen::compose::StatRowKind::{Build, Detail};
use crate::system::app_state::AppState;
use crate::system::dragdrop::{DragDrag, DragDrop, DragTarget, Dragging, DropTarget};
use crate::system::ui::{filled_rect, font_size, screen_exit, text, FontInfo, Screen, ScreenBundle, HUNDRED};

pub struct ComposePlugin;

impl Plugin for ComposePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_event::<FinishPlayer>()
            .add_systems(OnEnter(AppState::ComposeInit), composeinit_enter)
            .add_systems(Update, composeinit_update.run_if(in_state(AppState::ComposeInit)))
            .add_systems(OnEnter(AppState::Compose), compose_enter)
            .add_systems(Update, (drag_drag, drag_drop, finish_player, compose_update, button_update).run_if(in_state(AppState::Compose)))
            .add_systems(Update, button_commit_update.after(button_update).run_if(in_state(AppState::Compose)))
            .add_systems(OnExit(AppState::Compose), compose_exit);
    }
}

#[derive(Resource)]
struct ComposeInitHandoff {
    parts: [VagabondPart; 8],
}

fn composeinit_enter(gate: ResMut<GateIFace>) {
    gate.send_game_activate();
}

fn composeinit_update(
    // bevy system
    mut commands: Commands,
    mut gate: ResMut<GateIFace>,
    mut app_state: ResMut<NextState<AppState>>,
    dm: Res<DataManager>,
) {
    if let Ok(GateCommand::GameActivate(response)) = gate.grx.try_recv() {
        let init_handoff = ComposeInitHandoff {
            parts: [dm.convert_part(&response.parts[0]).unwrap_or_default(), dm.convert_part(&response.parts[1]).unwrap_or_default(), dm.convert_part(&response.parts[2]).unwrap_or_default(), dm.convert_part(&response.parts[3]).unwrap_or_default(), dm.convert_part(&response.parts[4]).unwrap_or_default(), dm.convert_part(&response.parts[5]).unwrap_or_default(), dm.convert_part(&response.parts[6]).unwrap_or_default(), dm.convert_part(&response.parts[7]).unwrap_or_default()],
        };
        gate.game_id = response.game_id;
        commands.insert_resource(init_handoff);
        app_state.set(AppState::Compose)
    }
}

#[derive(Resource, Default, PartialEq)]
enum ComposeState {
    #[default]
    Build,
    Ready,
    Committed,
}

#[derive(Event)]
struct FinishPlayer;

#[derive(Component)]
struct SubmitButton;

enum StatRowKind {
    Analyze,
    Breach,
    Compute,
    Disrupt,
    Build,
    Detail,
}

#[derive(Component)]
enum PlayerPartHolderKind {
    StatRow(StatRowKind),
    Build,
    Detail,
    Unallocated,
}

#[derive(Component, Clone)]
struct PlayerPartHolder(Option<VagabondPart>);

#[derive(Component, Clone)]
struct CardHolder(usize);

#[derive(Component)]
enum InfoKind {
    Name,
    ID,
    Birthplace,
    DoB,
}

#[derive(Resource, Default)]
pub(crate) struct PlayerCache {
    bio: PlayerBio,
    pub(crate) attr: [[AttributeValueType;4];4],
}

const ATTRIB_SIZE: f32 = 48.0;
const ROW_GAP: f32 = 4.0;
const COL_GAP: f32 = 4.0;

const PART_DISPLAY_VAL: Val = Val::Px(128.0);
const ATTRIB_VAL: Val = Val::Px(ATTRIB_SIZE);
const LABEL_VAL: Val = Val::Px(160.0);

fn h_vals(bg: Srgba) -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: ATTRIB_VAL,
            grid_template_columns: RepeatedGridTrack::percent(4, 25.0),
            grid_template_rows: GridTrack::max_content(),
            column_gap: Val::Px(2.0),
            ..default()
        },
        background_color: bg.into(),
        ..default()
    }
}

fn v_vals(width: Val) -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Grid,
            width,
            height: HUNDRED,
            grid_template_columns: GridTrack::max_content(),
            grid_template_rows: RepeatedGridTrack::max_content(5),
            grid_row: GridPlacement::span(5),
            row_gap: Val::Px(ROW_GAP),
            ..default()
        },
        background_color: bevy::color::palettes::css::NAVY.into(),
        ..default()
    }
}

fn attrib_node(w: Val, color: Srgba) -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Grid,
            width: w,
            height: ATTRIB_VAL,
            padding: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        background_color: color.into(),
        ..default()
    }
}

fn spawn_with_text(parent: &mut ChildBuilder, node: NodeBundle, string: impl Into<String>, tooltip: Option<impl Into<String>>, font_info: &FontInfo) -> Entity {
    let mut id = Entity::PLACEHOLDER;
    let mut base_node = if let Some(tip) = tooltip {
        let tooltip_node = pyri_tooltip::Tooltip::cursor(TooltipContent::from(tip.into()));
        parent.spawn((node, tooltip_node, Interaction::default()))
    } else {
        parent.spawn(node)
    };
    base_node.with_children(|parent| {
        id = parent.spawn(text(string, font_info)).id();
    });
    id
}

fn spawn_labelled(parent: &mut ChildBuilder, header: impl Into<String>, font_info: &FontInfo) -> Entity {
    let v_label = attrib_node(LABEL_VAL, bevy::color::palettes::css::SILVER);

    let mut header_font_info = font_info.clone();
    header_font_info.size *= 0.6;

    let mut id = Entity::PLACEHOLDER;
    parent.spawn(v_label).with_children(|parent| {
        parent.spawn(text(header, &header_font_info));
        id = parent.spawn(text("-", font_info)).id();
    });
    id
}

fn spawn_info(parent: &mut ChildBuilder, header: impl Into<String>, info: InfoKind, font_info: &FontInfo) {
    let v_label = attrib_node(LABEL_VAL, bevy::color::palettes::css::DARK_GRAY);

    let mut header_font_info = font_info.clone();
    header_font_info.size *= 0.6;

    parent.spawn(v_label).with_children(|parent| {
        parent.spawn(text(header, &header_font_info));
        parent.spawn((text("-", font_info), info));
    });
}

fn attrib_header(parent: &mut ChildBuilder, font_info: &FontInfo) {
    let h_vals = h_vals(Srgba::NONE);
    let val = attrib_node(ATTRIB_VAL, bevy::color::palettes::css::DARK_GRAY);

    parent.spawn(h_vals.clone()).with_children(|parent| {
        spawn_with_text(parent, val.clone(), "a", Some("Accuracy"), font_info);
        spawn_with_text(parent, val.clone(), "b", Some("Boost"), font_info);
        spawn_with_text(parent, val.clone(), "c", Some("Celerity"), font_info);
        spawn_with_text(parent, val.clone(), "d", Some("Duration"), font_info);
    });
}

fn spawn_val_label(parent: &mut ChildBuilder, val_kind: PlayerPartHolderKind, font_info_val: &FontInfo, label_kind: PlayerPartHolderKind, font_info_label: &FontInfo, headers: [&str; 4]) {
    let mut val_children = Vec::with_capacity(4);
    parent
        .spawn((v_vals(ATTRIB_VAL), DropTarget, val_kind, PlayerPartHolder(None)))
        .with_children(|parent| {
            let val = attrib_node(ATTRIB_VAL, bevy::color::palettes::css::SILVER);
            parent.spawn(attrib_node(ATTRIB_VAL, Srgba::NONE));
            val_children.push(spawn_with_text(parent, val.clone(), "-", None::<&str>, font_info_val));
            val_children.push(spawn_with_text(parent, val.clone(), "-", None::<&str>, font_info_val));
            val_children.push(spawn_with_text(parent, val.clone(), "-", None::<&str>, font_info_val));
            val_children.push(spawn_with_text(parent, val.clone(), "-", None::<&str>, font_info_val));
        })
        .insert(TextChildren(val_children));

    let mut label_children = Vec::with_capacity(4);
    parent
        .spawn((v_vals(HUNDRED), DropTarget, label_kind, PlayerPartHolder(None)))
        .with_children(|parent| {
            parent.spawn(attrib_node(ATTRIB_VAL, Srgba::NONE));
            label_children.push(spawn_labelled(parent, headers[0], font_info_label));
            label_children.push(spawn_labelled(parent, headers[1], font_info_label));
            label_children.push(spawn_labelled(parent, headers[2], font_info_label));
            label_children.push(spawn_labelled(parent, headers[3], font_info_label));
        })
        .insert(TextChildren(label_children));
}

#[derive(Component)]
struct TextChildren(Vec<Entity>);

fn attrib_row(parent: &mut ChildBuilder, kind: StatRowKind, font_info: &FontInfo) {
    let h_vals = h_vals(Srgba::rgb(0.0, 0.5, 0.0));
    let val = attrib_node(ATTRIB_VAL, bevy::color::palettes::css::SILVER);

    let mut text_children = Vec::with_capacity(4);

    parent
        .spawn((h_vals.clone(), DropTarget, PlayerPartHolderKind::StatRow(kind), PlayerPartHolder(None)))
        .with_children(|parent| {
            text_children.push(spawn_with_text(parent, val.clone(), "-", None::<&str>, font_info));
            text_children.push(spawn_with_text(parent, val.clone(), "-", None::<&str>, font_info));
            text_children.push(spawn_with_text(parent, val.clone(), "-", None::<&str>, font_info));
            text_children.push(spawn_with_text(parent, val.clone(), "-", None::<&str>, font_info));
        })
        .insert(TextChildren(text_children));
}

fn spawn_part(parent: &mut ChildBuilder, part: &VagabondPart, font_info: &FontInfo) {
    let part_display = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: PART_DISPLAY_VAL,
            height: PART_DISPLAY_VAL,
            grid_template_columns: GridTrack::percent(100.0),
            grid_template_rows: vec![GridTrack::px(50.0), GridTrack::px(8.0), GridTrack::px(50.0), GridTrack::px(8.0), GridTrack::px(12.0)],
            ..default()
        },
        background_color: bevy::color::palettes::css::SILVER.into(),
        ..default()
    };

    let label_container = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: GridTrack::percent(100.0),
            grid_template_rows: RepeatedGridTrack::auto(4),
            ..default()
        },
        background_color: bevy::color::palettes::css::SILVER.into(),
        ..default()
    };

    let val_container = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: RepeatedGridTrack::percent(4, 25.0),
            grid_template_rows: GridTrack::max_content(),
            ..default()
        },
        background_color: bevy::color::palettes::css::SILVER.into(),
        ..default()
    };

    let mut text_children = Vec::with_capacity(12);
    parent
        .spawn((part_display, PlayerPartHolderKind::Unallocated, DragTarget))
        .with_children(|parent| {
            parent.spawn(label_container.clone()).with_children(|parent| {
                for build in &part.build {
                    text_children.push(parent.spawn(text(build.title.clone(), font_info)).id());
                }
            });

            parent.spawn(text("-", font_info));

            parent.spawn(label_container.clone()).with_children(|parent| {
                for detail in &part.detail {
                    text_children.push(parent.spawn(text(detail.title.clone(), font_info)).id());
                }
            });

            parent.spawn(text("-", font_info));
            parent.spawn(val_container.clone()).with_children(|parent| {
                for value in &part.values {
                    text_children.push(parent.spawn(text(value.to_string(), font_info)).id());
                }
            });
        })
        .insert(PlayerPartHolder(Some(part.clone())))
        .insert(TextChildren(text_children));
}

fn spawn_card_holder(parent: &mut ChildBuilder, idx: usize, font_info: &FontInfo) -> Entity {
    parent.spawn(text("-", font_info)).insert(CardHolder(idx)).id()
}

fn compose_enter(
    // bevy system
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    init_handoff: Res<ComposeInitHandoff>,
) {
    let parts = init_handoff.parts.clone();
    commands.remove_resource::<ComposeInitHandoff>();
    commands.insert_resource(ComposeState::default());
    commands.insert_resource(PlayerCache::default());
    build_ui_compose(commands, parts, asset_server);
}

fn build_ui_compose(mut commands: Commands, parts: [VagabondPart; 8], asset_server: Res<AssetServer>) {
    let font_info_val = font_size(&asset_server, 48.0);
    let font_info_label = font_size(&asset_server, 16.0);
    let font_info_part = font_size(&asset_server, 12.0);
    let font_info_card = font_size(&asset_server, 14.0);

    let main_layout = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: vec![GridTrack::max_content(), GridTrack::flex(1.0)],
            grid_template_rows: vec![GridTrack::px(132.0), GridTrack::px(32.0), GridTrack::flex(1.0)],
            ..default()
        },
        ..default()
    };
    let part_gutter = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: RepeatedGridTrack::px(8, 128.0),
            grid_template_rows: vec![GridTrack::px(128.0)],
            row_gap: Val::Px(ROW_GAP),
            column_gap: Val::Px(COL_GAP),
            padding: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        ..default()
    };
    let deck_gutter = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_row: GridPlacement::span(3),
            ..default()
        },
        background_color: bevy::color::palettes::css::DARK_GRAY.into(),
        ..default()
    };
    let spacer = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    };
    let compose_area = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: vec![GridTrack::max_content(), GridTrack::percent(5.0), GridTrack::max_content(), GridTrack::percent(5.0), GridTrack::max_content(), GridTrack::percent(5.0), GridTrack::max_content()],
            grid_template_rows: vec![GridTrack::auto(), GridTrack::px(32.0)],
            ..default()
        },
        background_color: bevy::color::palettes::css::NAVY.into(),
        ..default()
    };
    let compose_attributes = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: Val::Px((ATTRIB_SIZE + ROW_GAP) * 5.0),
            grid_template_columns: RepeatedGridTrack::min_content(2),
            grid_template_rows: RepeatedGridTrack::min_content(5),
            row_gap: Val::Px(ROW_GAP),
            column_gap: Val::Px(COL_GAP),
            ..default()
        },
        background_color: bevy::color::palettes::css::MIDNIGHT_BLUE.into(),
        ..default()
    };

    commands.spawn(ScreenBundle::default()).with_children(|parent| {
        parent.spawn(main_layout).with_children(|parent| {
            parent.spawn(part_gutter).with_children(|parent| {
                for part in &parts {
                    spawn_part(parent, part, &font_info_part);
                }
            });
            parent.spawn(deck_gutter).with_children(|parent| {
                for idx in 0..40 {
                    spawn_card_holder(parent, idx, &font_info_card);
                }
            });
            parent.spawn(spacer);
            parent.spawn(compose_area).with_children(|parent| {
                parent.spawn(compose_attributes.clone()).with_children(|parent| {
                    parent.spawn(v_vals(ATTRIB_VAL)).with_children(|parent| {
                        let val = attrib_node(ATTRIB_VAL, bevy::color::palettes::css::DARK_GRAY);
                        parent.spawn(attrib_node(ATTRIB_VAL, Srgba::NONE));
                        spawn_with_text(parent, val.clone(), "A", Some("Analyze"), &font_info_val);
                        spawn_with_text(parent, val.clone(), "B", Some("Breach"), &font_info_val);
                        spawn_with_text(parent, val.clone(), "C", Some("Compute"), &font_info_val);
                        spawn_with_text(parent, val.clone(), "D", Some("Disrupt"), &font_info_val);
                    });
                    parent.spawn(v_vals(HUNDRED)).with_children(|parent| {
                        attrib_header(parent, &font_info_val);
                        attrib_row(parent, StatRowKind::Analyze, &font_info_val);
                        attrib_row(parent, StatRowKind::Breach, &font_info_val);
                        attrib_row(parent, StatRowKind::Compute, &font_info_val);
                        attrib_row(parent, StatRowKind::Disrupt, &font_info_val);
                    });
                });
                parent.spawn(attrib_node(ATTRIB_VAL, Srgba::NONE));
                parent.spawn(compose_attributes.clone()).with_children(|parent| {
                    let headers = ["ANT", "BRD", "CPU", "DSK"];
                    spawn_val_label(parent, PlayerPartHolderKind::StatRow(Build), &font_info_val, PlayerPartHolderKind::Build, &font_info_label, headers);
                });
                parent.spawn(attrib_node(ATTRIB_VAL, Srgba::NONE));
                parent.spawn(compose_attributes.clone()).with_children(|parent| {
                    let headers = ["Institution", "Role", "Location", "Distro"];
                    spawn_val_label(parent, PlayerPartHolderKind::StatRow(Detail), &font_info_val, PlayerPartHolderKind::Detail, &font_info_label, headers);
                });
                parent.spawn(attrib_node(ATTRIB_VAL, Srgba::NONE));
                parent.spawn(compose_attributes).with_children(|parent| {
                    parent.spawn(v_vals(HUNDRED)).with_children(|parent| {
                        parent.spawn(attrib_node(ATTRIB_VAL, Srgba::NONE));
                        spawn_info(parent, "ID", InfoKind::ID, &font_info_label);
                        spawn_info(parent, "Name", InfoKind::Name, &font_info_label);
                        spawn_info(parent, "Birthplace", InfoKind::Birthplace, &font_info_label);
                        spawn_info(parent, "Age", InfoKind::DoB, &font_info_label);
                    });
                });
                parent
                    .spawn((
                        SubmitButton,
                        ButtonBundle {
                            background_color: bevy::color::palettes::css::DARK_GRAY.into(),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(text("Submit", &font_info_label));
                    });
            });
        });
    });
}

type ButtonQuery<'a> = (&'a Interaction, &'a mut BackgroundColor, &'a mut BorderColor);

fn button_update(
    // bevy system
    mut interaction_query: Query<ButtonQuery, (Changed<Interaction>, With<Button>)>,
    state: Res<ComposeState>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = if *state == ComposeState::Ready {
                    bevy::color::palettes::basic::GREEN
                } else {
                    bevy::color::palettes::basic::RED
                }
                .into();
                *border_color = bevy::color::palettes::basic::RED.into();
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0.25, 0.25, 0.25).into();
                *border_color = Color::WHITE.into();
            }
            Interaction::None => {
                *background_color = Color::srgb(0.15, 0.15, 0.15).into();
                *border_color = Color::BLACK.into();
            }
        }
    }
}

fn button_commit_update(
    // bevy system
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<SubmitButton>)>,
    mut state: ResMut<ComposeState>,
    mut send: EventWriter<FinishPlayer>,
) {
    if *state != ComposeState::Ready {
        return;
    }
    for &interaction in &interaction_query {
        if interaction == Interaction::Pressed {
            *state = ComposeState::Committed;
            send.send(FinishPlayer);
        }
    }
}

fn drag_drag(
    // bevy system
    mut commands: Commands,
    mut receive: EventReader<DragDrag>,
    holder_q: Query<(&GlobalTransform, &PlayerPartHolder)>,
    asset_server: Res<AssetServer>,
) {
    for dragdrag in receive.read() {
        let (gt, holder) = holder_q.get(dragdrag.src).expect("");
        let transform = gt.compute_transform().translation.truncate();
        let font_info = font_size(&asset_server, 12.0);

        if let Some(part) = &holder.0 {
            commands.spawn((filled_rect(Val::Px(transform.x - 66.0), Val::Px(transform.y - 66.0), Val::Px(132.0), Val::Px(132.0), bevy::color::palettes::css::CHARTREUSE), Dragging(dragdrag.src))).with_children(|parent| {
                spawn_part(parent, part, &font_info);
            });
        }
    }
}

fn populate_children<F>(kids: &TextChildren, holder: &PlayerPartHolder, text_q: &mut Query<&mut Text>, func: F)
where
    F: Fn(&VagabondPart, usize) -> String,
{
    let holder = holder.0.as_ref();
    for (i, kid) in kids.0.iter().enumerate() {
        if let Ok(mut text) = text_q.get_mut(*kid) {
            let val = holder.map_or("-".to_owned(), |o| func(o, i));
            text.sections[0].value = val;
        }
    }
}

fn update_part_holder(kind: &PlayerPartHolderKind, kids: Option<&TextChildren>, holder: &PlayerPartHolder, text_q: &mut Query<&mut Text>) {
    if let Some(kids) = kids {
        let func = match kind {
            PlayerPartHolderKind::StatRow(_) => |o: &VagabondPart, i: usize| o.values[i].to_string(),
            PlayerPartHolderKind::Build => |o: &VagabondPart, i: usize| o.build[i].title.clone(),
            PlayerPartHolderKind::Detail => |o: &VagabondPart, i: usize| o.detail[i].title.clone(),
            PlayerPartHolderKind::Unallocated => |_: &VagabondPart, _: usize| "".to_owned(),
        };
        populate_children(kids, holder, text_q, func);
    }
}

fn drag_drop(
    // bevy system
    mut commands: Commands,
    mut receive: EventReader<DragDrop>,
    holder_q: Query<(Option<&TextChildren>, &PlayerPartHolder, &PlayerPartHolderKind)>,
    mut text_q: Query<&mut Text>,
    mut send: EventWriter<FinishPlayer>,
    state: Res<ComposeState>,
) {
    if *state == ComposeState::Committed {
        receive.clear();
        return;
    }

    for drag_drop in receive.read() {
        let src_entity = drag_drop.src;
        if let Some(dst_entity) = drag_drop.dst {
            let (src_kids, src_part, src_kind) = holder_q.get(src_entity).unwrap();
            let (dst_kids, dst_part, dst_kind) = holder_q.get(dst_entity).unwrap();
            if dst_part.0.is_none() {
                commands.entity(src_entity).remove::<DragTarget>();
                commands.entity(dst_entity).insert(DragTarget);
            }
            commands.entity(src_entity).remove::<PlayerPartHolder>().insert(dst_part.clone());
            commands.entity(dst_entity).remove::<PlayerPartHolder>().insert(src_part.clone());

            update_part_holder(src_kind, src_kids, dst_part, &mut text_q);
            update_part_holder(dst_kind, dst_kids, src_part, &mut text_q);

            send.send(FinishPlayer);
        }

        commands.entity(drag_drop.drag).despawn_recursive();
    }
}

fn seed_from_holder(holder: &PlayerPartHolder) -> u64 {
    holder.0.as_ref().map(|o| o.seed).unwrap_or_default()
}

fn values_from_holder(holder: &PlayerPartHolder) -> [AttributeValueType;4] {
    holder.0.as_ref().map(|o| o.values).unwrap_or_default()
}

fn finish_player(
    // bevy system
    receive: EventReader<FinishPlayer>,
    holder_q: Query<(&PlayerPartHolder, &PlayerPartHolderKind)>,
    gate: Res<GateIFace>,
    mut state: ResMut<ComposeState>,
    mut player_cache: ResMut<PlayerCache>,
) {
    if !receive.is_empty() {
        let mut parts = [0, 0, 0, 0, 0, 0, 0, 0];

        for (holder, holder_kind) in holder_q.iter() {
            match holder_kind {
                PlayerPartHolderKind::StatRow(row) => match row {
                    StatRowKind::Analyze => {
                        parts[0] = seed_from_holder(holder);
                        player_cache.attr[0] = values_from_holder(holder);
                    },
                    StatRowKind::Breach => {
                        parts[1] = seed_from_holder(holder);
                        player_cache.attr[1] = values_from_holder(holder);
                    },
                    StatRowKind::Compute => {
                        parts[2] = seed_from_holder(holder);
                        player_cache.attr[2] = values_from_holder(holder);
                    },
                    StatRowKind::Disrupt => {
                        parts[3] = seed_from_holder(holder);
                        player_cache.attr[3] = values_from_holder(holder);
                    },
                    Build => parts[5] = seed_from_holder(holder),
                    Detail => parts[7] = seed_from_holder(holder),
                },
                PlayerPartHolderKind::Build => parts[4] = seed_from_holder(holder),
                PlayerPartHolderKind::Detail => parts[6] = seed_from_holder(holder),
                PlayerPartHolderKind::Unallocated => {}
            }
        }

        if parts.iter().all(|&o| o != 0) {
            if *state == ComposeState::Build {
                *state = ComposeState::Ready;
            }
            gate.send_game_build(parts, *state == ComposeState::Committed);
        }
    }
}

fn compose_update(
    // bevy system
    mut gate: ResMut<GateIFace>,
    mut deck_q: Query<(&mut Text, &CardHolder), Without<InfoKind>>,
    mut info_q: Query<(&mut Text, &InfoKind), Without<CardHolder>>,
    wm: Res<WarehouseManager>,
    dm: Res<DataManager>,
    mut player_cache: ResMut<PlayerCache>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    match gate.grx.try_recv() {
        Ok(GateCommand::GameBuild(gate_response)) => {
            match wm.fetch_player(gate_response.seed) {
                Ok(warehouse_response) => {
                    if let Some(player_bio) = warehouse_response.player_bio {
                        for (mut info, info_kind) in info_q.iter_mut() {
                            match info_kind {
                                InfoKind::Name => info.sections[0].value.clone_from(&player_bio.name),
                                InfoKind::ID => info.sections[0].value.clone_from(&player_bio.id),
                                InfoKind::Birthplace => info.sections[0].value = player_bio.birthplace(),
                                InfoKind::DoB => info.sections[0].value = player_bio.age().to_string(),
                            }
                        }

                        let deck = dm.convert_deck(gate_response.deck);

                        for (idx, card) in deck.iter().enumerate() {
                            if let Some((mut card_text, _)) = deck_q.iter_mut().find(|o| o.1 .0 == idx) {
                                card_text.sections[0].value.clone_from(&card.title);
                            }
                        }
                        player_cache.bio = player_bio;
                    }
                }
                Err(err) => println!("Error: {err}"),
            }
        }
        Ok(GateCommand::GameStartGame(gate_response)) => {
            if gate_response.success {
                app_state.set(AppState::Gameplay);
            }
        }
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn compose_exit(
    // bevy system
    mut commands: Commands,
    screen_q: Query<Entity, With<Screen>>,
) {
    commands.remove_resource::<ComposeState>();
    screen_exit(commands, screen_q);
}
