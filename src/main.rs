use std::io::Error;
use std::env;

mod driveinfo;
use driveinfo::DriveInfo;

fn main() -> Result<(), Error>{
    let args : Vec<String>= env::args().collect();
    if args.len() > 1 && args.get(1).unwrap() == "--json" {
        let json_drive_str = DriveInfo::json().unwrap();
        print!("{}",json_drive_str);
    } else {
        DriveInfo::pretty_print().unwrap();
    }

    Ok(())

}
