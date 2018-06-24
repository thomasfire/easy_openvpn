extern crate easy_openvpn;

use std::env;
use easy_openvpn::config_tools::setup;
use easy_openvpn::proccess_tools::user_manager;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mode = args[1].as_str();
        match mode {
            "setup" => setup().unwrap(),
            _ => user_manager(mode).unwrap(),
        }
    } else {
        user_manager("last");
    }
}

