use dirs;
use serde::Deserialize;
use serde::Serialize;
use colored::Colorize;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub hide_location: bool,
    pub use_imperial: bool,
    pub use_color: bool,
    pub no_icon: bool,
    pub forecast: bool,
}

pub fn defaults() -> Config {
    Config {
        hide_location: true,
        use_imperial: false,
        use_color: true,
        no_icon: false,
        forecast: false,
    }
}

pub fn get() -> Option<Config> {
    let config_dir = dirs::config_dir();
    let defaults: Config = defaults();
    if let Some(dir) = config_dir {
        let config_path = dir.join("weatherfetch/config.json");
        if let Ok(config_file) = std::fs::read_to_string(config_path) {
            if let Ok(config) = serde_json::from_str(&config_file) {
                return Some(config);
            } else {
                println!("Error parsing config file! this is almost certainly caused by a formatting issue in config.json.");
                println!("Falling back to hardcoded defaults...");
            }
        } else {
            return None; // Option<Config> will save us all
        }
    }
    Some(defaults)
}

const CLEAR_ART: &str = include_str!("ascii/clear.txt");
const PARTLY_CLOUDY_ART: &str = include_str!("ascii/partly_cloudy.txt");
const CLOUDY_ART: &str = include_str!("ascii/cloudy.txt");

pub fn gen_config() -> Config {
    let mut raw_hidden = String::new();
    let mut raw_imperial = String::new();

    println!("{}", "No config file found... falling back to setup wizard".bold());

    println!("{}", "\nHide your location? (y/N) ".bold());
    std::io::stdin().read_line(&mut raw_hidden).expect("Failed to read input");
    raw_hidden = raw_hidden.to_lowercase();
    let hide_location = raw_hidden.chars().next() == Some('y');

    println!("{}", "\nUse imperial units? (y/N) ".bold());
    std::io::stdin().read_line(&mut raw_imperial).expect("Failed to read input");
    raw_imperial = raw_imperial.to_lowercase();
    let use_imperial = raw_imperial.chars().next() == Some('y');

    println!("{}", "\nSetup wizard finished! Read the docs for changing other properties".bold());

    let config = Config {
        hide_location: hide_location,
        use_imperial: use_imperial,
        use_color: true,
        no_icon: false,
        forecast: false,
    };
    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize config");

    println!("{}", "Writing to config...\n".bold());
    let config_path = dirs::config_dir().unwrap().join("weatherfetch/");

    std::fs::create_dir_all(&config_path).expect("Creating directory failed...");
    std::fs::write(config_path.join("config.json"), json).expect("Write to config.json failed...");

    std::fs::create_dir_all(&config_path.join("ascii/")).expect("Creating directory failed...");
    std::fs::write(config_path.join("ascii/clear.txt"), CLEAR_ART).expect("Writing clear.txt failed...");
    std::fs::write(config_path.join("ascii/partly_cloudy.txt"), PARTLY_CLOUDY_ART).expect("Writing partly_cloudy.txt failed...");
    std::fs::write(config_path.join("ascii/cloudy.txt"), CLOUDY_ART).expect("Writing cloudy.txt failed...");
    
    println!("{}", "Done!".bold());
    println!("\n{}\n{} {} {}\n{}\n", &"♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡", &"weatherfetch".bold().bright_cyan(), &"made with ♡ by", &"tildesilly <3".bold().bright_magenta(), &"♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡");

    config
}