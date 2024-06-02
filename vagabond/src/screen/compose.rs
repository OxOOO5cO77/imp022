use bevy::prelude::*;
use shared_data::player::part::PlayerPart;
use rand::RngCore;
use rand::rngs::ThreadRng;

use crate::manager::BackendManager;
use crate::screen::compose::StatRowKind::{Build, Category};
use crate::system::app_state::AppState;
use crate::system::dragdrop::{DragDrag, DragDrop, Dragging, DragTarget, DropTarget};
use crate::system::ui::{filled_rect, HUNDRED, screen_exit, ScreenBundle};

pub struct ComposePlugin;

impl Plugin for ComposePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FinishPlayer>()
            .add_systems(OnEnter(AppState::Compose), compose_enter)
            .add_systems(Update, (dragdrag, dragdrop, finish_player).run_if(in_state(AppState::Compose)))
            .add_systems(OnExit(AppState::Compose), screen_exit)
        ;
    }
}

#[derive(Event)]
struct FinishPlayer;

enum StatRowKind {
    Access,
    Breach,
    Compute,
    Disrupt,
    Build,
    Category,
}

#[derive(Component)]
enum PlayerPartHolderKind {
    StatRow(StatRowKind),
    Build,
    Category,
    Unallocated,
}

#[derive(Component, Clone)]
struct PlayerPartHolder(Option<PlayerPart>);

#[derive(Component, Clone)]
struct CardHolder(usize);


#[derive(Component)]
enum InfoKind {
    Name,
    ID,
    Birthplace,
    DoB,
}

const ATTRIB_SIZE: f32 = 48.0;
const ROW_GAP: f32 = 4.0;
const COL_GAP: f32 = 4.0;

const PART_DISPLAY_VAL: Val = Val::Px(128.0);
const ATTRIB_VAL: Val = Val::Px(ATTRIB_SIZE);
const LABEL_VAL: Val = Val::Px(160.0);

fn h_vals(bg: Color) -> NodeBundle {
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
        background_color: Color::NAVY.into(),
        ..default()
    }
}

fn node(w: Val, color: Color) -> NodeBundle {
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

#[derive(Clone)]
struct FontInfo {
    handle: Handle<Font>,
    size: f32,
    color: Color,
}

fn text(text: impl Into<String>, font_info: &FontInfo) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: font_info.handle.clone(),
            font_size: font_info.size,
            color: font_info.color,
        },
    ).with_style(Style {
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        ..default()
    })
}

fn spawn_with_text(parent: &mut ChildBuilder, node: NodeBundle, string: impl Into<String>, font_info: &FontInfo) -> Entity {
    let mut id = Entity::PLACEHOLDER;
    parent.spawn(node).with_children(|parent| { id = parent.spawn(text(string, font_info)).id(); });
    id
}

fn spawn_labelled(parent: &mut ChildBuilder, header: impl Into<String>, font_info: &FontInfo) -> Entity {
    let v_label = node(LABEL_VAL, Color::SILVER);

    let mut header_font_info = font_info.clone();
    header_font_info.size *= 0.6;

    let mut id = Entity::PLACEHOLDER;
    parent
        .spawn(v_label)
        .with_children(|parent| {
            parent.spawn(text(header, &header_font_info));
            id = parent.spawn(text("-", font_info)).id();
        })
    ;
    id
}

fn spawn_info(parent: &mut ChildBuilder, header: impl Into<String>, info: InfoKind, font_info: &FontInfo) {
    let v_label = node(LABEL_VAL, Color::DARK_GRAY);

    let mut header_font_info = font_info.clone();
    header_font_info.size *= 0.6;

    parent
        .spawn(v_label)
        .with_children(|parent| {
            parent.spawn(text(header, &header_font_info));
            parent.spawn((text("-", font_info), info));
        })
    ;
}

fn attrib_header(parent: &mut ChildBuilder, font_info: &FontInfo) {
    let h_vals = h_vals(Color::NONE);
    let val = node(ATTRIB_VAL, Color::DARK_GRAY);

    parent
        .spawn(h_vals.clone())
        .with_children(|parent| {
            spawn_with_text(parent, val.clone(), "a", font_info);
            spawn_with_text(parent, val.clone(), "b", font_info);
            spawn_with_text(parent, val.clone(), "c", font_info);
            spawn_with_text(parent, val.clone(), "d", font_info);
        })
    ;
}

