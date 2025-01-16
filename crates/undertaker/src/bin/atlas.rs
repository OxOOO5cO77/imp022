use std::io::ErrorKind;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    for entry in std::fs::read_dir("output/atlas")?
        .filter_map(|res| res.ok()) //
        .map(|dir_entry| dir_entry.path()) //
        .filter_map(|path| filter_extension(path, "atlas"))
    {
        let atlas = std::fs::read_to_string(&entry)?;
        let map = atlas_to_map(&atlas).ok_or(ErrorKind::Other)?;

        let mut output_filename = PathBuf::from("output/map");
        output_filename.push(entry.with_extension("map").file_name().unwrap());
        std::fs::create_dir_all("output/map")?;
        std::fs::write(output_filename, map.join("\n"))?;
    }

    Ok(())
}

fn filter_extension(path: PathBuf, extension: &str) -> Option<PathBuf> {
    path.extension().is_some_and(|ext| ext == extension).then_some(path)
}

fn atlas_to_map(atlas: &str) -> Option<Vec<String>> {
    let mut map = Vec::new();

    let mut lines = atlas.lines();

    let (name, _) = lines.next()?.split_once('.')?;
    let (_, size) = lines.next()?.split_once(':')?;
    map.push(format!("{name}:{size}"));

    lines.next()?; // repeat:none

    while let Some(line) = lines.next() {
        let image = line;
        let (_, bounds) = lines.next()?.split_once(':')?;
        map.push(format!("{image}:{bounds}"));
    }

    Some(map)
}
