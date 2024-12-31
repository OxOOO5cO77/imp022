use bevy::prelude::*;
use bevy_simple_text_input::{TextInputCursorPos, TextInputInactive, TextInputSubmitEvent, TextInputValue};
use tokio::sync::mpsc;

use shared_net::AuthType;

use crate::manager::{AtlasManager, NetworkManager, ScreenLayoutManager, ScreenLayoutManagerParams};
use crate::network::client_drawbridge;
use crate::network::client_drawbridge::{AuthInfo, DrawbridgeClient, DrawbridgeIFace};
use crate::screen::shared::AppScreenExt;
use crate::system::AppState;

const SCREEN_LAYOUT: &str = "login";

pub struct LoginDrawbridgePlugin;

impl Plugin for LoginDrawbridgePlugin {
    //noinspection Duplicates
    fn build(&self, app: &mut App) {
        app //
            .add_screen(AppState::LoginDrawbridge)
            .with_enter((drawbridge_enter, login_ui_setup).chain())
            .with_update((drawbridge_update, textedit_update))
            .with_exit(drawbridge_exit);
    }
}

#[derive(Component)]
struct ConnectedIcon;

#[derive(Resource)]
struct LoginContext {
    username: Entity,
    password: Entity,
}

#[derive(Resource)]
pub(crate) struct DrawbridgeHandoff {
    pub(crate) iface: String,
    pub(crate) auth: AuthType,
}

impl DrawbridgeHandoff {
    fn new(auth_info: AuthInfo) -> Self {
        Self {
            iface: format!("{}:{}", auth_info.ip, auth_info.port),
            auth: auth_info.auth,
        }
    }
}

fn drawbridge_enter(
    // bevy system
    mut commands: Commands,
    mut net: ResMut<NetworkManager>,
) {
    let (to_drawbridge_tx, to_drawbridge_rx) = mpsc::unbounded_channel();
    let (from_drawbridge_tx, from_drawbridge_rx) = mpsc::unbounded_channel();

    let username = std::env::var("VAGABOND_USERNAME").unwrap_or("".to_string());
    let password = std::env::var("VAGABOND_PASSWORD").unwrap_or("".to_string());

    let drawbridge = DrawbridgeIFace {
        username,
        password,
        dtx: to_drawbridge_tx,
        drx: from_drawbridge_rx,
    };

    commands.insert_resource(drawbridge);

    if let Some(task) = &net.current_task {
        task.abort();
    }
    net.current_task = DrawbridgeClient::start("[::1]:23450".to_string(), from_drawbridge_tx, to_drawbridge_rx, &net.runtime);
}

trait TextInputExt {
    fn with_text(self, text: &str, active: bool) -> Self;
}

impl TextInputExt for &mut EntityCommands<'_> {
    fn with_text(self, text: &str, active: bool) -> Self {
        self //
            .insert((TextInputInactive(!active), TextInputCursorPos(text.len()), TextInputValue(text.into())))
    }
}

fn login_ui_setup(
    // bevy system
    mut commands: Commands,
    drawbridge: Res<DrawbridgeIFace>,
    am: Res<AtlasManager>,
    mut slm: ResMut<ScreenLayoutManager>,
    mut slm_params: ScreenLayoutManagerParams,
) {
    let layout = slm.build(&mut commands, SCREEN_LAYOUT, &am, &mut slm_params, None);

    commands.entity(layout.entity("connected_icon")).insert(ConnectedIcon);

    let username = commands.entity(layout.entity("username")).with_text(&drawbridge.username, true).id();
    let password = commands.entity(layout.entity("password")).with_text(&drawbridge.password, false).id();

    let context = LoginContext {
        username,
        password,
    };
    commands.insert_resource(context)
}

fn textedit_update(
    // bevy system
    mut commands: Commands,
    mut events: EventReader<TextInputSubmitEvent>,
    tracker: Res<LoginContext>,
    mut drawbridge: ResMut<DrawbridgeIFace>,
) {
    for event in events.read() {
        commands.entity(tracker.username).insert(TextInputInactive(true));
        commands.entity(tracker.password).insert(TextInputInactive(false));

        if event.entity == tracker.username {
            drawbridge.username = event.value.clone();
        } else if event.entity == tracker.password {
            drawbridge.password = event.value.clone();

            if !drawbridge.username.is_empty() && !drawbridge.password.is_empty() {
                client_drawbridge::send_authorize(&drawbridge.dtx, drawbridge.username.clone(), drawbridge.password.clone());
            }
        }
    }
}

fn drawbridge_update(
    // bevy system
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut drawbridge: ResMut<DrawbridgeIFace>,
    mut sprite_q: Query<&mut Sprite, With<ConnectedIcon>>,
) {
    if let Ok(auth_info) = drawbridge.drx.try_recv() {
        if let Ok(mut sprite) = sprite_q.get_single_mut() {
            sprite.color = bevy::color::palettes::css::YELLOW.into()
        }
        commands.insert_resource(DrawbridgeHandoff::new(auth_info));
        app_state.set(AppState::LoginGate);
    }
}

fn drawbridge_exit(
    // bevy system
    mut commands: Commands,
) {
    commands.remove_resource::<LoginContext>();
    commands.remove_resource::<DrawbridgeIFace>();
}
