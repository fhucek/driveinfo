use std::io::Error;
use std::env;

mod driveinfo;
use driveinfo::DriveInfo;

fn main() -> Result<(), Error>{
    let args : Vec<String>= env::args().collect();

    match args.get(1) {
        Some(param) => {
            if param == "--json" { 
                let json_drive_str = DriveInfo::json().unwrap();
                print!("{}",json_drive_str);
            } 
            else if param == "--all" { DriveInfo::pretty_print(true).unwrap(); }
            else { DriveInfo::pretty_print(false).unwrap(); }
        },
        None => { DriveInfo::pretty_print(false).unwrap(); }
    }

    Ok(())

}
