use crate::manager::AtlasManager;
use bevy::prelude::*;
use bevy::sprite::{Anchor, MeshMaterial2d};
use bevy::text::TextBounds;
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
    size: Vec2,
}

type ColorMap = HashMap<String, Srgba>;

impl ScreenLayout {
    fn parse_position_size(rect: &str, z: f32) -> Option<(Vec3, Vec2)> {
        let mut components = rect.split(",");

        let x = components.next()?.parse().ok()?;
        let y = components.next()?.parse::<f32>().ok()?;
        let w = components.next()?.parse().ok()?;
        let h = components.next()?.parse().ok()?;

        Some((Vec3::new(x, -y, z), Vec2::new(w, h)))
    }

    fn parse_position(position: &str, z: f32) -> Option<Vec3> {
        let (x_str, y_str) = position.split_once(",")?;
        let x = x_str.parse().ok()?;
        let y = y_str.parse::<f32>().ok()?;
        Some(Vec3::new(x, -y, z))
    }

    fn parse_shape_kind(shape_kind: &str) -> Option<ShapeKind> {
        match shape_kind {
            "rect" => Some(ShapeKind::Rect),
            _ => None,
        }
    }

    fn parse_shape(&mut self, name: &str, remain: &str, z: f32, colors: &mut ColorMap) -> Option<bool> {
        let (shape_kind, remain) = remain.split_once("@")?;
        let (position_size, color) = remain.split_once("!")?;
        let (raw_position, size) = Self::parse_position_size(position_size, z)?;
        let position = Vec3::new(raw_position.x + (size.x / 2.0), raw_position.y - (size.y / 2.0), raw_position.z);

        let element = ShapeElement {
            kind: Self::parse_shape_kind(shape_kind)?,
            position,
            size,
            color: *colors.get(color)?,
        };

        if self.element_map.insert(name.to_string(), Element::Shape(element)).is_some() {
            return None;
        }
        Some(true)
    }

    fn parse_sprite(&mut self, name: &str, remain: &str, z: f32, colors: &mut ColorMap) -> Option<bool> {
        let (atlas_item, remain) = remain.split_once('@')?;
        let (atlas, item) = atlas_item.split_once('.')?;
        let (position, color) = remain.split_once("!")?;

        let element = SpriteElement {
            atlas: atlas.to_string(),
            item: item.to_string(),
            position: Self::parse_position(position, z)?,
            color: *colors.get(color)?,
        };

        if self.element_map.insert(name.to_string(), Element::Sprite(element)).is_some() {
            return None;
        }

        Some(true)
    }

    fn parse_font_info(font_info: &str) -> Option<(&'static str, f32, JustifyText)> {
        let mut parts = font_info.split(".");
        let name_str = parts.next()?;
        let font_size = parts.next()?.parse::<f32>().ok()?;
        let justify_str = parts.next()?;

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
        Some((font_name, font_size, justify))
    }

    fn parse_text(&mut self, name: &str, remain: &str, z: f32, colors: &mut ColorMap) -> Option<bool> {
        let (font_info, remain) = remain.split_once('@')?;
        let (position_size, remain) = remain.split_once('!')?;
        let (color, default_text) = remain.split_once('|')?;

        let (position, size) = Self::parse_position_size(position_size, z)?;
        let (font_name, font_size, justify) = Self::parse_font_info(font_info)?;

        let element = TextElement {
            default_text: default_text.to_string(),
            color: *colors.get(color)?,
            font_name,
            font_size,
            position,
            size,
            justify,
        };
        if self.element_map.insert(name.to_string(), Element::Text(element)).is_some() {
            return None;
        }
        Some(true)
    }
    fn parse_layout(&mut self, name: &str, remain: &str, z: f32) -> Option<bool> {
        let (layout, position_size) = remain.split_once('@')?;
        let (position, size) = Self::parse_position_size(position_size, z)?;

        let element = LayoutElement {
            layout: layout.to_string(),
            position,
            size,
        };
        if self.element_map.insert(name.to_string(), Element::Layout(element)).is_some() {
            return None;
        }
        Some(true)
    }

    pub(crate) fn entity(&self, name: &str) -> Entity {
        *self.entity_map.get(name).unwrap_or(&Entity::PLACEHOLDER)
    }
}

fn fail<T>(_: T) -> std::io::Error {
    std::io::ErrorKind::Other.into()
}

#[derive(Bundle, Default)]
struct ScreenLayoutContainer {
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
}

type BevyAssetsForBuildLayout<'a> = (&'a AssetServer, &'a mut Assets<Mesh>, &'a mut Assets<ColorMaterial>);

impl ScreenLayoutManager {
    pub(crate) fn new(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let mut layout_map = HashMap::new();

        for entry in std::fs::read_dir(path)? {
            let entry = entry.map_err(fail)?.path();
            let name = entry.file_stem().ok_or(std::io::ErrorKind::Other).map_err(fail)?.to_string_lossy().to_string();
            let layout = Self::load(&name, entry)?;
            layout_map.insert(name, layout);
        }

        Ok(Self {
            layout_map,
            entity_map: HashMap::new(),
        })
    }

