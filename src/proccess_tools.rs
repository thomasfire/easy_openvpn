use std::process::Command;
use io_tools;
use config_tools::{choose_file, read_config};

fn help_print() {
    println!("\n\nAvailable commands:\n");
    println!("help     - show this message;");
    println!("switch   - switch to the n file of the list;");
    println!("set      - set n file as default;");
}

pub fn run_openvpn(path: &str) -> Result((), String) {
    let mut openvpn = Command::new("sudo")
        .args(&["openvpn", "--config", path])
        .spawn()
        .expect("Failed to run `sudo openvpn`");

    let mut config = read_config().unwrap();
    loop {
        match io_tools::read_std_line("=> ") {
            "help" => help_print(),
            "switch" => {
                let filename = choose_file(&config.directory).unwrap();
                openvpn.kill().expect("couldn`t kill the openvpn process");
                // TODO
            }
        };
    }
}