fn spawn_val_label(parent: &mut ChildBuilder, val_kind: PlayerPartHolderKind, font_info_val: &FontInfo, label_kind: PlayerPartHolderKind, font_info_label: &FontInfo, headers: [&str; 4]) {
    let mut val_children = Vec::with_capacity(4);
    parent
        .spawn((v_vals(ATTRIB_VAL), DropTarget, val_kind, PlayerPartHolder(None)))
        .with_children(|parent| {
            let val = node(ATTRIB_VAL, Color::SILVER);
            parent.spawn(node(ATTRIB_VAL, Color::NONE));
            val_children.push(spawn_with_text(parent, val.clone(), "-", font_info_val));
            val_children.push(spawn_with_text(parent, val.clone(), "-", font_info_val));
            val_children.push(spawn_with_text(parent, val.clone(), "-", font_info_val));
            val_children.push(spawn_with_text(parent, val.clone(), "-", font_info_val));
        })
        .insert(TextChildren(val_children))
    ;

    let mut label_children = Vec::with_capacity(4);
    parent
        .spawn((v_vals(HUNDRED), DropTarget, label_kind, PlayerPartHolder(None)))
        .with_children(|parent| {
            parent.spawn(node(ATTRIB_VAL, Color::NONE));
            label_children.push(spawn_labelled(parent, headers[0], font_info_label));
            label_children.push(spawn_labelled(parent, headers[1], font_info_label));
            label_children.push(spawn_labelled(parent, headers[2], font_info_label));
            label_children.push(spawn_labelled(parent, headers[3], font_info_label));
        })
        .insert(TextChildren(label_children))
    ;
}

#[derive(Component)]
struct TextChildren(Vec<Entity>);

fn attrib_row(parent: &mut ChildBuilder, kind: StatRowKind, font_info: &FontInfo) {
    let h_vals = h_vals(Color::DARK_GREEN);
    let val = node(ATTRIB_VAL, Color::SILVER);

    let mut text_children = Vec::with_capacity(4);

    parent
        .spawn((h_vals.clone(), DropTarget, PlayerPartHolderKind::StatRow(kind), PlayerPartHolder(None)))
        .with_children(|parent| {
            text_children.push(spawn_with_text(parent, val.clone(), "-", font_info));
            text_children.push(spawn_with_text(parent, val.clone(), "-", font_info));
            text_children.push(spawn_with_text(parent, val.clone(), "-", font_info));
            text_children.push(spawn_with_text(parent, val.clone(), "-", font_info));
        })
        .insert(TextChildren(text_children))
    ;
}

fn spawn_part(parent: &mut ChildBuilder, part: &PlayerPart, font_info: &FontInfo) {
    let part_display = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: PART_DISPLAY_VAL,
            height: PART_DISPLAY_VAL,
            grid_template_columns: GridTrack::percent(100.0),
            grid_template_rows: vec![GridTrack::min_content(), GridTrack::min_content(), GridTrack::min_content(), GridTrack::flex(1.0), GridTrack::min_content()],
            ..default()
        },
        background_color: Color::SILVER.into(),
        ..default()
    };

    let label_container = NodeBundle {
        style: Style {
            display: Display::Grid,
            width: HUNDRED,
            height: HUNDRED,
            grid_template_columns: GridTrack::percent(100.0),
            grid_template_rows: RepeatedGridTrack::max_content(4),
            ..default()
        },
        background_color: Color::SILVER.into(),
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
        background_color: Color::SILVER.into(),
        ..default()
    };

    let mut text_children = Vec::with_capacity(12);
    parent
        .spawn((part_display, PlayerPartHolderKind::Unallocated, DragTarget))
        .with_children(|parent| {
            parent
                .spawn(label_container.clone())
                .with_children(|parent| {
                    for build in &part.build {
                        text_children.push(parent.spawn(text(build.1.clone(), font_info)).id());
                    }
                })
            ;

            parent.spawn(text("-", font_info));

            parent
                .spawn(label_container.clone())
                .with_children(|parent| {
                    for category in &part.category {
                        text_children.push(parent.spawn(text(category.1.clone(), font_info)).id());
                    }
                })
            ;

            parent.spawn(text("-", font_info));
            parent
                .spawn(val_container.clone())
                .with_children(|parent| {
                    for value in &part.values {
                        text_children.push(parent.spawn(text(value.to_string(), font_info)).id());
                    }
                })
            ;
        })
        .insert(PlayerPartHolder(Some(part.clone())))
        .insert(TextChildren(text_children))
    ;
}

