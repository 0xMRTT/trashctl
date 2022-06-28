use std::env;
use std::path::PathBuf;
use dirs::home_dir;
use dirs::data_local_dir;

pub fn home_trash_dir_path_from_env() -> Result<PathBuf, ()> {
    if env::var("XDG_DATA_HOME").is_ok() {
        let mut trash_dir = PathBuf::from(env::var("XDG_DATA_HOME").unwrap());
        trash_dir.push("Trash");
        return Ok(trash_dir);
    }

    return Err(());
}

pub fn home_trash_dir_path() -> PathBuf {
    let mut trash_dir = PathBuf::new();

    match home_trash_dir_path_from_env() {
        Ok(path) => {
            trash_dir = path;
        }
        Err(_) => {
            trash_dir.push(home_dir().unwrap());
            trash_dir.push(data_local_dir().unwrap());
            trash_dir.push("Trash");
        }
    }
    trash_dir
}

#[cfg(test)]
mod test {
    use super::*;
}