    fn load(layout_name: &str, path: impl AsRef<Path>) -> Result<ScreenLayout, std::io::Error> {
        const Z_OFFSET: f32 = 0.01;
        let mut z = 0.0;

        let layout_file = std::fs::read_to_string(path)?;

        let mut colors = HashMap::new();
        colors.insert("black".to_string(), Srgba::BLACK);
        colors.insert("white".to_string(), Srgba::WHITE);

        let mut screen_layout = ScreenLayout::default();
        for (line_number, line) in layout_file.lines().enumerate() {
            let parsed = Self::parse_kind(line) //
                .and_then(|(kind, remain)| Some((kind, Self::parse_name(remain)?)))
                .and_then(|(kind, (name, remain))| match kind {
                    '$' => match name {
                        "color" => Self::parse_color(remain, &mut colors),
                        _ => None,
                    },
                    '*' => screen_layout.parse_sprite(name, remain, z, &mut colors),
                    '&' => screen_layout.parse_text(name, remain, z, &mut colors),
                    '#' => screen_layout.parse_shape(name, remain, z, &mut colors),
                    '/' => screen_layout.parse_layout(name, remain, z),
                    _ => None,
                });

            if let Some(increment_z) = parsed {
                if increment_z {
                    z += Z_OFFSET;
                }
            } else {
                println!("[{layout_name}.self], Parse error at line {}", line_number + 1);
            }
        }

        Ok(screen_layout)
    }

    fn parse_name(remain: &str) -> Option<(&str, &str)> {
        remain.split_once(':')
    }

    fn parse_kind(line: &str) -> Option<(char, &str)> {
        let mut chars = line.chars();
        let (kind, remain) = (chars.next()?, chars.as_str());
        Some((kind, remain))
    }

    fn parse_color(remain: &str, colors: &mut HashMap<String, Srgba>) -> Option<bool> {
        let (name, remain) = remain.split_once('=')?;
        let mut components = remain.split(',');
        let r = components.next()?.parse::<f32>().ok()?;
        let g = components.next()?.parse::<f32>().ok()?;
        let b = components.next()?.parse::<f32>().ok()?;
        let a = components.next()?.parse::<f32>().ok()?;

        colors.insert(name.to_string(), Srgba::new(r, g, b, a));

        Some(false)
    }

    fn make_shape_bundle(element: &ShapeElement, offset_z: f32, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> impl Bundle {
        let mesh = match element.kind {
            ShapeKind::Rect => Mesh2d(meshes.add(Rectangle::new(element.size.x, element.size.y))),
        };
        let material = MeshMaterial2d(materials.add(Color::Srgba(element.color)));
        let translation = Vec3::new(element.position.x, element.position.y, element.position.z + offset_z);

        (mesh, material, Transform::from_translation(translation))
    }

    fn make_sprite_bundle(am: &AtlasManager, atlas_name: &str, texture_name: &str, translation: Vec3, color: Srgba) -> Option<impl Bundle> {
        let (atlas, image) = am.get_atlas_texture(atlas_name, texture_name)?;

        let sprite = (
            Sprite {
                color: Color::Srgba(color),
                anchor: Anchor::TopLeft,
                image,
                texture_atlas: Some(atlas),
                ..default()
            },
            Transform::from_translation(translation),
        );
        Some(sprite)
    }

    fn make_text_bundle(element: &TextElement, offset_z: f32, asset_server: &AssetServer) -> impl Bundle {
        let font = asset_server.load(element.font_name);
        let translation = Vec3::new(element.position.x, element.position.y - (element.size.y/2.0), element.position.z + offset_z);

        (
            //
            Text2d::new(&element.default_text),
            TextBounds::from(element.size),
            TextColor::from(element.color),
            TextFont::from_font(font).with_font_size(element.font_size),
            TextLayout::new_with_justify(element.justify),
            Transform::from_translation(translation),
            Anchor::CenterLeft,
        )
    }

    fn make_container_bundle(element: &LayoutElement, offset_z: f32) -> impl Bundle {
        let translation = Vec3::new(element.position.x, element.position.y, element.position.z + offset_z);
        (
            Sprite {
                color: Color::NONE,
                anchor: Anchor::TopLeft,
                custom_size: Some(element.size),
                ..default()
            },
            Transform::from_translation(translation),
            PickingBehavior::IGNORE,
        )
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
                        parent.spawn((sprite, PickingBehavior::IGNORE)).id()
                    } else {
                        continue;
                    }
                }
                Element::Text(e) => {
                    let text = Self::make_text_bundle(e, *offset_z, asset_server);
                    parent.spawn(text).id()
                }
                Element::Layout(e) => {
                    let container = Self::make_container_bundle(e, *offset_z);
                    parent
                        .spawn(container)
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
            .spawn(ScreenLayoutContainer::default())
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
