use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct DragTarget;

#[derive(Component)]
pub(crate) struct DropTarget;

#[derive(Component)]
pub(crate) struct Dragging(pub Entity);

pub(crate) struct DragDropPlugin;

#[derive(Event)]
pub(crate) struct DragDrag {
    pub(crate) src: Entity,
}

#[derive(Event)]
pub(crate) struct DragDrop {
    pub(crate) src: Entity,
    pub(crate) drag: Entity,
    pub(crate) dst: Option<Entity>,
}

impl Plugin for DragDropPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DragDropState::default())
            .add_event::<DragDrag>()
            .add_event::<DragDrop>()
            .add_systems(Update, (cursor_pos, draggable, dragging, drop))
        ;
    }
}

#[derive(Resource, Default)]
struct DragDropState {
    cursor_pos: Vec2,
}

fn cursor_pos(
    mut state: ResMut<DragDropState>,
    mut cursor_moved: EventReader<CursorMoved>,
) {
    // if cursor hales moved, transform to graphics coordinates and store in state.cursor_pos
    if let Some(cursor_event) = cursor_moved.read().last() {
        state.cursor_pos = cursor_event.position;
    }
}

fn draggable(
    state: Res<DragDropState>,
    input: Res<ButtonInput<MouseButton>>,
    mut draggable: Query<(Entity, &Node, &GlobalTransform), With<DragTarget>>,
    mut notify: EventWriter<DragDrag>,
) {
    if input.just_pressed(MouseButton::Left) {
        for (src, node, transform) in draggable.iter_mut() {
            let orig_pos = transform.translation();
            let pos = orig_pos.truncate();

            if intersects(state.cursor_pos, pos, node.size()) {
                notify.send(DragDrag { src });
            }
        }
    }
}

fn dragging(
    state: Res<DragDropState>,
    input: Res<ButtonInput<MouseButton>>,
    mut dragged: Query<(&Node, &mut Style), With<Dragging>>,
) {
    if input.pressed(MouseButton::Left) {
        for (node, mut style) in dragged.iter_mut() {
            let extents = node.size() / 2.0;
            style.left = Val::Px(state.cursor_pos.x - extents.x);
            style.top = Val::Px(state.cursor_pos.y - extents.y);
        }
    }
}

fn drop(
    state: Res<DragDropState>,
    input: Res<ButtonInput<MouseButton>>,
    mut dragging_q: Query<(Entity, &Dragging), With<Dragging>>,
    droptargets_q: Query<(Entity, &Node, &GlobalTransform), With<DropTarget>>,
    mut notify: EventWriter<DragDrop>,
) {
    if input.just_released(MouseButton::Left) {
        for (drag, dragging) in dragging_q.iter_mut() {
            let dst = droptargets_q
                .iter()
                .find(|(_, node, transform)| intersects(state.cursor_pos, transform.translation().truncate(), node.size()))
                .map(|(drop, _, _)| drop);

            notify.send(DragDrop {
                src: dragging.0,
                drag,
                dst,
            });
        }
    }
}

fn intersects(point: Vec2, target_pos: Vec2, target_size: Vec2) -> bool {
    let extents = target_size / 2.0;
    let min = target_pos - extents;
    let max = target_pos + extents;
    (min.x..max.x).contains(&point.x) && (min.y..max.y).contains(&point.y)
}
