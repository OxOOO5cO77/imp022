use std::fmt::Display;
use std::io::Error;
use std::path::Path;

pub(crate) mod hall;
pub(crate) mod vagabond;

pub(crate) fn save_data_single<T, P>(data: T, dest_file: P) -> Result<(), Error>
where
    T: serde::ser::Serialize,
    P: AsRef<Path> + Display,
{
    println!("[Smithy] > {dest_file}");

    let ron = ron::to_string::<T>(&data).map_err(Error::other)?;
    std::fs::write(dest_file, ron)?;

    Ok(())
}
