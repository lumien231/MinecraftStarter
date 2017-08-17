#![windows_subsystem = "windows"]

use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::fs::copy;

use std::process::Command;
use std::os::windows::process::CommandExt;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() <= 1
    {
        return;
    }

    let mut index = 1;
    while index < args.len()
    {
        args[index] = args[index].replace("/?", "/minecraft?");

        index = index + 1;
    }

    println!("{:?}", args);

    let ui_path = Path::new(&args[1]);
    let mut copy_path_buffer = PathBuf::from(ui_path);
    copy_path_buffer.set_file_name("TwitchUI_cp.exe");
    let copy_path = copy_path_buffer.as_path();
    
    copy(ui_path, copy_path).ok();

    
    let mut child = Command::new(copy_path)
                        .args(&args[2..])
                        .creation_flags(0x00000010)
                        .current_dir(ui_path.parent().unwrap().parent().unwrap())
                        .spawn()
                        .expect("Failed to Execute TwitchUI");
    
    child.wait().ok();
}
