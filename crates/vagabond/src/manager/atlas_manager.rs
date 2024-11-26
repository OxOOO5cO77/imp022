use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::path::Path;

#[derive(Resource, Default)]
pub(crate) struct AtlasManager {
    map: HashMap<String, Atlas>,
}

pub(crate) enum AtlasManagerError {
    Io(std::io::Error),
    ContentEmpty,
    MissingColon(usize),
    ParseSizeCount(usize),
    ParseSizeContent((usize, ParseIntError)),
}

impl std::fmt::Debug for AtlasManagerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AtlasManagerError::Io(err) => write!(f, "AtlasManagerError::Io: {}", err),
            AtlasManagerError::ContentEmpty => write!(f, "AtlasManagerError::ContentEmpty"),
            AtlasManagerError::MissingColon(line) => write!(f, "AtlasManagerError::MissingColon: Line {}", line),
            AtlasManagerError::ParseSizeCount(line) => write!(f, "AtlasManagerError::ParseSizeCount: Line {}", line),
            AtlasManagerError::ParseSizeContent((line, err)) => write!(f, "AtlasManagerError::ParseSizeContent: Line {}: {}", line, err),
        }
    }
}

impl AtlasManager {
    pub(crate) fn load_atlas(&mut self, path: impl AsRef<Path>, asset_server: &Res<AssetServer>, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>) -> Result<(), AtlasManagerError> {
        let (name, atlas) = Atlas::new(path, asset_server, texture_atlas_layouts)?;
        self.map.insert(name, atlas);
        Ok(())
    }

    pub(crate) fn make_sprite_bundle(&self, atlas_name: &str, texture_name: &str, translation: Vec3, color: Srgba) -> Option<(SpriteBundle, TextureAtlas)> {
        let atlas = self.map.get(atlas_name)?;
        let entry = atlas.map.get(texture_name)?;
        let texture = atlas.image.clone();
        let layout = atlas.layout.clone();

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
        let atlas = TextureAtlas {
            layout,
            index: entry.index,
        };
        Some((sprite, atlas))
    }
}

struct Atlas {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    map: HashMap<String, AtlasEntry>,
}

struct AtlasEntry {
    index: usize,
}

fn parse_first_line(line: &str) -> Result<(String, UVec2), AtlasManagerError> {
    let (name, line_size) = line.split_once(':').ok_or(AtlasManagerError::MissingColon(1))?;
    let (line_x, line_y) = line_size.split_once(',').ok_or(AtlasManagerError::ParseSizeCount(1))?;
    let size = UVec2 {
        x: line_x.parse().map_err(|err| AtlasManagerError::ParseSizeContent((1, err)))?,
        y: line_y.parse().map_err(|err| AtlasManagerError::ParseSizeContent((1, err)))?,
    };
    Ok((name.to_string(), size))
}

fn parse_segment(line_number: usize, segment: Option<&str>) -> Result<u32, AtlasManagerError> {
    let result = segment.ok_or(AtlasManagerError::ParseSizeCount(line_number))?.parse::<u32>().map_err(|err| AtlasManagerError::ParseSizeContent((1, err)))?;
    Ok(result)
}

fn parse_line(line_number: usize, line: &str) -> Result<(String, URect), AtlasManagerError> {
    let (name, size_line) = line.split_once(':').ok_or(AtlasManagerError::MissingColon(line_number))?;
    let mut split = size_line.split(',');
    let min_x = parse_segment(line_number, split.next())?;
    let min_y = parse_segment(line_number, split.next())?;
    let size_x = parse_segment(line_number, split.next())?;
    let size_y = parse_segment(line_number, split.next())?;
    let min = UVec2 {
        x: min_x,
        y: min_y,
    };
    let max = UVec2 {
        x: min.x + size_x,
        y: min.y + size_y,
    };
    let size = URect {
        min,
        max,
    };
    Ok((name.to_string(), size))
}

fn parse_file(layout_file: &str) -> Result<(String, HashMap<String, AtlasEntry>, TextureAtlasLayout), AtlasManagerError> {
    let mut map = HashMap::new();

    let mut layout_lines = layout_file.lines();
    let (atlas_name, atlas_size) = parse_first_line(layout_lines.next().ok_or(AtlasManagerError::ContentEmpty)?)?;
    let mut layout = TextureAtlasLayout::new_empty(atlas_size);
    for (line_number, line) in layout_lines.enumerate() {
        let (name, size) = parse_line(line_number + 1, line)?;
        let entry = AtlasEntry {
            index: layout.add_texture(size),
        };
        map.insert(name, entry);
    }

    Ok((atlas_name, map, layout))
}

impl Atlas {
    pub(crate) fn new(path: impl AsRef<Path>, asset_server: &Res<AssetServer>, layouts: &mut ResMut<Assets<TextureAtlasLayout>>) -> Result<(String, Self), AtlasManagerError> {
        let image_path = path.as_ref().with_extension("png");
        let image_handle = asset_server.load(image_path);

        let layout_path = Path::new("assets").join(path.as_ref().with_extension("atlas"));
        let layout_file = std::fs::read_to_string(layout_path).map_err(AtlasManagerError::Io)?;

        let (atlas_name, map, layout) = parse_file(&layout_file)?;

        let layout_handle = layouts.add(layout);
        Ok((
            atlas_name,
            Self {
                image: image_handle,
                layout: layout_handle,
                map,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::manager::atlas_manager::{parse_file, AtlasManagerError};

    #[test]
    fn test_atlas_manager() -> Result<(), AtlasManagerError> {
        let layout_file = // test case
            "gameplay:1024,1024
064x064:2,2,64,64
236x312:734,56,236,312
364x200:368,168,364,200
364x300:2,68,364,300
998x540:2,370,998,540"
            ;
        let (atlas_name, map, _layout) = parse_file(layout_file)?;

        assert_eq!(atlas_name, "gameplay");
        assert_eq!(map.len(), 5);
        assert!(map.contains_key("998x540"));
        assert!(!map.contains_key("gameplay"));
        Ok(())
    }
}
