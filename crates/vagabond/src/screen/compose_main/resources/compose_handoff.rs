use bevy::prelude::Resource;

#[derive(Resource)]
pub(crate) struct ComposeHandoff {
    pub(crate) local_name: String,
    pub(crate) local_id: String,
}
