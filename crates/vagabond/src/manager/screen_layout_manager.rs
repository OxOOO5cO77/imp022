use crate::manager::AtlasManager;
use crate::system::ui::Screen;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::Text2dBounds;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Resource)]
pub(crate) struct ScreenLayoutManager {
    layout_map: HashMap<String, ScreenLayout>,
    entity_map: HashMap<String, Entity>,
}

#[derive(Eq, Hash, PartialEq)]
pub(crate) enum EntityKind {
    Sprite,
    Text,
}

#[derive(Default)]
pub(crate) struct ScreenLayout {
    sprite_map: HashMap<String, SpriteElement>,
    text_map: HashMap<String, TextElement>,
    layout_map: HashMap<String, LayoutElement>,
    entity_map: HashMap<(EntityKind, String), Entity>,
}

struct SpriteElement {
    atlas: String,
    item: String,
    position: Vec2,
    color: Srgba,
}

struct TextElement {
    default_text: String,
    color: Srgba,
    font_size: f32,
    position: Vec2,
    size: Vec2,
}

struct LayoutElement {
    layout: String,
    position: Vec2,
}

impl ScreenLayout {
    fn parse_color(color: &str) -> Srgba {
        match color {
            "grey" => bevy::color::palettes::css::DARK_GRAY,
            "yellow" => bevy::color::palettes::css::DARK_GOLDENROD,
            "red" => bevy::color::palettes::css::DARK_RED,
            "green" => bevy::color::palettes::css::DARK_GREEN,
            "purple" => bevy::color::palettes::css::DARK_BLUE,
            "white" => bevy::color::palettes::css::WHITE,
            _ => Srgba::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    fn parse_position_size(rect: &str) -> (Vec2, Vec2) {
        let [x_str, y_str, w_str, h_str] = rect.split(",").collect::<Vec<&str>>().try_into().unwrap();

        let x = x_str.parse::<u32>().unwrap_or_default() as f32;
        let y = y_str.parse::<u32>().unwrap_or_default() as f32;
        let w = w_str.parse::<u32>().unwrap_or_default() as f32;
        let h = h_str.parse::<u32>().unwrap_or_default() as f32;

        (Vec2::new(x, y), Vec2::new(w, h))
    }

    fn parse_position(position: &str) -> Vec2 {
        let (x, y) = position.split_once(",").unwrap();
        Vec2 {
            x: x.parse().unwrap_or_default(),
            y: y.parse().unwrap_or_default(),
        }
    }

    fn parse_sprite(&mut self, name: &str, remain: &str) {
        let (atlas_item, remain) = remain.split_once('@').unwrap();
        let (atlas, item) = atlas_item.split_once('.').unwrap();
        let (position, color) = remain.split_once("!").unwrap();

        let element = SpriteElement {
            atlas: atlas.to_string(),
            item: item.to_string(),
            position: Self::parse_position(position),
            color: Self::parse_color(color),
        };

        self.sprite_map.insert(name.to_string(), element);
    }
    fn parse_text(&mut self, name: &str, remain: &str) {
        let (default_text, remain) = remain.split_once('@').unwrap();
        let (position_size, remain) = remain.split_once('!').unwrap();
        let (color, font_size) = remain.split_once(',').unwrap();

        let (position, size) = Self::parse_position_size(position_size);

        let element = TextElement {
            default_text: default_text.to_string(),
            color: Self::parse_color(color),
            font_size: font_size.parse().unwrap(),
            position,
            size,
        };
        self.text_map.insert(name.to_string(), element);
    }
    fn parse_layout(&mut self, name: &str, remain: &str) {
        let (layout, remain) = remain.split_once('@').unwrap();
        let element = LayoutElement {
            layout: layout.to_string(),
            position: Self::parse_position(remain),
        };
        self.layout_map.insert(name.to_string(), element);
    }

    fn decorate(&self, commands: &mut Commands, kind: EntityKind, name: &str, decorator: impl Bundle) {
        let key = (kind, name.to_string());
        if let Some(entity) = self.entity_map.get(&key) {
            commands.entity(*entity).insert(decorator);
        }
    }

    fn sprite(&self, commands: &mut Commands, name: &str, decorator: impl Bundle) {
        self.decorate(commands, EntityKind::Sprite, name, decorator);
    }
    pub(crate) fn decorate_text(&self, commands: &mut Commands, name: &str, decorator: impl Bundle) {
        self.decorate(commands, EntityKind::Text, name, decorator)
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
        let layout_file = fs::read_to_string(path)?;

        let mut screen_layout = ScreenLayout::default();

        for line in layout_file.lines() {
            let mut chars = line.chars();
            let (kind, remain) = (chars.next().unwrap(), chars.collect::<String>());
            let (name, remain) = remain.split_once(':').unwrap();
            match kind {
                '*' => screen_layout.parse_sprite(name, remain),
                '+' => screen_layout.parse_layout(name, remain),
                '&' => screen_layout.parse_text(name, remain),
                _ => continue,
            }
        }

        Ok(screen_layout)
    }

    fn spawn_text_bundle(element: &TextElement, offset: Vec2, font: Handle<Font>) -> impl Bundle {
        let position = element.position + offset;
        Text2dBundle {
            text: Text::from_section(
                element.default_text.clone(),
                TextStyle {
                    font,
                    font_size: element.font_size,
                    color: Color::Srgba(element.color),
                },
            )
            .with_justify(JustifyText::Center),
            text_anchor: Anchor::TopLeft,
            text_2d_bounds: Text2dBounds {
                size: element.size,
            },
            transform: Transform::from_xyz(position.x, -position.y, 0.0),
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

    fn build_layout(&self, layout_name: &str, parent: &mut ChildBuilder, offset: Vec2, base_name: &str, font_handle: Handle<Font>, am: &Res<AtlasManager>) -> Vec<((EntityKind, String), Entity)> {
        let mut entities = Vec::new();
        let layout = self.layout_map.get(layout_name).unwrap();
        for (name, element) in &layout.sprite_map {
            let full_name = Self::make_name(base_name, name);
            let sprite = am.sprite(element.atlas.as_str(), element.item.as_str(), element.position + offset, element.color).unwrap();
            let id = parent.spawn(sprite).id();
            entities.push(((EntityKind::Sprite, full_name), id));
        }
        for (name, element) in &layout.text_map {
            let full_name = Self::make_name(base_name, name);
            let text = Self::spawn_text_bundle(element, offset, font_handle.clone());
            let id = parent.spawn(text).id();
            entities.push(((EntityKind::Text, full_name), id));
        }
        for (name, element) in &layout.layout_map {
            let full_name = Self::make_name(base_name, name);
            let mut nested_entities = self.build_layout(&element.layout, parent, element.position, &full_name, font_handle.clone(), am);
            entities.append(&mut nested_entities);
        }
        entities
    }

    pub(crate) fn build(&mut self, commands: &mut Commands, layout_name: &str, font_handle: Handle<Font>, am: Res<AtlasManager>) -> &ScreenLayout {
        let id = commands
            .spawn((ScreenLayoutContainer::default(), Screen))
            .with_children(|parent| {
                let entities = self.build_layout(layout_name, parent, Vec2::default(), "", font_handle, &am);
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
