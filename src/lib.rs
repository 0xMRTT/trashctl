//! A command line interface for trash
//! Copyright (C) 2022 0xMRTT <0xMRTT@tuta.io>
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//! 
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//! 
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! # `trashctl` lib
//! 
//! This module contains the library code for trashctl.
//! There is a lot of tools in this library, you can reuse them in your own projects.
//! 
 
use std::env;
use std::path::PathBuf;
use dirs::home_dir;
use dirs::data_local_dir;


/// Returns the path to the trash directory if XDG_DATA_HOME is set.
/// If not, returns an error. 
/// 
/// Do not use this function, use instead `home_trash_dir_path`.
pub fn home_trash_dir_path_from_env() -> Result<PathBuf, ()> {
    if env::var("XDG_DATA_HOME").is_ok() {
        let mut trash_dir = PathBuf::from(env::var("XDG_DATA_HOME").unwrap());
        trash_dir.push("Trash");
        return Ok(trash_dir);
    }

    return Err(());
}

/// Returns the path to the trash directory.
/// If XDG_DATA_HOME is set, return XDG_DATA_HOME/Trash.
/// If not, return ~/.local/share/Trash.
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