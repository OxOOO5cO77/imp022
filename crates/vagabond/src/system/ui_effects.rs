use bevy::app::{PluginGroup, PluginGroupBuilder};

mod blinker;
mod color;
mod glower;
mod hider;
mod size;
mod text_tip;

pub(crate) use blinker::Blinker;
pub(crate) use color::{SetColorEvent, UiFxTrackedColor};
pub(crate) use glower::Glower;
pub(crate) use hider::Hider;
pub(crate) use size::UiFxTrackedSize;
pub(crate) use text_tip::TextTip;

pub(crate) struct UiEffectsPlugins;

impl PluginGroup for UiEffectsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>() //
            .add(blinker::BlinkerPlugin)
            .add(glower::GlowerPlugin)
            .add(hider::HiderPlugin)
            .add(color::ColorPlugin)
    }
}
