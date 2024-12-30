use crate::system::AppState;
use bevy::app::{App, PostUpdate, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit};

pub(crate) trait AppScreenExt {
    fn build_screen<E, U, X>(&mut self, app_state: AppState, enter: impl IntoSystemConfigs<E>, update: impl IntoSystemConfigs<U>, exit: impl IntoSystemConfigs<X>);
    fn build_screen_with_post_update<E, U, P, X>(&mut self, app_state: AppState, enter: impl IntoSystemConfigs<E>, update: impl IntoSystemConfigs<U>, post_update: impl IntoSystemConfigs<P>, exit: impl IntoSystemConfigs<X>);
}

impl AppScreenExt for App {
    fn build_screen<E, U, X>(&mut self, app_state: AppState, enter: impl IntoSystemConfigs<E>, update: impl IntoSystemConfigs<U>, exit: impl IntoSystemConfigs<X>) {
        self //
            .add_systems(OnEnter(app_state), enter)
            .add_systems(Update, update.run_if(in_state(app_state)))
            .add_systems(OnExit(app_state), exit);
    }

    fn build_screen_with_post_update<E, U, P, X>(&mut self, app_state: AppState, enter: impl IntoSystemConfigs<E>, update: impl IntoSystemConfigs<U>, post_update: impl IntoSystemConfigs<P>, exit: impl IntoSystemConfigs<X>) {
        self //
            .add_systems(OnEnter(app_state), enter)
            .add_systems(Update, update.run_if(in_state(app_state)))
            .add_systems(PostUpdate, post_update.run_if(in_state(app_state)))
            .add_systems(OnExit(app_state), exit);
    }
}
