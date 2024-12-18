use crate::system::ui_effects::UiEffectsPlugins;
use bevy::app::{PluginGroup, PluginGroupBuilder};

mod app_state;
pub(crate) mod ui_effects;

pub(crate) use app_state::AppState;

pub(super) struct SystemPlugins;

impl PluginGroup for SystemPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>() //
            .add_group(UiEffectsPlugins)
    }
}
