extern crate toml;
extern crate rand;

use io_tools;
use std::fs::create_dir;
use std::env::home_dir;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub directory: String,
    pub default_file: String,
}


fn enable_home_dir(path: &str) -> String {
    let phome = home_dir().unwrap();
    let home = phome.as_os_str().to_str().unwrap();
    String::from(path.replace("~", home))
}

/// Reads `~/.ovpn/easy_openvpn.config.toml` and returns Result with Config on Ok()
///
/// # Examples
///
/// ```rust
/// let config = read_config().unwrap();
/// ```
pub fn read_config() -> Result<Config, String> {
    if !io_tools::exists(&enable_home_dir("~/.ovpn/easy_openvpn.config.toml")) {
        return Err(String::from(
            "No setup was processed. Please run `$ easy_openvpn --setup` for setup",
        ));
    }
    let conf_str = io_tools::read_str(&enable_home_dir("~/.ovpn/easy_openvpn.config.toml"));
    let config: Config = match toml::from_str(&conf_str) {
        Ok(value) => value,
        Err(err) => {
            println!("Something goes wrong while reading the config: {}", err);
            return Err(format!("{:?}", err));
        }
    };
    Ok(config)
}

/// Writes Config to the `~/.ovpn/easy_openvpn.config.toml`, returns Result
///
/// # Examples
///
/// ```rust
/// let config = Config {
///     directory: String::from("~/.ovpn"),
///     default_file: String::from("~/San_Francisco.ovpn"),
/// };
/// write_config(config).unwrap();
/// ```
pub fn write_config(config: &Config) -> Result<(), String> {
    let conf_str = match toml::to_string(config) {
        Ok(value) => value,
        Err(err) => {
            println!("Something went wrong while parsing the config: {}", err);
            panic!("{}", err);
        }
    };

    if !io_tools::exists(&enable_home_dir("~/.ovpn")) {
        match create_dir(&enable_home_dir("~/.ovpn")) {
            Ok(_) => {
                println!("Home .ovpn has been created");
            }
            Err(err) => {
                println!("An error occured in creating the direcrory: {}", err);
                return Err(format!("{:?}", err));
            }
        };
    }

    match io_tools::write_to_file(&enable_home_dir("~/.ovpn/easy_openvpn.config.toml"),
                                  conf_str) {
        Ok(_) => return Ok(()),
        Err(err) => {
            println!("An error occured while writing to the config: {}", err);
            return Err(format!("{:?}", err));
        }
    };
}

/// Updates one item of the config
///
/// # Examples
///
/// ```rust
/// update_config("directory", "~/OpenVPN").unwrap();
/// update_config("defailt_file", "Telegram.ovpn").unwrap();
/// ```
pub fn update_config(key: &str, value: &str) -> Result<(), String> {
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

    match write_config(&config) {
        Ok(_) => println!("The config has been updated"),
        Err(err) => {
            println!("Error while updating the config: {}", err);
            return Err(err);
        }
    }
    Ok(())
}

/// Asks user to choose file and returns filename, or `last` or `random` on Ok() and error string on Err()
///
/// # Examples
///
/// ```rust
/// let filename = match choose_file("~/.ovpn", false) {
///     Ok(name) => name,
///     Err(err) => panic!("{}", err),
/// };
/// ```
pub fn choose_file(directory: &str, random: bool) -> Result<String, String> {
    let files = io_tools::get_ovpn_files(directory);
    println!("Choose file you want to connect:\n");
    println!("l - last file;\n r - random file from the directory");

    for x in 0..files.len() {
        println!("{} - {}", x, files[x]);
    }

    let n: i32;

    if random == false {
        let choosen = io_tools::read_std_line("=> ");
        n = match choosen.parse::<i32>() {
            Ok(t) => t,
            Err(_) => -1,
        };
    } else {
        n = rand::random::<i32>() % (files.len() as i32);
    }

    if !n <= -1 {
        if n as usize >= files.len() {
            return Err(String::from("You number is bigger than you have files."));
        }
        return Ok(format!("{}", files[n as usize]));
    }
    let config = read_config().unwrap();
    Ok(config.default_file)
}

/// Runs initial setup and sets default file and directory
///
/// Returns nothing on Ok() and string with error on Err()
pub fn setup() -> Result<(), String> {
    let tdirectory = enable_home_dir(&io_tools::read_std_line("Enter path to your working directory: "));
    let tdefault_file: String;

    loop {
        match choose_file(&tdirectory, false) {
            Ok(name) => {
                tdefault_file = name;
                break;
            }
            Err(err) => println!("{}", err),
        };
    }

    let config = Config {
        directory: tdirectory,
        default_file: tdefault_file,
    };

    match write_config(&config) {
        Ok(_) => return Ok(()),
        Err(err) => {
            println!("Something went wrong in setup: {}", err);
            return Err(err);
        }
    }
}
