use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};

mod atlas_manager;
mod data_manager;
mod network_manager;
mod screen_layout_manager;
mod warehouse_manager;

pub(crate) use crate::manager::atlas_manager::AtlasManager;
use crate::manager::atlas_manager::AtlasManagerPlugin;
pub(crate) use crate::manager::data_manager::DataManager;
pub(crate) use crate::manager::network_manager::NetworkManager;
pub(crate) use crate::manager::screen_layout_manager::{ScreenLayout, ScreenLayoutManager};
pub(crate) use crate::manager::warehouse_manager::WarehouseManager;

pub(super) struct ManagerPlugins;

impl PluginGroup for ManagerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>() //
            .add(ManagerPlugin)
            .add(AtlasManagerPlugin)
    }
}

struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkManager::new());
        app.insert_resource(WarehouseManager::new("http://127.0.0.1:23235"));
        app.insert_resource(DataManager::new().expect("[Vagabond] Failed to initialize DataManager"));
        app.insert_resource(ScreenLayoutManager::new("assets/screen").expect("[Vagabond] Failed to initialize ScreenLayoutManager"));
    }
}
