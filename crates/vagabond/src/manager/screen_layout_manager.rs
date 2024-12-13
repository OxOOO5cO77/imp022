use crate::manager::AtlasManager;
use bevy::prelude::*;
use bevy::sprite::{Anchor, MeshMaterial2d};
use bevy::text::TextBounds;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::path::{Path, PathBuf};

#[derive(Resource)]
pub(crate) struct ScreenLayoutManager {
    layout_map: HashMap<String, ScreenLayout>,
    entity_map: HashMap<String, Entity>,
}

#[derive(Default)]
struct ScreenResources {
    color_map: HashMap<String, Srgba>,
    font_map: HashMap<String, String>,
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
    CapsuleX,
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
    font_name: String,
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

impl ScreenLayout {
    fn parse_size(rect: &str) -> Option<Vec2> {
        let mut components = rect.split(",");

        let w = components.next()?.parse::<f32>().ok()?;
        let h = components.next()?.parse::<f32>().ok()?;

        Some(Vec2::new(w, h))
    }

    fn parse_position(position: &str) -> Option<Vec3> {
        const Z_FACTOR: f32 = 0.01;
        let mut components = position.split(",");

        let x = components.next()?.parse::<f32>().ok()?;
        let y = components.next()?.parse::<f32>().ok()?;
        let z = components.next()?.parse::<f32>().ok()?;

        Some(Vec3::new(x, -y, z * Z_FACTOR))
    }

    fn parse_shape_kind(shape_kind: &str) -> Option<ShapeKind> {
        match shape_kind {
            "rect" => Some(ShapeKind::Rect),
            "capsule_x" => Some(ShapeKind::CapsuleX),
            _ => None,
        }
    }

    fn parse_shape(&mut self, name: &str, remain: &str, resources: &ScreenResources, overrides: &ScreenResources) -> Option<bool> {
        let (shape_kind_str, remain) = remain.split_once("%")?;
        let (size_str, remain) = remain.split_once("@")?;
        let (position_str, color_str) = remain.split_once("!")?;

        let raw_position = Self::parse_position(position_str)?;
        let size = Self::parse_size(size_str)?;

        let position = Vec3::new(raw_position.x + (size.x / 2.0), raw_position.y - (size.y / 2.0), raw_position.z);

        let element = ShapeElement {
            kind: Self::parse_shape_kind(shape_kind_str)?,
            position,
            size,
            color: *overrides.color_map.get(color_str).or(resources.color_map.get(color_str))?,
        };

        if self.element_map.insert(name.to_string(), Element::Shape(element)).is_some() {
            return None;
        }
        Some(true)
    }

    fn parse_sprite(&mut self, name: &str, remain: &str, resources: &ScreenResources, overrides: &ScreenResources) -> Option<bool> {
        let (atlas_item, remain) = remain.split_once('@')?;
        let (atlas, item) = atlas_item.split_once('.')?;
        let (position, color) = remain.split_once("!")?;

        let element = SpriteElement {
            atlas: atlas.to_string(),
            item: item.to_string(),
            position: Self::parse_position(position)?,
            color: *overrides.color_map.get(color).or(resources.color_map.get(color))?,
        };

        if self.element_map.insert(name.to_string(), Element::Sprite(element)).is_some() {
            return None;
        }

        Some(true)
    }

    fn parse_font_info(font_info: &str, resources: &ScreenResources, overrides: &ScreenResources) -> Option<(String, f32, JustifyText)> {
        let mut parts = font_info.split(".");
        let name_str = parts.next()?;
        let font_size = parts.next()?.parse::<f32>().ok()?;
        let justify_str = parts.next()?;

        let font_name = overrides.font_map.get(name_str).or(resources.font_map.get(name_str))?.clone();
        let justify = match justify_str {
            "left" => JustifyText::Left,
            "center" => JustifyText::Center,
            "right" => JustifyText::Right,
            _ => JustifyText::Center,
        };
        Some((font_name, font_size, justify))
    }

