use std::fs;
use std::path::Path;
use serde_json::Result;
use serde::{Deserialize, Serialize};

const CONFIG_NAME: &str = "config.json";
const CONFIG_DEFAULT: &str = r#"
{
    "udp_address": "127.0.0.1",
    "udp_port": 20002
}
"#;

#[derive(Serialize, Deserialize)]
struct ClientConfig
{
    udp_address: String,
    udp_port: u32,
}

fn load_config() -> Result<ClientConfig> {

    // check if file exists, if not overwrite it
    if !Path::new(CONFIG_NAME).exists()
    {
        println!("config does not exists, creating default file");

        fs::write(CONFIG_NAME, CONFIG_DEFAULT)
            .expect("TODO: panic message");
    }

    let contents = fs::read_to_string(CONFIG_NAME);

    if contents.is_err()
    {
        println!("could not open config file");
        assert!(false);
    }

    let config_as_str = contents.unwrap();
    println!("found config, raw json: {}", config_as_str);

    // parse into json crate
    return serde_json::from_str(config_as_str.as_str())
}

fn main() {
    println!("Hello, world!");

    let config = load_config().expect("poop");
    println!("address: {}, port: {}", config.udp_address, config.udp_port);
}
