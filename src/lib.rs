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

use chrono::prelude::*;
use dirs::data_local_dir;
use dirs::home_dir;
use std::env;
use std::path::PathBuf;
extern crate fs_extra;
use fs_extra::dir::move_dir;
use fs_extra::file::move_file;
use fs_extra::file::CopyOptions as FileCopyOptions;
use fs_extra::dir::CopyOptions as DirCopyOptions;
use std::fs;

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

/// Returns the path to the trash directory inside a volume
/// The returned path is like `volume/.Trash/uid`.
/// There is a related function `volume_trash_dir_2` that returns the path to the trash directory inside a volume with the form `volume/.Trash-uid/`.
pub fn volume_trash_dir_1(volume: PathBuf, uid: u32) -> PathBuf {
    let mut trash_dir = PathBuf::new();
    trash_dir.push(volume);
    trash_dir.push(".Trash");
    trash_dir.push(format!("{}", uid));
    trash_dir
}

/// Returns the path to the trash directory inside a volume
/// The returned path is like `volume/.Trash-uid`.
/// There is a related function `volume_trash_dir_1` that returns the path to the trash directory inside a volume with the form `volume/.Trash/uid/`.
pub fn volume_trash_dir_2(volume: PathBuf, uid: u32) -> PathBuf {
    let mut trash_dir = PathBuf::new();
    trash_dir.push(volume);
    trash_dir.push(format!(".Trash-{}", uid));
    trash_dir
}

/// Represent a `.trashinfo` file. (in `Trash/info`)
/// The `.trashinfo` file contains the following information:
/// - The path to the file or directory before it was moved to trash.
/// - The time when the file or directory was moved to trash.
///
/// Example:
/// ```ini
/// [Trash Info]
/// Path=/home/user/test-need-to-go-to-trash-lol.md
/// DeletionDate=2022-06-28T09:40:03
/// ```
#[derive(Debug)]
pub struct TrashInfo {
    pub path: PathBuf,
    pub deletion_date: DateTime<Utc>,
}

 impl TrashInfo {
    /// Parse a date with the format `YYYY-MM-DDTHH:MM:SS`
    pub fn parse_date(date: String) -> Result<DateTime<Utc>, ()> {
        let date = Utc.datetime_from_str(&date, "%Y-%m-%dT%H:%M:%S");
        match date {
            Ok(date) => {
                return Ok(date);
            }
            Err(_) => {
                return Err(());
            }
        }
    }

    /// Create a new TrashInfo object from a path and a date.
    /// The date must be in the format `YYYY-MM-DDTHH:MM:SS`.
    /// If the date is not valid, an error is returned.
    pub fn new(path: PathBuf, deletion_date: String) -> TrashInfo {
        // example deletion_date : 2022-06-28T09:40:03
        let deletion_date = TrashInfo::parse_date(deletion_date).unwrap();
        TrashInfo {
            path,
            deletion_date,
        }
    }
}

#[derive(Debug)]
pub struct Trash {
    pub files: Vec<String>,
    pub info: Vec<TrashInfo>,
    pub path: PathBuf,
}

impl Trash {
    pub fn from(files: Vec<String>, info: Vec<TrashInfo>, path:PathBuf) -> Trash {
        Trash {
            files: files,
            info: info,
            path: path
        }
    }

    pub fn new(path:PathBuf) -> Trash {
        Trash {
            files: Vec::new(),
            info: Vec::new(),
            path:path
        }
    }

    pub fn add(&self, file:PathBuf) -> Result<(), &str> {
        if file.exists() {
            let mut to = self.path.clone();
            to.push("files"); // Trash/files/<filename>
            to.push(file.file_name().unwrap().to_str().unwrap().to_string());
            println!("TO : {:?}", to);
            if file.is_dir() {
                let options = DirCopyOptions::new(); //Initialize default values for CopyOptions
                move_dir(file, to, &options).unwrap();
            } else if file.is_file() {
                let options = FileCopyOptions::new();
                move_file(file, to, &options).unwrap();
            }
            return Ok(());
        } else {
            Err("File doesn't exists")
        }
    }

    pub fn auto_recon_trash(path:PathBuf) -> Trash {
        let mut files_path = path.clone();
        files_path.push("files");

        let mut info_path = path.clone();
        info_path.push("info");

        let mut infos:Vec<TrashInfo> = Vec::new();
        for info_file in fs::read_dir(info_path).unwrap() {
            //infos.push(TrashInfo::from_file(info_file.unwrap()));
        }

        let mut files:Vec<String> = Vec::new();
        for file in fs::read_dir(files_path).unwrap() {
            files.push(file.unwrap().file_name().to_str().unwrap().to_string());
        }

        Trash { files: files, info: infos, path:  path}
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// `home_trash_dir_path` returns the path to the trash directory in the user's home directory
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
        assert_eq!(
            trash_dir.unwrap(),
            PathBuf::from(xdg_data_home.clone()).join("Trash")
        );
    }

    #[test]
    fn test_home_trash_dir_path_from_env_if_xdg_data_home_not_set() {
        env::remove_var("XDG_DATA_HOME");
        let trash_dir = home_trash_dir_path_from_env();
        assert!(trash_dir.is_err());
    }

    #[test]
    fn test_volume_trash_dir_1() {
        let volume = PathBuf::from("/run/mount/user/volume");
        let uid = 123;
        let trash_dir = volume_trash_dir_1(volume, uid);
        assert_eq!(
            trash_dir,
            PathBuf::from("/run/mount/user/volume/.Trash/123")
        );
    }

    #[test]
    fn test_volume_trash_dir_2() {
        let volume = PathBuf::from("/run/mount/user/volume");
        let uid = 123;
        let trash_dir = volume_trash_dir_2(volume, uid);
        assert_eq!(
            trash_dir,
            PathBuf::from("/run/mount/user/volume/.Trash-123")
        );
    }

    #[test]
    fn test_new_trash_info() {
        let path = PathBuf::from("/home/user/test-need-to-go-to-trash-lol.md");
        let deletion_date = "2022-06-28T09:40:03";
        let trash_info = TrashInfo::new(path, deletion_date.to_string());
        assert_eq!(
            trash_info.path,
            PathBuf::from("/home/user/test-need-to-go-to-trash-lol.md")
        );
        assert_eq!(
            trash_info.deletion_date,
            Utc.datetime_from_str("2022-06-28T09:40:03", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
        );
    }

    #[test]
    fn test_parse_date() {
        let date = "2022-06-28T09:40:03";
        let date_result = TrashInfo::parse_date(date.to_string());
        assert_eq!(
            date_result.unwrap(),
            Utc.datetime_from_str("2022-06-28T09:40:03", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
        );
    }
}
