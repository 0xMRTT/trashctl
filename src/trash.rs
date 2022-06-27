use std::env;
use std::path::PathBuf;
pub fn home_trash_dir_path_from_env() -> PathBuf {
    if env::var("XDG_DATA_HOME").is_ok() {
        let mut trash_dir = PathBuf::from(env::var("XDG_DATA_HOME").unwrap());
        trash_dir.push("Trash");
        trash_dir
    } else {
        let home_dir = env::home_dir().unwrap();
        let mut trash_dir = home_dir.clone();
        trash_dir.push(".local/share/Trash");
        trash_dir
    }
}