fn spawn_card_holder(parent: &mut ChildBuilder, idx: usize, font_info: &FontInfo) -> Entity {
    parent
        .spawn(text("-", font_info))
        .insert(CardHolder(idx))
        .id()
}

fn font_size(asset_server: &Res<AssetServer>, size: f32) -> FontInfo {
    let font = asset_server.load("font/RobotoMono.ttf");
    FontInfo {
        handle: font.clone(),
        size,
        color: Color::BLACK,
    }
}

fn compose_enter(
    mut commands: Commands,
    bm: Res<BackendManager>,
    asset_server: Res<AssetServer>,
) {
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
        background_color: Color::DARK_GRAY.into(),
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
            grid_template_rows: GridTrack::auto(),
            ..default()
        },
        background_color: Color::NAVY.into(),
        ..default()
    };
    let compose_attribs = NodeBundle {
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
        background_color: Color::MIDNIGHT_BLUE.into(),
        ..default()
    };

    let mut rng = ThreadRng::default();

    commands
        .spawn(ScreenBundle::default())
        .with_children(|parent| {
            parent
                .spawn(main_layout)
                .with_children(|parent| {
                    parent
                        .spawn(part_gutter)
                        .with_children(|parent| {
                            let mut seeds = [0, 0, 0, 0, 0, 0, 0, 0];
                            for seed in seeds.iter_mut() {
                                *seed = rng.next_u64();
                            }
                            match bm.fetch_parts(seeds) {
                                Ok(parts) => {
                                    for part in &parts.parts {
                                        spawn_part(parent, part, &font_info_part);
                                    }
                                }
                                Err(err) => {
                                    println!("Error: {err}");
                                }
                            }
                        })
                    ;
                    parent
                        .spawn(deck_gutter)
                        .with_children(|parent| {
                            for idx in 0..40 {
                                spawn_card_holder(parent, idx, &font_info_card);
                            }
                        })
                    ;
                    parent
                        .spawn(spacer)
                    ;
                    parent
                        .spawn(compose_area)
                        .with_children(|parent| {
                            parent
                                .spawn(compose_attribs.clone())
                                .with_children(|parent| {
                                    parent
                                        .spawn(v_vals(ATTRIB_VAL))
                                        .with_children(|parent| {
                                            let val = node(ATTRIB_VAL, Color::DARK_GRAY);
                                            parent.spawn(node(ATTRIB_VAL, Color::NONE));
                                            spawn_with_text(parent, val.clone(), "A", &font_info_val);
                                            spawn_with_text(parent, val.clone(), "B", &font_info_val);
                                            spawn_with_text(parent, val.clone(), "C", &font_info_val);
                                            spawn_with_text(parent, val.clone(), "D", &font_info_val);
                                        })
                                    ;
                                    parent
                                        .spawn(v_vals(HUNDRED))
                                        .with_children(|parent| {
                                            attrib_header(parent, &font_info_val);
                                            attrib_row(parent, StatRowKind::Access, &font_info_val);
                                            attrib_row(parent, StatRowKind::Breach, &font_info_val);
                                            attrib_row(parent, StatRowKind::Compute, &font_info_val);
                                            attrib_row(parent, StatRowKind::Disrupt, &font_info_val);
                                        })
                                    ;
                                })
                            ;
                            parent.spawn(node(ATTRIB_VAL, Color::NONE));
                            parent
                                .spawn(compose_attribs.clone())
                                .with_children(|parent| {
                                    let headers = ["ANT", "BRD", "CPU", "DSC"];
                                    spawn_val_label(parent, PlayerPartHolderKind::StatRow(Build), &font_info_val, PlayerPartHolderKind::Build, &font_info_label, headers);
                                })
                            ;
                            parent.spawn(node(ATTRIB_VAL, Color::NONE));
                            parent
                                .spawn(compose_attribs.clone())
                                .with_children(|parent| {
                                    let headers = ["Institution", "Role", "Location", "Distro"];
                                    spawn_val_label(parent, PlayerPartHolderKind::StatRow(Category), &font_info_val, PlayerPartHolderKind::Category, &font_info_label, headers);
                                })
                            ;
                            parent.spawn(node(ATTRIB_VAL, Color::NONE));
                            parent
                                .spawn(compose_attribs)
                                .with_children(|parent| {
                                    parent
                                        .spawn(v_vals(HUNDRED))
                                        .with_children(|parent| {
                                            parent.spawn(node(ATTRIB_VAL, Color::NONE));
                                            spawn_info(parent, "ID", InfoKind::ID, &font_info_label);
                                            spawn_info(parent, "Name", InfoKind::Name, &font_info_label);
                                            spawn_info(parent, "Birthplace", InfoKind::Birthplace, &font_info_label);
                                            spawn_info(parent, "Age", InfoKind::DoB, &font_info_label);
                                        })
                                    ;
                                })
                            ;
                        })
                    ;
                })
            ;
        })
    ;
}

