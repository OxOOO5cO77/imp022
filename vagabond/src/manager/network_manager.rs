use bevy::prelude::Resource;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

#[derive(Resource)]
pub(crate) struct NetworkManager {
    pub(crate) runtime: Runtime,
    pub(crate) current_task: Option<JoinHandle<Result<(), ()>>>,
}

impl NetworkManager {
    pub(crate) fn new() -> Self {
        Self {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not build tokio runtime"),
            current_task: None,
        }
    }
}
