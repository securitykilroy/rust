use winreg::enums::*;
use winreg::RegKey;
use std::path;
use std::io;

fn persist() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
   let cur_ver = hkcu.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE)?;

    let mut exe_string = String::new();

    match std::env::current_exe() {
        Ok(this_exe) => exe_string = format!("{}", this_exe.display()),
        Err(e) => println!("failed to get current exe {}", e)
    };

    match cur_ver.set_value("MyProcess", &exe_string) {
        Ok(_) => println!("Successfully added value"),
        Err(e) => println!("There was an error {}", e)
    };

    Ok(())
}


fn main() {

    match persist() {
        Ok(o) => o,
        Err(e) => panic!("Error! {}", e)
    };

}