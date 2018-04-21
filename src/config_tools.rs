extern crate toml;
use io_tools;
use std::fs::create_dir;

struct Config {
    directory: String,
    default_file: String,
}

fn read_config() -> Result<Config, &'static str> {
    if !io_tools::exists("~/.ovpn/easy_openvpn.config") {
        return Err("No setup was processed. Please run `$ easy_openvpn --setup` for setup");
    }
    let conf_str = io_tools::read_str("~/.ovpn/easy_openvpn.config");
    let config: Config = match toml::from_str(&conf_str) {
        Ok(value) => value,
        Err(err) => {
            println!("Something goes wrong while reading the config: {}", err);
            return Err(err);
        }
    };
    Ok(config)
}

fn write_config(dir: &str, default_file: &str) -> Result<&'static str, &'static str> {
    let config = Config {
        directory: String::from_str(dir),
        default_file: String::from_str(default_file),
    };

    let conf_str = match toml::to_string(&config) {
        Ok(value) => value,
        Err(err) => {
            println!("Something went wrong while parsing the config: {}", err);
            return (err);
        }
    };

    if !io_tools::exists("~/.ovpn") {
        match create_dir("~/.ovpn") {
            Ok(_) => {
                println!("Home .ovpn has been created");
            }
            Err(err) => {
                println!("An error occured in creating the direcrory: {}", err);
                return Err(err);
            }
        };
    }

    match io_tools::write_to_file("~/.ovpn/easy_openvpn.config", conf_str) {
        Ok(_) => return ("Ok"),
        Err(err) => println!("An error occured while writing to the config: {}", err),
    };
}
