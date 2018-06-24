extern crate libc;
use std::process::{Command, Child};
use io_tools;
use config_tools::{choose_file, read_config, write_config};
//use libc::{kill, SIGTERM};


fn help_print() {
    println!("\n\nAvailable commands:\n");
    println!("help     - show this message;");
    println!("switch   - switch to the n file of the list;");
    println!("restart  - restart openvpn with current profile;");
    println!("exit     - stop openvpn and exit;")
}

pub fn run_openvpn(path: &str) -> Result<Child, String> {
    let openvpn = Command::new("sudo")
        .args(&["openvpn", "--config", path])
        .spawn()
        .expect("Failed to run `sudo openvpn`");
    Ok(openvpn)
}

pub fn kill_openvpn(process: &Child) {
    let proc_id = process.id();
    println!("Killing ID: {}", proc_id);
    let mut kill_proc = Command::new("sudo")
        .args(&["kill", "-s", "SIGTERM", &format!("{}", proc_id)])
        .spawn()
        .expect("Couldn`t kill openvpn");
    kill_proc.wait().expect("Something has gone wrong while killing the openvpn");
    //println!("status: {}", kill_proc.status);
    //unsafe { libc::kill(proc_id as i32, libc::SIGINT);}
}

pub fn user_manager(mode: &str) -> Result<(), String> {
    let mut config = read_config().unwrap();

    let fpath = match mode {
        "last" => format!("{}/{}", config.directory, config.default_file),
        "random" => format!("{}/{}", config.directory,
                            choose_file(&config.directory, false).unwrap()),
        _ => String::from(mode),
    };

    let mut ovpn_proc = run_openvpn(&fpath)
        .expect("openvpn failed");
    loop {
        match io_tools::read_std_line("=> ").as_str() {
            "help" => help_print(),
            "switch" => {
                kill_openvpn(&ovpn_proc);
                ovpn_proc.wait().expect("Failed to wait killing openvpn");
                let filename = choose_file(&config.directory, false).unwrap();
                // ovpn_proc.kill().expect("couldn`t kill the openvpn process");
                config.default_file = String::from(filename);

                match write_config(&config) {
                    Ok(_) => println!("The config has been updated"),
                    Err(err) => println!("Something has gone wrong on writing to config: {}", err),
                }

                ovpn_proc = run_openvpn(format!("{}/{}", config.directory, config.default_file)
                    .as_str())
                    .expect("failed to restart openvpn");
            }
            "exit" => ovpn_proc.kill().expect("couldn`t kill the openvpn process"),
            "restart" => {
                ovpn_proc.kill().expect("couldn`t kill the openvpn process");
                ovpn_proc = run_openvpn(&fpath)
                    .expect("openvpn failed");
            },
            _ => help_print(),
        };
    }
}