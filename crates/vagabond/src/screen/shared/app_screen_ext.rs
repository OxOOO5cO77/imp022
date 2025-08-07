use crate::system::AppState;
use bevy::app::{App, PostUpdate, Update};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{in_state, IntoScheduleConfigs, OnEnter, OnExit};

pub(crate) trait AppScreenExt {
    fn add_screen(&mut self, app_state: AppState) -> ScreenBuilder<'_>;
}

impl AppScreenExt for App {
    fn add_screen(&mut self, app_state: AppState) -> ScreenBuilder<'_> {
        ScreenBuilder {
            app: self,
            app_state,
        }
    }
}

pub(crate) struct ScreenBuilder<'a> {
    app: &'a mut App,
    app_state: AppState,
}

impl ScreenBuilder<'_> {
    pub(crate) fn with_enter<T>(self, enter: impl IntoScheduleConfigs<ScheduleSystem, T>) -> Self {
        self.app.add_systems(OnEnter(self.app_state), enter);
        self
    }
    pub(crate) fn with_update<T>(self, update: impl IntoScheduleConfigs<ScheduleSystem, T>) -> Self {
        self.app.add_systems(Update, update.run_if(in_state(self.app_state)));
        self
    }
    pub(crate) fn with_post_update<T>(self, post_update: impl IntoScheduleConfigs<ScheduleSystem, T>) -> Self {
        self.app.add_systems(PostUpdate, post_update.run_if(in_state(self.app_state)));
        self
    }
    pub(crate) fn with_exit<T>(self, exit: impl IntoScheduleConfigs<ScheduleSystem, T>) -> Self {
        self.app.add_systems(OnExit(self.app_state), exit);
        self
    }
}
