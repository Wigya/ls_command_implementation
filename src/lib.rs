pub mod utils;

use chrono::{Local, NaiveDateTime};
use nix::unistd::{Gid, Group, Uid, User};
use std::{
    fmt,
    fs::{self, Metadata},
    io::Error,
    os::unix::prelude::{MetadataExt, PermissionsExt},
};
use utils::oct_to_rwx_permissions;

#[derive(Debug)]
pub struct Config<'a> {
    pub tool: &'a str,
    pub path: &'a str,
    pub flag: &'a str,
}

pub struct LongListItem {
    pub file_type: char,
    pub permissions: String,
    pub hard_links_count: u64,
    pub owner: String,
    pub group: String,
    pub size: u64,
    pub date: String,
    pub file_name: String,
}

impl LongListItem {
    pub fn new(metadata: &Metadata, filename: &str) -> LongListItem {
        // generate directory char
        let file_type: char = if metadata.is_dir() { 'd' } else { '-' };

        // generate permissions
        let oct: String = format!("{:o}", &metadata.permissions().mode());
        let oct = String::from(&oct[oct.len() - 3..]);
        let permissions = oct_to_rwx_permissions(&oct);

        // generate number of hardlinks
        let hard_links_count = metadata.nlink();

        // generate owner's username
        let user_uid = metadata.uid();
        let user: User = User::from_uid(Uid::from_raw(user_uid)).unwrap().unwrap();

        let owner = user.name;

        // generate group's name
        let group_uid = metadata.gid();
        let group: Group = Group::from_gid(Gid::from_raw(group_uid)).unwrap().unwrap();

        let group = group.name;

        // generate file's size
        let size = metadata.size();

        // generate last time modified
        let timezone: i64 = Local::now().offset().local_minus_utc().into();
        let last_time_modified =
            chrono::NaiveDateTime::from_timestamp_opt(metadata.mtime() + timezone, 0).unwrap();

        let date = NaiveDateTime::format(&last_time_modified, "%d %h %H:%M").to_string();

        // generate file name
        let file_name = filename.to_string();

        LongListItem {
            file_type,
            permissions,
            hard_links_count,
            owner,
            group,
            size,
            date,
            file_name,
        }
    }
}

impl fmt::Display for LongListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}  {}  {}  {} {}  {} {}",
            &self.file_type,
            &self.permissions,
            &self.hard_links_count,
            &self.owner,
            &self.group,
            &self.size,
            &self.date,
            &self.file_name
        )
    }
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Config<'a>, Error> {
        let mut config = Config {
            tool: "",
            path: "",
            flag: "",
        };

        args.into_iter().for_each(|arg| {
            if arg.contains("-") {
                config.flag = arg;
            } else if arg.eq("ls") {
                config.tool = "ls";
            } else {
                config.path = arg;
            }
        });

        Ok(config)
    }
}

pub fn run(config: &Config) -> Result<(), Error> {
    match config.flag {
        "-h" => help_func(),
        "-l" => list_ls(&config.path).unwrap(),
        _ => standard_ls(&config.path).unwrap(),
    }
    Ok(())
}

pub fn standard_ls(path: &str) -> Result<(), Error> {
    let dir_iter = fs::read_dir(path).unwrap();

    dir_iter.for_each(|entry| {
        println!("{} ", entry.as_ref().unwrap().file_name().to_str().unwrap());
    });

    Ok(())
}

fn help_func() {
    println!("Current available flags for ls command:");
    println!("use -h to display help");
    println!("use -l to display content in long list format");
}

pub fn list_ls(path: &str) -> Result<(), Error> {
    let dir_iter = std::fs::read_dir(path).unwrap();

    dir_iter.for_each(|entry| {
        let entry_meta_data = entry.as_ref().unwrap().metadata().unwrap();

        let long_list_item = LongListItem::new(
            &entry_meta_data,
            entry.as_ref().unwrap().file_name().to_str().unwrap(),
        );
        println!("{}", long_list_item);
    });

    Ok(())
}
