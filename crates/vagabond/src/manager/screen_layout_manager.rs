use crate::manager::AtlasManager;
use crate::system::ui::Screen;
use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::text::Text2dBounds;
use bevy_mod_picking::prelude::*;
use std::collections::HashMap;
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
    pickable: bool,
}

struct TextElement {
    default_text: String,
    color: Srgba,
    font_name: &'static str,
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

        let x = x_str.parse().unwrap_or_default();
        let y = y_str.parse::<f32>().unwrap_or_default();
        let w = w_str.parse().unwrap_or_default();
        let h = h_str.parse().unwrap_or_default();

        (Vec3::new(x, -y, z), Vec2::new(w, h))
    }

    fn parse_position(position: &str, z: f32) -> Vec3 {
        let (x_str, y_str) = position.split_once(",").unwrap();
        let x = x_str.parse().unwrap_or_default();
        let y = y_str.parse::<f32>().unwrap_or_default();
        Vec3::new(x, -y, z)
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
        let (raw_position, size) = Self::parse_position_size(position_size, z);
        let position = Vec3::new(raw_position.x + (size.x / 2.0), raw_position.y - (size.y / 2.0), raw_position.z);

        let element = ShapeElement {
            kind: Self::parse_shape_kind(shape_kind),
            position,
            size,
            color: Self::parse_color(color),
        };

        self.element_map.insert(name.to_string(), Element::Shape(element));
    }

    fn parse_sprite(&mut self, name: &str, remain: &str, pickable: bool, z: f32) {
        let (atlas_item, remain) = remain.split_once('@').unwrap();
        let (atlas, item) = atlas_item.split_once('.').unwrap();
        let (position, color) = remain.split_once("!").unwrap();

        let element = SpriteElement {
            atlas: atlas.to_string(),
            item: item.to_string(),
            position: Self::parse_position(position, z),
            color: Self::parse_color(color),
            pickable,
        };

        self.element_map.insert(name.to_string(), Element::Sprite(element));
    }

    fn parse_font_info(font_info: &str) -> (&'static str, f32, JustifyText) {
        let mut parts = font_info.split(".");
        let name_str = parts.next().unwrap_or("main");
        let font_size = parts.next().unwrap_or("12").parse::<f32>().unwrap_or(12.0);
        let justify_str = parts.next().unwrap_or("center");

        let font_name = match name_str {
            "main" => "font/RobotoMono.ttf",
            _ => "font/RobotoMono.ttf",
        };
        let justify = match justify_str {
            "left" => JustifyText::Left,
            "center" => JustifyText::Center,
            "right" => JustifyText::Right,
            _ => JustifyText::Center,
        };
        (font_name, font_size, justify)
    }

    fn parse_text(&mut self, name: &str, remain: &str, z: f32) {
        let (font_info, remain) = remain.split_once('@').unwrap();
        let (position_size, remain) = remain.split_once('!').unwrap();
        let (color, default_text) = remain.split_once('|').unwrap();

        let (position, size) = Self::parse_position_size(position_size, z);
        let (font_name, font_size, justify) = Self::parse_font_info(font_info);

        let element = TextElement {
            default_text: default_text.to_string(),
            color: Self::parse_color(color),
            font_name,
            font_size,
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

    pub(crate) fn decorate(&self, commands: &mut Commands, name: &str, decorator: impl Bundle) -> Entity {
        if let Some(entity) = self.entity_map.get(name) {
            commands.entity(*entity).insert(decorator);
            *entity
        } else {
            Entity::PLACEHOLDER
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

impl ScreenLayoutContainer {
    fn new_at(translation: Vec3) -> Self {
        Self {
            transform: Transform::from_translation(translation),
            ..default()
        }
    }
}

fn fail<T>(_: T) -> std::io::Error {
    std::io::ErrorKind::Other.into()
}

type BevyAssetsForBuildLayout<'a> = (&'a AssetServer, &'a mut Assets<Mesh>, &'a mut Assets<ColorMaterial>);

impl ScreenLayoutManager {
    pub(crate) fn new(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let mut layout_map = HashMap::new();

        for entry in std::fs::read_dir(path)? {
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
        const Z_OFFSET: f32 = 0.01;
        let mut z = 0.0;

        let layout_file = std::fs::read_to_string(path)?;

        let mut screen_layout = ScreenLayout::default();
        for line in layout_file.lines() {
            let mut chars = line.chars();
            let (kind, remain) = (chars.next().unwrap(), chars.collect::<String>());
            let (name, remain) = remain.split_once(':').unwrap();
            match kind {
                '*' => screen_layout.parse_sprite(name, remain, false, z),
                '?' => screen_layout.parse_sprite(name, remain, true, z),
                '+' => screen_layout.parse_layout(name, remain, z),
                '&' => screen_layout.parse_text(name, remain, z),
                '#' => screen_layout.parse_shape(name, remain, z),
                _ => continue,
            }
            z += Z_OFFSET;
        }

        Ok(screen_layout)
    }

    fn make_shape_bundle(element: &ShapeElement, offset_z: f32, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> impl Bundle {
        let mesh = match element.kind {
            ShapeKind::Rect => Mesh2dHandle(meshes.add(Rectangle::new(element.size.x, element.size.y))),
        };
        let material = materials.add(Color::Srgba(element.color));
        let translation = Vec3::new(element.position.x, element.position.y, element.position.z + offset_z);

        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_translation(translation),
            ..default()
        }
    }
    pub(crate) fn make_sprite_bundle(am: &AtlasManager, atlas_name: &str, texture_name: &str, translation: Vec3, color: Srgba) -> Option<(SpriteBundle, TextureAtlas)> {
        let (atlas, texture) = am.get_atlas_texture(atlas_name, texture_name)?;

        let sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::Srgba(color),
                anchor: Anchor::TopLeft,
                ..default()
            },
            texture,
            transform: Transform::from_translation(translation),
            ..default()
        };
        Some((sprite, atlas))
    }

    fn make_text_bundle(element: &TextElement, offset_z: f32, asset_server: &AssetServer) -> impl Bundle {
        let font = asset_server.load(element.font_name);
        let translation = Vec3::new(element.position.x, element.position.y, element.position.z + offset_z);

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

    fn build_layout(&self, layout_name: &str, parent: &mut ChildBuilder, offset_z: &mut f32, base_name: &str, am: &AtlasManager, (asset_server, meshes, materials): BevyAssetsForBuildLayout) -> Vec<(String, Entity)> {
        let mut entities = Vec::new();
        let layout = self.layout_map.get(layout_name).unwrap();

        for (name, element) in &layout.element_map {
            let full_name = Self::make_name(base_name, name);
            let id = match element {
                Element::Shape(e) => {
                    let shape = Self::make_shape_bundle(e, *offset_z, meshes, materials);
                    parent.spawn(shape).id()
                }
                Element::Sprite(e) => {
                    let translation = Vec3::new(e.position.x, e.position.y, e.position.z + *offset_z);
                    if let Some(sprite) = Self::make_sprite_bundle(am, e.atlas.as_str(), e.item.as_str(), translation, e.color) {
                        if e.pickable {
                            parent.spawn((sprite, PickableBundle::default())).id()
                        } else {
                            parent.spawn((sprite, Pickable::IGNORE)).id()
                        }
                    } else {
                        continue;
                    }
                }
                Element::Text(e) => {
                    let text = Self::make_text_bundle(e, *offset_z, asset_server);
                    parent.spawn(text).id()
                }
                Element::Layout(e) => {
                    let translation = Vec3::new(e.position.x, e.position.y, e.position.z + *offset_z);
                    parent
                        .spawn(ScreenLayoutContainer::new_at(translation))
                        .with_children(|parent| {
                            let mut nested_entities = self.build_layout(&e.layout, parent, offset_z, &full_name, am, (asset_server, meshes, materials));
                            entities.append(&mut nested_entities);
                        })
                        .id()
                }
            };
            entities.push((full_name, id));
        }

        entities
    }

    pub(crate) fn build(&mut self, commands: &mut Commands, layout_name: &str, am: &AtlasManager, asset_server: &AssetServer, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> &ScreenLayout {
        let id = commands
            .spawn((ScreenLayoutContainer::default(), Screen))
            .with_children(|parent| {
                let mut offset = 0.0;
                let entities = self.build_layout(layout_name, parent, &mut offset, "", am, (asset_server, meshes, materials));
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
