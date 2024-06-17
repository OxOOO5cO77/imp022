use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::path::Path;

use walkdir::WalkDir;

pub(crate) mod hall;
pub(crate) mod vagabond;

pub(crate) fn save_data_single<T, P>(data: T, dest_file: P) -> Result<(), Error>
where
    T: serde::ser::Serialize,
    P: AsRef<Path> + Display,
{
    println!("[Smithy] > {}", dest_file);

    let ron = ron::to_string::<T>(&data).map_err(|o| Error::new(ErrorKind::Other, o))?;
    std::fs::write(dest_file, ron)?;

    Ok(())
}

fn load_data_single<T, P>(source_file: P) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let ron = std::fs::read_to_string(source_file)?;
    let parsed = ron::from_str::<T>(&ron).map_err(|o| Error::new(ErrorKind::Other, o))?;
    Ok(parsed)
}

pub(crate) fn load_data<T, P>(source_dir: P) -> Result<Vec<T>, Error>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let mut result = Vec::new();
    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
    {
        let path = entry.path();
        println!("[Smithy] < {}", path.display());
        let loaded = load_data_single(path)?;
        result.push(loaded);
    }

    Ok(result)
}
