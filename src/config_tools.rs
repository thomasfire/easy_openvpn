//#[macro_use]
//extern crate serde_derive;
extern crate toml;
use io_tools;
use std::fs::create_dir;

#[derive(Serialize, Deserialize)]
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
            return Err(&format!("{:?}",err));
        }
    };
    Ok(config)
}

fn write_config(config: Config) -> Result<(), &'static str> {
    let conf_str = match toml::to_string(&config) {
        Ok(value) => value,
        Err(err) => {
            println!("Something went wrong while parsing the config: {}", err);
            panic!("{}", err);
        }
    };

    if !io_tools::exists("~/.ovpn") {
        match create_dir("~/.ovpn") {
            Ok(_) => {
                println!("Home .ovpn has been created");
            }
            Err(err) => {
                println!("An error occured in creating the direcrory: {}", err);
                return Err(&format!("{:?}",err));
            }
        };
    }

    match io_tools::write_to_file("~/.ovpn/easy_openvpn.config", conf_str) {
        Ok(_) => return Ok(),
        Err(err) => {
            println!("An error occured while writing to the config: {}", err);
            return Err(&format!("{:?}",err));
        }
    };
}

fn update_config(key: &str, value: &str) -> Result<(), ()> {
    let mut config = match read_config() {
        Ok(v) => v,
        Err(err) => {
            println!("Something went wrong in updating the config: {}", err);
            return Err(err);
        }
    };

    match key {
        "directory" => config.directory = String::from_str(value),
        "default_file" => config.default_file = String::from_str(value),
        _ => return Err("Wrong key in update_config"),
    };

    match write_config(config) {
        Ok(_) => println!("The config has been updated"),
        Err(err) => {
            println!("Error while updating the config: {}", err);
            return Err(err);
        }
    }
    Ok()
}

fn setup() -> Result<(), ()> {
    let dir = io_tools::read_std_line("Enter path to your working directory: ");
    let default_file = io_tools::read_std_line("Enter name of the file in the working directory: ");

    if !io_tools::exists(format!("{}/{}", dir, default_file)) {
        return Err("Wrong path. Maybe you wrote `/ ` in working directory");
    }

    let config = Config { dir, default_file };

    match write_config(config) {
        Ok(_) => return Ok("Ok"),
        Err(err) => {
            println!("Something went wrong in setup: {}", err);
            return err;
        }
    }
}
