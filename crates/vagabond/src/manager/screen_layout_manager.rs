use crate::manager::AtlasManager;
use crate::system::ui::Screen;
use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::text::Text2dBounds;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Resource)]
pub(crate) struct ScreenLayoutManager {
    layout_map: HashMap<String, ScreenLayout>,
    entity_map: HashMap<String, Entity>,
}

#[derive(Default)]
pub(crate) struct ScreenLayout {
    element_map: HashMap<String, Element>,
    entity_map: HashMap<String, Entity>,
}

enum Element {
    Shape(ShapeElement),
    Sprite(SpriteElement),
    Text(TextElement),
    Layout(LayoutElement),
}

enum ShapeKind {
    Rect,
}

struct ShapeElement {
    kind: ShapeKind,
    position: Vec3,
    size: Vec2,
    color: Srgba,
}

struct SpriteElement {
    atlas: String,
    item: String,
    position: Vec3,
    color: Srgba,
}

struct TextElement {
    default_text: String,
    color: Srgba,
    font_size: f32,
    position: Vec3,
    size: Vec2,
    justify: JustifyText,
}

struct LayoutElement {
    layout: String,
    position: Vec3,
}

impl ScreenLayout {
    fn parse_color(color: &str) -> Srgba {
        match color {
            "black" => bevy::color::palettes::css::BLACK,
            "grey" => bevy::color::palettes::css::DARK_GRAY,
            "yellow" => bevy::color::palettes::css::DARK_GOLDENROD,
            "red" => bevy::color::palettes::css::DARK_RED,
            "green" => bevy::color::palettes::css::DARK_GREEN,
            "purple" => bevy::color::palettes::css::DARK_BLUE,
            "white" => bevy::color::palettes::css::WHITE,
            "silver" => bevy::color::palettes::css::SILVER,
            _ => Srgba::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    fn parse_position_size(rect: &str, z: f32) -> (Vec3, Vec2) {
        let [x_str, y_str, w_str, h_str] = rect.split(",").collect::<Vec<&str>>().try_into().unwrap();

        let x = x_str.parse::<u32>().unwrap_or_default() as f32;
        let y = y_str.parse::<u32>().unwrap_or_default() as f32;
        let w = w_str.parse::<u32>().unwrap_or_default() as f32;
        let h = h_str.parse::<u32>().unwrap_or_default() as f32;

        (Vec3::new(x, y, z), Vec2::new(w, h))
    }

    fn parse_position(position: &str, z: f32) -> Vec3 {
        let (x, y) = position.split_once(",").unwrap();
        Vec3 {
            x: x.parse().unwrap_or_default(),
            y: y.parse().unwrap_or_default(),
            z,
        }
    }

    fn parse_shape_kind(shape_kind: &str) -> ShapeKind {
        match shape_kind {
            "rect" => ShapeKind::Rect,
            _ => ShapeKind::Rect,
        }
    }

    fn parse_shape(&mut self, name: &str, remain: &str, z: f32) {
        let (shape_kind, remain) = remain.split_once("@").unwrap();
        let (position_size, color) = remain.split_once("!").unwrap();
        let (position, size) = Self::parse_position_size(position_size, z);

        let element = ShapeElement {
            kind: Self::parse_shape_kind(shape_kind),
            position,
            size,
            color: Self::parse_color(color),
        };

        self.element_map.insert(name.to_string(), Element::Shape(element));
    }

    fn parse_sprite(&mut self, name: &str, remain: &str, z: f32) {
        let (atlas_item, remain) = remain.split_once('@').unwrap();
        let (atlas, item) = atlas_item.split_once('.').unwrap();
        let (position, color) = remain.split_once("!").unwrap();

        let element = SpriteElement {
            atlas: atlas.to_string(),
            item: item.to_string(),
            position: Self::parse_position(position, z),
            color: Self::parse_color(color),
        };

        self.element_map.insert(name.to_string(), Element::Sprite(element));
    }
    fn parse_text(&mut self, name: &str, remain: &str, justify: JustifyText, z: f32) {
        let (default_text, remain) = remain.split_once('@').unwrap();
        let (position_size, remain) = remain.split_once('!').unwrap();
        let (color, font_size) = remain.split_once(',').unwrap();

        let (position, size) = Self::parse_position_size(position_size, z);

        let element = TextElement {
            default_text: default_text.to_string(),
            color: Self::parse_color(color),
            font_size: font_size.parse().unwrap(),
            position,
            size,
            justify,
        };
        self.element_map.insert(name.to_string(), Element::Text(element));
    }
    fn parse_layout(&mut self, name: &str, remain: &str, z: f32) {
        let (layout, remain) = remain.split_once('@').unwrap();
        let element = LayoutElement {
            layout: layout.to_string(),
            position: Self::parse_position(remain, z),
        };
        self.element_map.insert(name.to_string(), Element::Layout(element));
    }

    pub(crate) fn decorate(&self, commands: &mut Commands, name: &str, decorator: impl Bundle) {
        if let Some(entity) = self.entity_map.get(name) {
            commands.entity(*entity).insert(decorator);
        }
    }
}

#[derive(Bundle, Default)]
struct ScreenLayoutContainer {
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
}

fn fail<T>(_: T) -> std::io::Error {
    std::io::ErrorKind::Other.into()
}

impl ScreenLayoutManager {
    pub(crate) fn new(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let mut layout_map = HashMap::new();

        for entry in fs::read_dir(path)? {
            let entry = entry.map_err(fail)?.path();
            let name = entry.file_stem().ok_or(std::io::ErrorKind::Other).map_err(fail)?.to_string_lossy().to_string();
            let layout = Self::load(entry)?;
            layout_map.insert(name, layout);
        }

        Ok(Self {
            layout_map,
            entity_map: HashMap::new(),
        })
    }
    fn load(path: impl AsRef<Path>) -> Result<ScreenLayout, std::io::Error> {
        const Z_OFFSET: f32 = 0.001;
        let mut z = 0.0;

        let layout_file = fs::read_to_string(path)?;

        let mut screen_layout = ScreenLayout::default();
        for line in layout_file.lines() {
            let mut chars = line.chars();
            let (kind, remain) = (chars.next().unwrap(), chars.collect::<String>());
            let (name, remain) = remain.split_once(':').unwrap();
            match kind {
                '*' => screen_layout.parse_sprite(name, remain, z),
                '+' => screen_layout.parse_layout(name, remain, z),
                '^' => screen_layout.parse_text(name, remain, JustifyText::Left, z),
                '&' => screen_layout.parse_text(name, remain, JustifyText::Center, z),
                '$' => screen_layout.parse_text(name, remain, JustifyText::Right, z),
                '#' => screen_layout.parse_shape(name, remain, z),
                _ => continue,
            }
            z += Z_OFFSET;
        }

        Ok(screen_layout)
    }

    fn make_shape_bundle(element: &ShapeElement, offset: Vec3, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> impl Bundle {
        let mesh = match element.kind {
            ShapeKind::Rect => Mesh2dHandle(meshes.add(Rectangle::new(element.size.x, element.size.y))),
        };
        let material = materials.add(Color::Srgba(element.color));
        let position = element.position + (element.size.extend(0.0) / 2.0);
        let translation = Vec3::new(position.x + offset.x, -(position.y + offset.y), position.z + offset.z);

        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_translation(translation),
            ..default()
        }
    }

    fn make_text_bundle(element: &TextElement, offset: Vec3, font: Handle<Font>) -> impl Bundle {
        let translation = Vec3::new(element.position.x + offset.x, -(element.position.y + offset.y), element.position.z + offset.z);

        Text2dBundle {
            text: Text::from_section(
                element.default_text.clone(),
                TextStyle {
                    font,
                    font_size: element.font_size,
                    color: Color::Srgba(element.color),
                },
            )
            .with_justify(element.justify),
            text_anchor: Anchor::TopLeft,
            text_2d_bounds: Text2dBounds {
                size: element.size,
            },
            transform: Transform::from_translation(translation),
            ..default()
        }
    }

    fn make_name(base_name: &str, name: &str) -> String {
        if base_name.is_empty() {
            name.to_string()
        } else {
            format!("{}/{}", base_name, name)
        }
    }

    fn build_layout(&self, layout_name: &str, parent: &mut ChildBuilder, offset: &mut Vec3, base_name: &str, font_handle: Handle<Font>, am: &AtlasManager, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> Vec<(String, Entity)> {
        let mut entities = Vec::new();
        let layout = self.layout_map.get(layout_name).unwrap();

        for (name, element) in &layout.element_map {
            let full_name = Self::make_name(base_name, name);
            let id = match element {
                Element::Shape(e) => {
                    let shape = Self::make_shape_bundle(e, *offset, meshes, materials);
                    let id = parent.spawn(shape).id();
                    Some(id)
                }
                Element::Sprite(e) => {
                    let translation = Vec3::new(e.position.x + offset.x, -(e.position.y + offset.y), e.position.z + offset.z);
                    let sprite = am.make_sprite_bundle(e.atlas.as_str(), e.item.as_str(), translation, e.color).unwrap();
                    let id = parent.spawn(sprite).id();
                    Some(id)
                }
                Element::Text(e) => {
                    let text = Self::make_text_bundle(e, *offset, font_handle.clone());
                    let id = parent.spawn(text).id();
                    Some(id)
                }
                Element::Layout(e) => {
                    let mut new_offset = e.position + *offset;
                    let mut nested_entities = self.build_layout(&e.layout, parent, &mut new_offset, &full_name, font_handle.clone(), am, meshes, materials);
                    entities.append(&mut nested_entities);
                    offset.z = new_offset.z;
                    None
                }
            };
            if let Some(id) = id {
                entities.push((full_name, id));
            }
        }

        entities
    }

    pub(crate) fn build(&mut self, commands: &mut Commands, layout_name: &str, font_handle: Handle<Font>, am: &AtlasManager, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> &ScreenLayout {
        let id = commands
            .spawn((ScreenLayoutContainer::default(), Screen))
            .with_children(|parent| {
                let mut offset = Vec3::default();
                let entities = self.build_layout(layout_name, parent, &mut offset, "", font_handle, am, meshes, materials);
                let layout = self.layout_map.get_mut(layout_name).unwrap();
                layout.entity_map = entities.into_iter().collect();
            })
            .id();
        self.entity_map.insert(layout_name.to_string(), id);
        self.layout_map.get(layout_name).unwrap()
    }

    pub(crate) fn destroy(&mut self, mut commands: Commands, layout_name: &str) {
        let entity = self.entity_map.remove(layout_name).unwrap();
        commands.entity(entity).despawn_recursive();
    }
}
