use std::{fs::File, io::Read, path::PathBuf};

pub fn get_theme_toml() -> std::io::Result<String> {
    let config = xdg::BaseDirectories::with_prefix("oxiced");
    let theme_path = config.find_config_file("theme.toml");
    if theme_path.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
    }
    open_file(theme_path.unwrap())
}

pub fn open_file(path: PathBuf) -> std::io::Result<String> {
    let mut theme_file = File::open(path)?;
    let mut theme_string = String::from("");
    theme_file.read_to_string(&mut theme_string)?;
    Ok(theme_string)
}
