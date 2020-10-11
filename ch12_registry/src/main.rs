use std::io;
use std::path::Path;
use std::env;
use winreg::enums::*;
use winreg::RegKey;

fn enumerate_run() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let runkey = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")?;

    for (name, value) in runkey.enum_values().map(|entry| entry.unwrap()) {
        println!("{} = {:?}", name, value);
    }

    Ok(())
}

fn write_value() -> io::Result<()> {

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software").join("KilroySoft");
    let (key, reg_resp) = hkcu.create_subkey(&path)?;


    match reg_resp {
        REG_CREATED_NEW_KEY => println!("A new key has been created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    }

    let working_dir = env::current_dir()?;
    let display_dir = working_dir.to_str().unwrap();
    
    key.set_value("location", &display_dir)?;

    Ok(())
}

fn read_value() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let read_key = hkcu.open_subkey("SOFTWARE\\kilroySoft")?;

    let location: String = read_key.get_value("location")?;
    println!("Location: {}", location);

    Ok(())

}

fn main() -> io::Result<()> {

    enumerate_run()?;
    write_value()?;
    read_value()?;

    Ok(())
}