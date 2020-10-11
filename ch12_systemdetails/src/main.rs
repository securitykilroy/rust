use sysinfo::{ProcessExt, SystemExt, UserExt};
use std::env;

fn display_memory() {
    let mut system = sysinfo::System::new_all();
    system.refresh_memory();

    println!("System memory:\t {} KB", system.get_total_memory());
    println!("Used memory:\t {} KB", system.get_used_memory());
    println!("Total swap:\t {} KB", system.get_total_swap());
    println!("Used swap:\t {} KB", system.get_used_swap());
}

fn display_disks() {
    let mut system = sysinfo::System::new_all();
    system.refresh_disks_list();

    for disk in system.get_disks() {
        println!("{:?}", disk);
    }
}

fn list_processes() {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    for (pid, proc_entry) in system.get_processes() {
        println!("{}:{}, status: {:?}", pid, proc_entry.name(), proc_entry.status());
    }
}

fn display_users() {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    for user in system.get_users() {
        println!("{} is in {} groups", user.get_name(), user.get_groups().len());
    }
}

fn main() {

    let s = sysinfo::System::new();
    println!("This system has been up {} seconds", s.get_boot_time());
    println!("The current process id is {}", sysinfo::get_current_pid().unwrap());

    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "disks" => display_disks(),
        "memory" => display_memory(),
        "process" => list_processes(),
        "users" => display_users(),
        _ => println!("You haven't provided an acceptable parameter")
    }

}
