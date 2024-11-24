use bevy::app::{App, Plugin};

mod atlas_manager;
mod data_manager;
mod network_manager;
mod warehouse_manager;
mod screen_layout_manager;

pub(crate) use crate::manager::atlas_manager::AtlasManager;
pub(crate) use crate::manager::data_manager::DataManager;
pub(crate) use crate::manager::network_manager::NetworkManager;
pub(crate) use crate::manager::screen_layout_manager::ScreenLayoutManager;
pub(crate) use crate::manager::warehouse_manager::WarehouseManager;

pub(crate) struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkManager::new());
        app.insert_resource(WarehouseManager::new("http://127.0.0.1:23235"));
        app.insert_resource(DataManager::new().expect("[Vagabond] Failed to initialize DataManager"));
        app.insert_resource(AtlasManager::default());
        app.insert_resource(ScreenLayoutManager::new("assets/screen").expect("[Vagabond] Failed to initialize ScreenLayoutManager"));
    }
}
