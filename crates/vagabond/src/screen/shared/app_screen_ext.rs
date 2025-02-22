use crate::system::AppState;
use bevy::app::{App, PostUpdate, Update};
use bevy::prelude::{IntoSystemConfigs, OnEnter, OnExit, in_state};

pub(crate) trait AppScreenExt {
    fn add_screen(&mut self, app_state: AppState) -> ScreenBuilder;
}

impl AppScreenExt for App {
    fn add_screen(&mut self, app_state: AppState) -> ScreenBuilder {
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
    pub(crate) fn with_enter<T>(self, enter: impl IntoSystemConfigs<T>) -> Self {
        self.app.add_systems(OnEnter(self.app_state), enter);
        self
    }
    pub(crate) fn with_update<T>(self, update: impl IntoSystemConfigs<T>) -> Self {
        self.app.add_systems(Update, update.run_if(in_state(self.app_state)));
        self
    }
    pub(crate) fn with_post_update<T>(self, post_update: impl IntoSystemConfigs<T>) -> Self {
        self.app.add_systems(PostUpdate, post_update.run_if(in_state(self.app_state)));
        self
    }
    pub(crate) fn with_exit<T>(self, exit: impl IntoSystemConfigs<T>) -> Self {
        self.app.add_systems(OnExit(self.app_state), exit);
        self
    }
}
