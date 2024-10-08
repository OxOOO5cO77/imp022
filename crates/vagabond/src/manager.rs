use bevy::app::{App, Plugin};

pub(crate) use crate::manager::backend_manager::BackendManager;
pub(crate) use crate::manager::data_manager::DataManager;
pub(crate) use crate::manager::network_manager::NetworkManager;

mod backend_manager;
mod network_manager;
mod data_manager;

pub(crate) struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkManager::new());
        app.insert_resource(BackendManager::new());
        app.insert_resource(DataManager::new().expect("[Vagabond] Failed to initialize DataManager"));
    }
}