    fn parse_text(&mut self, name: &str, remain: &str, resources: &ScreenResources, overrides: &ScreenResources) -> Option<bool> {
        let (font_info, remain) = remain.split_once('%')?;
        let (size_str, remain) = remain.split_once('@')?;
        let (position_str, remain) = remain.split_once('!')?;
        let (color, default_text) = remain.split_once('|')?;

        let size = Self::parse_size(size_str)?;
        let position = Self::parse_position(position_str)?;
        let (font_name, font_size, justify) = Self::parse_font_info(font_info, resources, overrides)?;

        let element = TextElement {
            default_text: default_text.to_string(),
            color: *overrides.color_map.get(color).or(resources.color_map.get(color))?,
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
    fn parse_layout(&mut self, name: &str, remain: &str) -> Option<bool> {
        let (layout, remain) = remain.split_once('%')?;
        let (size_str, position_str) = remain.split_once('@')?;

        let size = Self::parse_size(size_str)?;
        let position = Self::parse_position(position_str)?;

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

    pub(crate) fn entity_option(&self, name: &str) -> Option<&Entity> {
        self.entity_map.get(name)
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

        let mut init_path = PathBuf::from(path.as_ref());
        init_path.push("_.self");
        let resources = Self::load_init(init_path)?;

        for entry in std::fs::read_dir(path)? {
            let entry = entry.map_err(fail)?.path();
            let name = entry.file_stem().ok_or(std::io::ErrorKind::Other).map_err(fail)?.to_string_lossy().to_string();
            if name != "_" {
                let layout = Self::load(&name, entry, &resources)?;
                layout_map.insert(name, layout);
            }
        }
        let screen_layout_manager = Self {
            layout_map,
            entity_map: HashMap::new(),
        };
        Ok(screen_layout_manager)
    }

    fn load_init(path: impl AsRef<Path>) -> Result<ScreenResources, std::io::Error> {
        let init_file = std::fs::read_to_string(path)?;

        let base_resources = ScreenResources::default();
        //todo: pre-populate with CSS colors?

        let mut resources = ScreenResources::default();
        resources.color_map.insert("black".to_string(), Srgba::BLACK);
        resources.color_map.insert("white".to_string(), Srgba::WHITE);

        for (line_number, line) in init_file.lines().enumerate() {
            let parsed = Self::parse_kind(line) //
                .and_then(|(kind, remain)| Some((kind, Self::parse_name(remain)?)))
                .and_then(|(kind, (name, remain))| match kind {
                    '$' => Self::parse_resource(name, remain, &base_resources, &mut resources),
                    _ => None,
                });
            if parsed.is_none() {
                println!("[_.self] Parse error at line {}", line_number + 1);
            }
        }

        Ok(resources)
    }

    fn load(layout_name: &str, path: impl AsRef<Path>, resources: &ScreenResources) -> Result<ScreenLayout, std::io::Error> {
        let layout_file = std::fs::read_to_string(path)?;

        let mut overrides = ScreenResources::default();

        let mut screen_layout = ScreenLayout::default();
        for (line_number, line) in layout_file.lines().enumerate() {
            let parsed = Self::parse_kind(line) //
                .and_then(|(kind, remain)| Some((kind, Self::parse_name(remain)?)))
                .and_then(|(kind, (name, remain))| match kind {
                    '$' => Self::parse_resource(name, remain, resources, &mut overrides),
                    '*' => screen_layout.parse_sprite(name, remain, resources, &overrides),
                    '&' => screen_layout.parse_text(name, remain, resources, &overrides),
                    '#' => screen_layout.parse_shape(name, remain, resources, &overrides),
                    '/' => screen_layout.parse_layout(name, remain),
                    _ => None,
                });

            if parsed.is_none() {
                println!("[{layout_name}.self] Parse error at line {}", line_number + 1);
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

    fn parse_color_components(remain: &str) -> Option<Srgba> {
        let mut components = remain.split(',');
        let r = components.next()?.parse::<f32>().ok()?;
        let g = components.next()?.parse::<f32>().ok()?;
        let b = components.next()?.parse::<f32>().ok()?;
        let a = components.next()?.parse::<f32>().ok()?;

        Some(Srgba::new(r, g, b, a))
    }

    fn parse_color(remain: &str, in_resources: &ScreenResources, out_colors: &mut HashMap<String, Srgba>) -> Option<bool> {
        let (name, remain) = remain.split_once('=')?;

        let color = in_resources.color_map.get(remain).cloned().or_else(|| Self::parse_color_components(remain))?;

        out_colors.insert(name.to_string(), color);
        Some(false)
    }

    fn parse_font(remain: &str, fonts: &mut HashMap<String, String>) -> Option<bool> {
        let (name, remain) = remain.split_once('=')?;

        fonts.insert(name.to_string(), remain.to_string());

        Some(false)
    }

    fn parse_resource(name: &str, remain: &str, in_resources: &ScreenResources, out_resources: &mut ScreenResources) -> Option<bool> {
        match name {
            "color" => Self::parse_color(remain, in_resources, &mut out_resources.color_map),
            "font" => Self::parse_font(remain, &mut out_resources.font_map),
            _ => None,
        }
    }

    fn make_shape_bundle(element: &ShapeElement, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> impl Bundle {
        let handle = match element.kind {
            ShapeKind::Rect => meshes.add(Rectangle::new(element.size.x, element.size.y)),
            ShapeKind::CapsuleX => meshes.add(Capsule2d::new(element.size.y / 2.0, element.size.x - element.size.y)),
        };
        let mesh = Mesh2d(handle);
        let material = MeshMaterial2d(materials.add(Color::Srgba(element.color)));

        let rotation = match element.kind {
            ShapeKind::Rect => Quat::IDENTITY,
            ShapeKind::CapsuleX => Quat::from_rotation_z(PI / 2.0),
        };

        let transform = Transform {
            translation: element.position,
            rotation,
            scale: Vec3::ONE,
        };

        (mesh, material, transform)
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

    fn make_text_bundle(element: &TextElement, asset_server: &AssetServer) -> impl Bundle {
        let font = asset_server.load(&element.font_name);
        let translation = Vec3::new(element.position.x, element.position.y - (element.size.y / 2.0), element.position.z);

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

    fn make_container_bundle(element: &LayoutElement) -> impl Bundle {
        (
            Sprite {
                color: Color::NONE,
                anchor: Anchor::TopLeft,
                custom_size: Some(element.size),
                ..default()
            },
            Transform::from_translation(element.position),
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

    fn build_layout(&self, layout_name: &str, parent: &mut ChildBuilder, base_name: &str, am: &AtlasManager, (asset_server, meshes, materials): BevyAssetsForBuildLayout) -> Vec<(String, Entity)> {
        let mut entities = Vec::new();
        let layout = self.layout_map.get(layout_name).unwrap();

        for (name, element) in &layout.element_map {
            let full_name = Self::make_name(base_name, name);
            let id = match element {
                Element::Shape(e) => {
                    let shape = Self::make_shape_bundle(e, meshes, materials);
                    parent.spawn(shape).id()
                }
                Element::Sprite(e) => {
                    if let Some(sprite) = Self::make_sprite_bundle(am, &e.atlas, &e.item, e.position, e.color) {
                        parent.spawn((sprite, PickingBehavior::IGNORE)).id()
                    } else {
                        continue;
                    }
                }
                Element::Text(e) => {
                    let text = Self::make_text_bundle(e, asset_server);
                    parent.spawn(text).id()
                }
                Element::Layout(e) => {
                    let container = Self::make_container_bundle(e);
                    parent
                        .spawn(container)
                        .with_children(|parent| {
                            let mut nested_entities = self.build_layout(&e.layout, parent, &full_name, am, (asset_server, meshes, materials));
                            entities.append(&mut nested_entities);
                        })
                        .id()
                }
            };
            entities.push((full_name, id));
        }

        entities
    }

    pub(crate) fn build(&mut self, commands: &mut Commands, layout_name: &str, am: &AtlasManager, (asset_server, mut meshes, mut materials): (Res<AssetServer>, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>)) -> &ScreenLayout {
        let id = commands
            .spawn(ScreenLayoutContainer::default())
            .with_children(|parent| {
                let entities = self.build_layout(layout_name, parent, "", am, (&asset_server, &mut meshes, &mut materials));
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
