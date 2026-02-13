use std::collections::HashMap;
use std::f32::consts::PI;
use std::path::{Path, PathBuf};

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::TextBounds;
use bevy::ui::FocusPolicy;
use bevy_simple_text_input::{TextInput, TextInputInactive, TextInputSettings, TextInputTextColor, TextInputTextFont};

use crate::gfx::FrameMaterial;
use crate::manager::AtlasManager;
use crate::system::ui_effects::{UiFxTrackedColor, UiFxTrackedSize};

#[derive(Resource)]
pub(crate) struct ScreenLayoutManager {
    layout_map: HashMap<String, ScreenLayout>,
    base_entity_map: HashMap<String, Entity>,
    ui_entity_map: HashMap<String, Entity>,
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
    has_ui: bool,
}

enum Element {
    Layout(LayoutElement),
    Shape(ShapeElement),
    Sprite(SpriteElement),
    Text(TextElement),
    UiInputBox(UiInputBoxElement),
}

#[derive(Debug)]
enum ShapeKind {
    CapsuleX,
    Circle,
    DashFrame,
    Frame,
    Rect,
    Region,
}

#[derive(Debug)]
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
    size: Vec2,
    color: Srgba,
}

struct TextElement {
    default_text: String,
    color: Srgba,
    font_name: String,
    font_size: f32,
    position: Vec3,
    size: Vec2,
    justify: Justify,
}

struct LayoutElement {
    layout: String,
    position: Vec3,
    size: Vec2,
}

struct UiInputBoxElement {
    text_element: TextElement,
    password: bool,
}

#[derive(SystemParam)]
pub(crate) struct ScreenLayoutManagerParams<'w> {
    assets: Res<'w, AssetServer>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials_color: ResMut<'w, Assets<ColorMaterial>>,
    materials_frame: ResMut<'w, Assets<FrameMaterial>>,
}

impl ScreenLayout {
    fn parse_size(rect: &str) -> Result<Vec2, String> {
        let mut components = rect.split(",");

        let w = components.next().and_then(|width| width.parse::<f32>().ok()).ok_or_else(|| format!("size: {rect}"))?;
        let h = components.next().and_then(|height| height.parse::<f32>().ok()).ok_or_else(|| format!("size: {rect}"))?;

        Ok(Vec2::new(w, h))
    }

    fn parse_position(position: &str) -> Result<Vec3, String> {
        const Z_FACTOR: f32 = 0.01;
        let mut components = position.split(",");

        let x = components.next().and_then(|c| c.parse::<f32>().ok()).ok_or_else(|| format!("position: {position}"))?;
        let y = components.next().and_then(|c| c.parse::<f32>().ok()).ok_or_else(|| format!("position: {position}"))?;
        let z = components.next().and_then(|c| c.parse::<f32>().ok()).ok_or_else(|| format!("position: {position}"))?;

        Ok(Vec3::new(x, -y, z * Z_FACTOR))
    }

    fn parse_shape_kind(shape_kind: &str) -> Result<ShapeKind, String> {
        match shape_kind {
            "capsule_x" => Ok(ShapeKind::CapsuleX),
            "circle" => Ok(ShapeKind::Circle),
            "dash_frame" => Ok(ShapeKind::DashFrame),
            "frame" => Ok(ShapeKind::Frame),
            "rect" => Ok(ShapeKind::Rect),
            "region" => Ok(ShapeKind::Region),
            _ => Err(format!("shape_kind: {shape_kind}")),
        }
    }