fn dragdrag(
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
            commands
                .spawn((
                    filled_rect(Val::Px(transform.x - 64.0), Val::Px(transform.y - 64.0), Val::Px(128.0), Val::Px(128.0), Color::ALICE_BLUE),
                    Dragging(dragdrag.src)
                ))
                .with_children(|parent| {
                    spawn_part(parent, part, &font_info);
                })
            ;
        }
    }
}

fn populate_children<F>(kids: &TextChildren, holder: &PlayerPartHolder, text_q: &mut Query<&mut Text>, func: F)
    where F: Fn(&PlayerPart, usize) -> String
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
            PlayerPartHolderKind::StatRow(_) => |o: &PlayerPart, i: usize| o.values[i].to_string(),
            PlayerPartHolderKind::Build => |o: &PlayerPart, i: usize| o.build[i].1.clone(),
            PlayerPartHolderKind::Category => |o: &PlayerPart, i: usize| o.category[i].1.clone(),
            PlayerPartHolderKind::Unallocated => |_: &PlayerPart, _: usize| "".to_owned(),
        };
        populate_children(kids, holder, text_q, func);
    }
}

fn dragdrop(
    mut commands: Commands,
    mut receive: EventReader<DragDrop>,
    holder_q: Query<(Option<&TextChildren>, &PlayerPartHolder, &PlayerPartHolderKind)>,
    mut text_q: Query<&mut Text>,
    mut send: EventWriter<FinishPlayer>,
) {
    for dragdrop in receive.read() {
        let src_entity = dragdrop.src;
        if let Some(dst_entity) = dragdrop.dst {
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

        commands.entity(dragdrop.drag).despawn_recursive();
    }
}

fn seed_from_holder(holder: &PlayerPartHolder) -> u64 {
    holder.0.as_ref().map(|o| o.seed).unwrap_or_default()
}

fn finish_player(
    receive: EventReader<FinishPlayer>,
    holder_q: Query<(&PlayerPartHolder, &PlayerPartHolderKind)>,
    mut deck_q: Query<(&mut Text, &CardHolder), Without<InfoKind>>,
    mut info_q: Query<(&mut Text, &InfoKind), Without<CardHolder>>,
    bm: Res<BackendManager>,
) {
    if !receive.is_empty() {
        let mut seeds = [0, 0, 0, 0, 0, 0, 0, 0, ];

        for (holder, holder_kind) in holder_q.iter() {
            match holder_kind {
                PlayerPartHolderKind::StatRow(row) => {
                    match row {
                        StatRowKind::Access => seeds[0] = seed_from_holder(holder),
                        StatRowKind::Breach => seeds[1] = seed_from_holder(holder),
                        StatRowKind::Compute => seeds[2] = seed_from_holder(holder),
                        StatRowKind::Disrupt => seeds[3] = seed_from_holder(holder),
                        Build => seeds[5] = seed_from_holder(holder),
                        Category => seeds[7] = seed_from_holder(holder),
                    }
                }
                PlayerPartHolderKind::Build => seeds[4] = seed_from_holder(holder),
                PlayerPartHolderKind::Category => seeds[6] = seed_from_holder(holder),
                PlayerPartHolderKind::Unallocated => {}
            }
        }

        if seeds.iter().all(|&o| o != 0) {
            match bm.fetch_player(seeds) {
                Ok(response) => {
                    if let Some(player) = response.player {
                        for (mut info, info_kind) in info_q.iter_mut() {
                            match info_kind {
                                InfoKind::Name => info.sections[0].value.clone_from(&player.name),
                                InfoKind::ID => info.sections[0].value.clone_from(&player.id),
                                InfoKind::Birthplace => info.sections[0].value = player.birthplace(),
                                InfoKind::DoB => info.sections[0].value = player.age().to_string(),
                            }
                        }

                        for (idx, card) in player.deck.iter().enumerate() {
                            if let Some((mut card_text, _)) = deck_q.iter_mut().find(|o| o.1.0 == idx) {
                                card_text.sections[0].value.clone_from(&card.name);
                            }
                        }

                    }
                }
                Err(err) => println!("Error: {err}"),
            }
        }
    }
}
