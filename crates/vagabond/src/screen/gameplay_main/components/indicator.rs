use bevy::prelude::*;

use crate::screen::gameplay_main::components::MachineKind;

#[derive(Component)]
pub(crate) struct Indicator {
    pub(crate) translation: Vec3,
    pub(crate) offset: Vec2,
    pub(crate) parent: Entity,
    pub(crate) target: Option<MachineKind>,
}

#[derive(Component)]
pub(crate) struct IndicatorTracker;

#[derive(Component)]
pub(crate) struct IndicatorActive;

impl Indicator {
    pub(crate) fn make_bundle(parent: Entity, translation: Vec3, offset: Vec2, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) -> impl Bundle {
        (
            Indicator {
                translation,
                offset,
                parent,
                target: None,
            },
            Mesh2d(meshes.add(Rectangle::new(16.0, 1.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::Srgba(Srgba::new(0.0, 0.75, 0.0, 0.35))))),
            Transform::from_translation(translation),
            PickingBehavior::IGNORE,
        )
    }
}
