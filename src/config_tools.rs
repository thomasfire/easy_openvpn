extern crate toml;
use io_tools;
use std::fs::create_dir;

#[derive(Serialize, Deserialize)]
struct Config {
    directory: String,
    default_file: String,
}

fn read_config() -> Result<Config, String> {
    if !io_tools::exists("~/.ovpn/easy_openvpn.config") {
        return Err(String::from(
            "No setup was processed. Please run `$ easy_openvpn --setup` for setup",
        ));
    }
    let conf_str = io_tools::read_str("~/.ovpn/easy_openvpn.config");
    let config: Config = match toml::from_str(&conf_str) {
        Ok(value) => value,
        Err(err) => {
            println!("Something goes wrong while reading the config: {}", err);
            return Err(format!("{:?}", err));
        }
    };
    Ok(config)
}

fn write_config(config: Config) -> Result<(), String> {
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
                return Err(format!("{:?}", err));
            }
        };
    }

    match io_tools::write_to_file("~/.ovpn/easy_openvpn.config", conf_str) {
        Ok(_) => return Ok(()),
        Err(err) => {
            println!("An error occured while writing to the config: {}", err);
            return Err(format!("{:?}", err));
        }
    };
}

fn update_config(key: &str, value: &str) -> Result<(), String> {
    let mut config = match read_config() {
        Ok(v) => v,
        Err(err) => {
            println!("Something went wrong in updating the config: {}", err);
            return Err(format!("{:?}", err));
        }
    };

    match key {
        "directory" => config.directory = String::from(value),
        "default_file" => config.default_file = String::from(value),
        _ => return Err(String::from("Wrong key in update_config")),
    };

    match write_config(config) {
        Ok(_) => println!("The config has been updated"),
        Err(err) => {
            println!("Error while updating the config: {}", err);
            return Err(err);
        }
    }
    Ok(())
}

fn choose_file(directory: &str) -> Result<String, String> {
    let files = io_tools::get_ovpn_files(directory);
    println!("Choose file you want to connect:\n");
    println!("l - last file;\n r - random file from the directory");
    for x in 0..files.len() {
        println!("{} - {}", x, files[x]);
    }
    let choosen = io_tools::read_std_line("=> ");
    let n: i32 = match choosen.parse::<i32>() {
        Ok(t) => t,
        Err(_) => -1,
    };
    if !n <= -1 {
        if n as usize >= files.len() {
            return Err(String::from("You number is bigger than you have files."));
        }
        return Ok(format!("{}", files[n as usize]));
    }
    match choosen.as_str() {
        "r" => return Ok(String::from("random")),
        "l" => return Ok(String::from("last")),
        _ => {
            return Err(String::from(
                "Your choice must be `l`, `r` or number from 0.",
            ))
        }
    }
}

fn setup() -> Result<(), String> {
    let directory = io_tools::read_std_line("Enter path to your working directory: ");
    let mut default_file = String::new();

    loop {
        match choose_file(directory) {
            Ok(name) => {
                default_file = name;
                break;
            }
            Err(err) => println!("{}", err),
        };
    }

    let config = Config {
        directory,
        default_file,
    };

    match write_config(config) {
        Ok(_) => return Ok(()),
        Err(err) => {
            println!("Something went wrong in setup: {}", err);
            return Err(err);
        }
    }
}