    fn parse_shape<'a>(&mut self, name: &'a str, remain: &str, resources: &ScreenResources, overrides: &ScreenResources) -> Result<Option<(&'a str, Element)>, String> {
        let (shape_kind_str, remain) = remain.split_once("%").ok_or_else(|| format!("shape: {remain}"))?;
        let (size_str, remain) = remain.split_once("@").ok_or_else(|| format!("shape: {remain}"))?;
        let (position_str, color_str) = remain.split_once("!").ok_or_else(|| format!("shape: {remain}"))?;

        let raw_position = Self::parse_position(position_str)?;
        let size = Self::parse_size(size_str)?;

        let position = Vec3::new(raw_position.x + (size.x / 2.0), raw_position.y - (size.y / 2.0), raw_position.z);

        let element = ShapeElement {
            kind: Self::parse_shape_kind(shape_kind_str)?,
            position,
            size,
            color: *overrides.color_map.get(color_str).or(resources.color_map.get(color_str)).ok_or_else(|| format!("shape: {remain}"))?,
        };

        Ok(Some((name, Element::Shape(element))))
    }

    fn parse_sprite<'a>(&mut self, name: &'a str, remain: &str, resources: &ScreenResources, overrides: &ScreenResources) -> Result<Option<(&'a str, Element)>, String> {
        let (atlas_item, remain) = remain.split_once('%').ok_or_else(|| format!("sprite: {remain}"))?;
        let (size_str, remain) = remain.split_once("@").ok_or_else(|| format!("sprite: {remain}"))?;
        let (position, color) = remain.split_once("!").ok_or_else(|| format!("sprite: {remain}"))?;
        let (atlas, item) = atlas_item.split_once('.').ok_or_else(|| format!("sprite: {atlas_item}"))?;

        let element = SpriteElement {
            atlas: atlas.to_string(),
            item: item.to_string(),
            position: Self::parse_position(position)?,
            size: Self::parse_size(size_str)?,
            color: *overrides.color_map.get(color).or(resources.color_map.get(color)).ok_or_else(|| format!("sprite: {color}"))?,
        };

        Ok(Some((name, Element::Sprite(element))))
    }

    fn parse_font_info(font_info: &str, resources: &ScreenResources, overrides: &ScreenResources) -> Result<(String, f32, Justify), String> {
        let mut parts = font_info.split(".");
        let name_str = parts.next().ok_or_else(|| format!("font_info: {font_info}"))?;
        let font_size = parts.next().and_then(|p| p.parse::<f32>().ok()).ok_or_else(|| format!("font_info: {font_info}"))?;
        let justify_str = parts.next().ok_or_else(|| format!("font_info: {font_info}"))?;

        let font_name = overrides.font_map.get(name_str).or(resources.font_map.get(name_str)).ok_or_else(|| format!("font_info: {font_info}"))?.clone();
        let justify = match justify_str {
            "left" => Justify::Left,
            "center" => Justify::Center,
            "right" => Justify::Right,
            _ => Justify::Center,
        };
        Ok((font_name, font_size, justify))
    }

    fn parse_text_internal(remain: &str, resources: &ScreenResources, overrides: &ScreenResources) -> Result<TextElement, String> {
        let (font_info, remain) = remain.split_once('%').ok_or_else(|| format!("text: {remain}"))?;
        let (size_str, remain) = remain.split_once('@').ok_or_else(|| format!("text: {remain}"))?;
        let (position_str, remain) = remain.split_once('!').ok_or_else(|| format!("text: {remain}"))?;
        let (color, default_text) = remain.split_once('|').ok_or_else(|| format!("text: {remain}"))?;

        let size = Self::parse_size(size_str)?;
        let position = Self::parse_position(position_str)?;
        let (font_name, font_size, justify) = Self::parse_font_info(font_info, resources, overrides)?;

        let element = TextElement {
            default_text: default_text.to_string(),
            color: *overrides.color_map.get(color).or(resources.color_map.get(color)).ok_or_else(|| format!("text: {color}"))?,
            font_name,
            font_size,
            position,
            size,
            justify,
        };
        Ok(element)
    }
    fn parse_text<'a>(&mut self, name: &'a str, remain: &str, resources: &ScreenResources, overrides: &ScreenResources) -> Result<Option<(&'a str, Element)>, String> {
        let element = Self::parse_text_internal(remain, resources, overrides)?;
        Ok(Some((name, Element::Text(element))))
    }

    fn parse_layout<'a>(&mut self, name: &'a str, remain: &str) -> Result<Option<(&'a str, Element)>, String> {
        let (layout, remain) = remain.split_once('%').ok_or_else(|| format!("layout: {remain}"))?;
        let (size_str, position_str) = remain.split_once('@').ok_or_else(|| format!("layout: {remain}"))?;

        let size = Self::parse_size(size_str)?;
        let position = Self::parse_position(position_str)?;

        let element = LayoutElement {
            layout: layout.to_string(),
            position,
            size,
        };

        Ok(Some((name, Element::Layout(element))))
    }
    fn parse_ui_input<'a>(&mut self, name: &'a str, remain: &str, resources: &ScreenResources, overrides: &ScreenResources) -> Result<Option<(&'a str, Element)>, String> {
        let text_element = Self::parse_text_internal(remain, resources, overrides)?;
        let mut chars = name.chars();
        let password = chars.next().ok_or_else(|| format!("ui_input: {name}"))? == '?';
        let new_name = if password {
            &name[1..]
        } else {
            name
        };

        self.has_ui = true;

        let element = UiInputBoxElement {
            text_element,
            password,
        };

        Ok(Some((new_name, Element::UiInputBox(element))))
    }

    pub(crate) fn entity_option(&self, name: &str) -> Option<&Entity> {
        self.entity_map.get(name)
    }
    pub(crate) fn entity(&self, name: &str) -> Entity {
        *self.entity_map.get(name).unwrap_or_else(|| {
            error!("Trying to access unknown entity: {name}");
            &Entity::PLACEHOLDER
        })
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
            base_entity_map: HashMap::new(),
            ui_entity_map: HashMap::new(),
        };
        Ok(screen_layout_manager)
    }

    fn load_init(path: impl AsRef<Path>) -> Result<ScreenResources, std::io::Error> {
        let init_file = std::fs::read_to_string(path)?;

        let base_resources = ScreenResources::default();
        //todo: pre-populate with CSS colors?

        let mut resources = ScreenResources::default();
        resources.color_map.insert("none".to_string(), Srgba::NONE);
        resources.color_map.insert("black".to_string(), Srgba::BLACK);
        resources.color_map.insert("white".to_string(), Srgba::WHITE);

        for (line_number, line) in init_file.lines().enumerate() {
            let result = Self::parse_kind(line) //
                .and_then(|(kind, remain)| Ok((kind, Self::parse_name(remain)?)))
                .and_then(|(kind, (name, remain))| match kind {
                    '$' => Self::parse_resource(name, remain, &base_resources, &mut resources),
                    _ => Err(format!("invalid kind in init file: {kind}")),
                });
            match result {
                Ok(_) => continue,
                Err(err) => error!("[_.self] Parse error on line {}: {err}", line_number + 1),
            }
        }

        Ok(resources)
    }

    fn load(layout_name: &str, path: impl AsRef<Path>, resources: &ScreenResources) -> Result<ScreenLayout, std::io::Error> {
        let layout_file = std::fs::read_to_string(path)?;

        let mut overrides = ScreenResources::default();

        let mut screen_layout = ScreenLayout::default();
        for (line_number, line) in layout_file.lines().enumerate() {
            let result = Self::parse_kind(line) //
                .and_then(|(kind, remain)| Ok((kind, Self::parse_name(remain)?)))
                .and_then(|(kind, (name, remain))| match kind {
                    '$' => Self::parse_resource(name, remain, resources, &mut overrides),
                    '*' => screen_layout.parse_sprite(name, remain, resources, &overrides),
                    '&' => screen_layout.parse_text(name, remain, resources, &overrides),
                    '#' => screen_layout.parse_shape(name, remain, resources, &overrides),
                    '?' => screen_layout.parse_ui_input(name, remain, resources, &overrides),
                    '/' => screen_layout.parse_layout(name, remain),
                    _ => Err(format!("kind: {kind}")),
                });

            match result {
                Ok(Some((name, element))) => {
                    if screen_layout.element_map.insert(name.to_string(), element).is_some() {
                        warn!("[{layout_name}.self] Duplicate element on line {}: {name}", line_number + 1);
                    }
                }
                Ok(None) => continue,
                Err(err) => error!("[{layout_name}.self] Parse error on line {}: {err}", line_number + 1),
            }
        }

        Ok(screen_layout)
    }

    fn parse_name(remain: &str) -> Result<(&str, &str), String> {
        remain.split_once(':').ok_or_else(|| format!("name: {remain}"))
    }

    fn parse_kind(line: &str) -> Result<(char, &str), String> {
        let mut chars = line.chars();
        let (kind, remain) = (chars.next().ok_or_else(|| format!("kind: {line}"))?, chars.as_str());
        Ok((kind, remain))
    }

    fn parse_color_components(remain: &str) -> Option<Srgba> {
        let mut components = remain.split(',');
        let r = components.next()?.parse::<f32>().ok()?;
        let g = components.next()?.parse::<f32>().ok()?;
        let b = components.next()?.parse::<f32>().ok()?;
        let a = components.next()?.parse::<f32>().ok()?;

        Some(Srgba::new(r, g, b, a))
    }

    fn parse_color<'a>(remain: &'a str, in_resources: &ScreenResources, out_colors: &mut HashMap<String, Srgba>) -> Result<Option<(&'a str, Element)>, String> {
        let (name, remain) = remain.split_once('=').ok_or_else(|| format!("color: {remain}"))?;

        let color = in_resources.color_map.get(remain).cloned().or_else(|| Self::parse_color_components(remain)).ok_or_else(|| format!("color: {remain}"))?;

        out_colors.insert(name.to_string(), color);
        Ok(None)
    }

    fn parse_font<'a>(remain: &'a str, fonts: &mut HashMap<String, String>) -> Result<Option<(&'a str, Element)>, String> {
        let (name, remain) = remain.split_once('=').ok_or_else(|| format!("font: {remain}"))?;

        fonts.insert(name.to_string(), remain.to_string());

        Ok(None)
    }

    fn parse_resource<'a>(name: &'a str, remain: &'a str, in_resources: &ScreenResources, out_resources: &mut ScreenResources) -> Result<Option<(&'a str, Element)>, String> {
        match name {
            "color" => Self::parse_color(remain, in_resources, &mut out_resources.color_map),
            "font" => Self::parse_font(remain, &mut out_resources.font_map),
            _ => Err(format!("resource: {name}")),
        }
    }

    fn spawn_shape_bundle_frame(parent: &mut ChildSpawnerCommands, element: &ShapeElement, dash_size: f32, meshes: &mut Assets<Mesh>, materials: &mut Assets<FrameMaterial>) -> Entity {
        let mesh = Mesh2d(meshes.add(Rectangle::new(element.size.x, element.size.y)));
        let material = MeshMaterial2d(materials.add(FrameMaterial::new(LinearRgba::from(element.color), element.size, dash_size)));
        let transform = Transform::from_translation(element.position);

        parent.spawn((mesh, material, transform, UiFxTrackedColor::from(element.color), UiFxTrackedSize::from(element.size), Pickable::IGNORE)).id()
    }

    fn spawn_shape_bundle_region(parent: &mut ChildSpawnerCommands, element: &ShapeElement, meshes: &mut Assets<Mesh>) -> Entity {
        let mesh = Mesh2d(meshes.add(Rectangle::new(element.size.x, element.size.y)));
        let transform = Transform::from_translation(element.position);

        parent.spawn((mesh, transform, UiFxTrackedColor::from(element.color), UiFxTrackedSize::from(element.size), Pickable::default())).id()
    }

    fn spawn_shape_bundle_rect(parent: &mut ChildSpawnerCommands, element: &ShapeElement, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> Entity {
        let mesh = Mesh2d(meshes.add(Rectangle::new(element.size.x, element.size.y)));
        let material = MeshMaterial2d(materials.add(Color::Srgba(element.color)));
        let transform = Transform::from_translation(element.position);

        parent.spawn((mesh, material, transform, UiFxTrackedColor::from(element.color), UiFxTrackedSize::from(element.size), Pickable::IGNORE)).id()
    }

    fn spawn_shape_bundle_capsule_x(parent: &mut ChildSpawnerCommands, element: &ShapeElement, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> Entity {
        let mesh = Mesh2d(meshes.add(Capsule2d::new(element.size.y / 2.0, element.size.x - element.size.y)));
        let material = MeshMaterial2d(materials.add(Color::Srgba(element.color)));
        let rotation = Quat::from_rotation_z(PI / 2.0);

        let transform = Transform {
            translation: element.position,
            rotation,
            scale: Vec3::ONE,
        };

        parent.spawn((mesh, material, transform, UiFxTrackedColor::from(element.color), UiFxTrackedSize::from(element.size), Pickable::IGNORE)).id()
    }

    fn spawn_shape_bundle_circle(parent: &mut ChildSpawnerCommands, element: &ShapeElement, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) -> Entity {
        let mesh = Mesh2d(meshes.add(Circle::new(element.size.x / 2.0)));
        let material = MeshMaterial2d(materials.add(Color::Srgba(element.color)));
        let transform = Transform::from_translation(element.position);

        parent.spawn((mesh, material, transform, UiFxTrackedColor::from(element.color), UiFxTrackedSize::from(element.size), Pickable::IGNORE)).id()
    }

    fn spawn_sprite_bundle(parent: &mut ChildSpawnerCommands, element: &SpriteElement, am: &AtlasManager) -> Option<Entity> {
        let (atlas, image) = am.get_atlas_texture(&element.atlas, &element.item)?;

        let sprite = (
            Sprite {
                color: Color::Srgba(element.color),
                image,
                texture_atlas: Some(atlas),
                ..default()
            },
            Anchor::TOP_LEFT,
            Transform::from_translation(element.position),
        );
        let sprite = parent.spawn((sprite, UiFxTrackedColor::from(element.color), UiFxTrackedSize::from(element.size), Pickable::IGNORE)).id();
        Some(sprite)
    }

    fn spawn_text_bundle(parent: &mut ChildSpawnerCommands, element: &TextElement, asset_server: &AssetServer) -> Entity {
        let font = asset_server.load(&element.font_name);
        let translation = Vec3::new(element.position.x, element.position.y - (element.size.y / 2.0), element.position.z);

        let bundle = (
            //
            Text2d::new(&element.default_text),
            TextBounds::from(element.size),
            TextColor::from(element.color),
            TextFont::from(font).with_font_size(element.font_size),
            TextLayout::new_with_justify(element.justify),
            Transform::from_translation(translation),
            Anchor::CENTER_LEFT,
        );
        parent.spawn(bundle).id()
    }

    fn make_container_bundle(element: &LayoutElement) -> impl Bundle + use<> {
        (
            Sprite {
                color: Color::NONE,
                custom_size: Some(element.size),
                ..default()
            },
            Anchor::TOP_LEFT,
            Transform::from_translation(element.position),
            UiFxTrackedSize::from(element.size),
            Pickable::IGNORE,
        )
    }

    fn spawn_input_bundle(parent: &mut ChildSpawnerCommands, element: &UiInputBoxElement, asset_server: &AssetServer) -> Entity {
        let font = asset_server.load(&element.text_element.font_name);
        let bundle = (
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(element.text_element.position.x),
                top: Val::Px(-element.text_element.position.y),
                width: Val::Px(element.text_element.size.x),
                height: Val::Px(element.text_element.size.y),
                ..default()
            },
            // Prevent clicks on the input from also bubbling down to the container
            // behind it
            FocusPolicy::Block,
            TextInput,
            TextInputTextFont(TextFont::from(font).with_font_size(element.text_element.font_size)),
            TextInputTextColor(TextColor(Color::WHITE)),
            TextInputInactive(true),
            TextInputSettings {
                retain_on_submit: true,
                mask_character: element.password.then_some('*'),
                max_length: None,
            },
        );
        parent.spawn(bundle).id()
    }

    fn make_name(base_name: &str, name: &str) -> String {
        if base_name.is_empty() {
            name.to_string()
        } else {
            format!("{base_name}/{name}")
        }
    }

    fn build_layout(&self, layout_name: &str, parent: &mut ChildSpawnerCommands, base_name: &str, am: &AtlasManager, slm_params: &mut ScreenLayoutManagerParams) -> Vec<(String, Entity)> {
        let mut entities = Vec::new();
        let layout = self.layout_map.get(layout_name).unwrap();

        for (name, element) in &layout.element_map {
            let full_name = Self::make_name(base_name, name);
            let id = match element {
                Element::Shape(e) => match e.kind {
                    ShapeKind::CapsuleX => Self::spawn_shape_bundle_capsule_x(parent, e, &mut slm_params.meshes, &mut slm_params.materials_color),
                    ShapeKind::Circle => Self::spawn_shape_bundle_circle(parent, e, &mut slm_params.meshes, &mut slm_params.materials_color),
                    ShapeKind::DashFrame => Self::spawn_shape_bundle_frame(parent, e, 8.0, &mut slm_params.meshes, &mut slm_params.materials_frame),
                    ShapeKind::Frame => Self::spawn_shape_bundle_frame(parent, e, 0.0, &mut slm_params.meshes, &mut slm_params.materials_frame),
                    ShapeKind::Rect => Self::spawn_shape_bundle_rect(parent, e, &mut slm_params.meshes, &mut slm_params.materials_color),
                    ShapeKind::Region => Self::spawn_shape_bundle_region(parent, e, &mut slm_params.meshes),
                },
                Element::Sprite(e) => {
                    if let Some(sprite) = Self::spawn_sprite_bundle(parent, e, am) {
                        sprite
                    } else {
                        continue;
                    }
                }
                Element::Text(e) => Self::spawn_text_bundle(parent, e, &slm_params.assets),
                Element::Layout(e) => {
                    let container = Self::make_container_bundle(e);
                    parent
                        .spawn(container)
                        .with_children(|parent| {
                            let nested_entities = self.build_layout(&e.layout, parent, &full_name, am, slm_params);
                            entities.extend(nested_entities);
                        })
                        .id()
                }
                Element::UiInputBox(_) => continue,
            };
            entities.push((full_name, id));
        }

        entities
    }
    fn build_ui_layer(&self, layout_name: &str, parent: &mut ChildSpawnerCommands, asset_server: &AssetServer) -> Vec<(String, Entity)> {
        let mut entities = Vec::new();
        let layout = self.layout_map.get(layout_name).unwrap();
        for (name, element) in &layout.element_map {
            let id = match element {
                Element::UiInputBox(e) => Self::spawn_input_bundle(parent, e, asset_server),
                Element::Shape(_) => continue,
                Element::Sprite(_) => continue,
                Element::Text(_) => continue,
                Element::Layout(_) => continue,
            };
            entities.push((name.clone(), id));
        }
        entities
    }

    pub(crate) fn build(&mut self, commands: &mut Commands, layout_name: &str, am: &AtlasManager, slm_params: &mut ScreenLayoutManagerParams, observe: Option<fn(&mut ChildSpawnerCommands)>) -> &ScreenLayout {
        let base_id = commands
            .spawn(ScreenLayoutContainer::default())
            .with_children(|parent| {
                let entities = self.build_layout(layout_name, parent, "", am, slm_params);
                if let Some(layout) = self.layout_map.get_mut(layout_name) {
                    layout.entity_map = entities.into_iter().collect();
                }
            })
            .id();
        self.base_entity_map.insert(layout_name.to_string(), base_id);

        let layout = self.layout_map.get(layout_name).unwrap();
        if layout.has_ui {
            let ui_id = commands
                .spawn(Self::ui_base())
                .with_children(|parent| {
                    let entities = self.build_ui_layer(layout_name, parent, &slm_params.assets);
                    if let Some(layout) = self.layout_map.get_mut(layout_name) {
                        layout.entity_map.extend(entities);
                    }
                })
                .id();
            self.ui_entity_map.insert(layout_name.to_string(), ui_id);
        }

        if let Some(observe_fn) = observe {
            commands.entity(base_id).with_children(observe_fn);
        }

        self.layout_map.get(layout_name).unwrap()
    }

    pub(crate) fn destroy(&mut self, mut commands: Commands, layout_name: &str) {
        if let Some(entity) = self.base_entity_map.remove(layout_name) {
            commands.entity(entity).despawn();
        }
        if let Some(entity) = self.ui_entity_map.remove(layout_name) {
            commands.entity(entity).despawn();
        }
    }

    fn ui_base() -> impl Bundle {
        (
            Node {
                display: Display::Block,
                position_type: PositionType::Absolute,
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            Pickable::IGNORE,
        )
    }
}
