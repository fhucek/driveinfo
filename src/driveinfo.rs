use std::process::Command;
use std::error::Error;
use std::fmt;
use serde::{Deserialize, Serialize};
use console;
use console::style;

/*
    Goal here is to:
    1) get DriveInfos
        - various commands and functions to populate all info
            - populate from df -H output
        - provide a function, given a filesystem name, eg /dev/sda2, return a new DriveInfo
    2) Print DriveInfos
    */

#[derive(Serialize, Deserialize)]
pub struct DriveInfo {
    pub filesystem: String, // eg /dev/sda2
    pub mount_point: String, // eg /drives/breen
    pub size: String, // eg 109G
    pub used: String,
    pub avail: String,
    pub percent_used: String
    
}

impl fmt::Display for DriveInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DriveInfo: filesystem {}, mounted on {}, size {}, used {}, available {}, percent used {}", 
                    self.filesystem,
                    self.mount_point,
                    self.size,
                    self.used,
                    self.avail,
                    self.percent_used)
    }
}



impl DriveInfo {
    pub fn json() -> std::result::Result<String, Box<dyn Error>>{
        let drives = DriveInfo::parse_all_drives(true)?;

        // let json_drive_string = serde_json::to_string(&DriveInfo::new(String::from("hi")))?;
        let json_drive_string = serde_json::to_string(&drives)?;
        Ok(json_drive_string)
    }

    pub fn pretty_print(show_all_drives: bool) -> std::result::Result<(), Box<dyn Error>> {
        let drives = DriveInfo::parse_all_drives(show_all_drives)?;


        let mount_point_spacing = DriveInfo::find_longest_string_length(
            drives.iter().map(|driveinfo| &(driveinfo.mount_point)).collect()
        );
        let filesystem_spacing = DriveInfo::find_longest_string_length(
            drives.iter().map(|driveinfo| &(driveinfo.filesystem)).collect()
        );
        let size_spacing = DriveInfo::find_longest_string_length(
            drives.iter().map(|driveinfo| &(driveinfo.size)).collect()
        );
        // let used_spacing = DriveInfo::find_longest_string_length(
        //     drives.iter().map(|driveinfo| &(driveinfo.used)).collect()
        // );
        let avail_spacing = DriveInfo::find_longest_string_length(
            drives.iter().map(|driveinfo| &(driveinfo.avail)).collect()
        );

        for drive in drives {
            let percent_used: i32 = (drive.percent_used.replace("%", "").parse()).unwrap_or(0);

            let mnt_pt = DriveInfo::insert_spacing(mount_point_spacing, drive.mount_point);
            let fs =  DriveInfo::insert_spacing(filesystem_spacing, drive.filesystem);
            let size = DriveInfo::insert_spacing(size_spacing, drive.size);
            // let used =  DriveInfo::insert_spacing(used_spacing, drive.used);
            let avail = DriveInfo::insert_spacing(avail_spacing, drive.avail);
            let percent = DriveInfo::insert_spacing(4, drive.percent_used);

            let (formatted_percent, formatted_avail) = if percent_used < 40 {
                (style(percent).bold().green(), style(avail).bold().green())
            } else if percent_used < 80 {
                (style(percent).bold().yellow(), style(avail).bold().yellow())
            } else {
                (style(percent).bold().red(), style(avail).bold().red())
            };

            println!("{}\tUsed: {}\t{}/{} available\t{}",
                    style(mnt_pt).bold().cyan(),
                    formatted_percent,
                    formatted_avail,
                    style(size).bold(),
                    fs
                    );
        }
        
        Ok(())
    }

    fn insert_spacing(total_length: usize, string: String) -> String {
        let mut newstring = string.clone();

        while newstring.len() < total_length {
            newstring = newstring + " ";
        }
        newstring
    }

    fn find_longest_string_length(string_list: Vec<&String>) -> usize {
         match string_list.iter().map(|string| (**string).len() ).max() {
            Some(val) => val,
            None => 30 // 30 chars long default
         }
    }

    fn null() -> DriveInfo {
        DriveInfo {
            filesystem: String::from("null"),
            mount_point: String::from("null"), // eg /drives/breen
            size: String::from("null"), // eg 109G
            used: String::from("null"),
            avail: String::from("null"),
            percent_used: String::from("null")
        }
    }

    fn parse_df_line(df_line : &str) -> Option<DriveInfo> {
        let newdriveinfo : Vec<&str> = df_line.split(" ")
                                                .filter(|string| *string != "")
                                                .collect();
        // use regex instead                          
        if newdriveinfo.len() == 6 {
            let fsname = newdriveinfo.get(0)?;
            let size = newdriveinfo.get(1)?;
            let used = newdriveinfo.get(2)?;
            let avail = newdriveinfo.get(3)?;
            let percent_used = newdriveinfo.get(4)?;
            let mount_point = newdriveinfo.get(5)?;

            Some(DriveInfo {
                filesystem: String::from(*fsname),
                mount_point: String::from(*mount_point),
                size: String::from(*size),
                used: String::from(*used),
                avail: String::from(*avail),
                percent_used: String::from(*percent_used)
            })
        } else {
            None
        }
        
    }

    fn parse_all_drives(show_all_drives: bool) -> std::result::Result<Vec<DriveInfo>, Box<dyn Error>> {

        let df_proc = Command::new("df").arg("-H").output()?;
        let df_out = String::from_utf8(df_proc.stdout)?;
        let drive_info_list : Vec<DriveInfo> = df_out.split("\n")
                                                        .filter( | string | {
                                                            if !show_all_drives{
                                                                (**string).contains("/dev/sd")
                                                            } else {
                                                                !(**string).contains("Filesystem") && !((*string) == "")
                                                            }
                                                        })
                                                        .map(|line| {
                                                            match DriveInfo::parse_df_line(line) {
                                                                Some(driveinfo) => driveinfo,
                                                                None => DriveInfo::null()
                                                            }
                                                        })
                                                        .collect();
        Ok(drive_info_list)
    }

    
}
