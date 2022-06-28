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

    #[test]
    fn test_home_trash_dir_path_with_xdg_data_home_not_set() {
        env::remove_var("XDG_DATA_HOME");
        let mut trash_dir = home_dir().unwrap();
        trash_dir.push(data_local_dir().unwrap());
        trash_dir.push("Trash");

        let trash_dir_result = home_trash_dir_path();
        assert_eq!(trash_dir, trash_dir_result);
    }

    #[test]
    fn test_home_trash_dir_path_from_env_if_xdg_data_home_set() {
        let mut xdg_data_home = home_dir().unwrap();
        xdg_data_home.push(".local");
        xdg_data_home.push("share");
        env::set_var("XDG_DATA_HOME", xdg_data_home.clone());
        let trash_dir = home_trash_dir_path_from_env();
        assert_eq!(trash_dir.unwrap(), PathBuf::from(xdg_data_home.clone()).join("Trash"));
    }

    #[test]
    fn test_home_trash_dir_path_from_env_if_xdg_data_home_not_set() {
        env::remove_var("XDG_DATA_HOME");
        let trash_dir = home_trash_dir_path_from_env();
        assert!(trash_dir.is_err());
    }
}