use dirs;
use serde::Deserialize;
use serde::Serialize;
use colored::Colorize;

#[derive(Deserialize, Serialize)]
pub struct CustomLocation {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub hide_location: bool,
    pub use_imperial: bool,
    pub use_color: bool,
    pub no_icon: bool,
    pub show_forecast: bool,
    pub custom_location: Option<CustomLocation>,
}

pub fn defaults() -> Config {
    Config {
        hide_location: true,
        use_imperial: false,
        use_color: true,
        no_icon: false,
        show_forecast: false,
        custom_location: None,
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
                println!("Regenerating config file...");
                println!("\n{} {}", "If you want to keep your old config, exit".bright_red(), "now.".bright_red().bold());
                println!("\nWaiting 3 seconds before regenerating config...");
                std::thread::sleep(std::time::Duration::from_secs(3));
                return None;
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
const RAINING_ART: &str = include_str!("ascii/raining.txt");
const THUNDERSTORM_ART: &str = include_str!("ascii/thunderstorm.txt");
const SNOW_HAIL_ART: &str = include_str!("ascii/snow_hail.txt");

pub fn gen_config() -> Config {
    let mut raw_hidden = String::new();
    let mut raw_imperial = String::new();

    println!("\n{} {} {}", "Welcome to the", "weatherfetch".bright_blue().bold(), "setup wizard!");

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
        show_forecast: false,
        custom_location: None,
    };
    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize config");

    println!("{}", "Writing to config...\n".bold());
    let config_path = dirs::config_dir().unwrap().join("weatherfetch/");

    std::fs::create_dir_all(&config_path).expect("Creating directory failed...");
    std::fs::write(config_path.join("config.json"), json).expect("Write to config.json failed...");

    std::fs::create_dir_all(&config_path.join("ascii/")).expect("Creating ascii directory failed...");
    std::fs::write(config_path.join("ascii/clear.txt"), CLEAR_ART).expect("Writing clear.txt failed...");
    std::fs::write(config_path.join("ascii/partly_cloudy.txt"), PARTLY_CLOUDY_ART).expect("Writing partly_cloudy.txt failed...");
    std::fs::write(config_path.join("ascii/cloudy.txt"), CLOUDY_ART).expect("Writing cloudy.txt failed...");
    std::fs::write(config_path.join("ascii/raining.txt"), RAINING_ART).expect("Writing raining.txt failed...");
    std::fs::write(config_path.join("ascii/thunderstorm.txt"), THUNDERSTORM_ART).expect("Writing thunderstorm.txt failed...");
    std::fs::write(config_path.join("ascii/snow_hail.txt"), SNOW_HAIL_ART).expect("Writing snow_hail.txt failed...");

    std::fs::create_dir_all(&config_path.join("profiles/")).expect("Creating profiles directory failed...");
    
    println!("{}", "Done!".bold());
    println!("\n{}\n{} {} {} {}\n{} {} {}\n{}\n", &"♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡", &"weatherfetch".bold().bright_cyan(), &"made with ♡ by", &"tildesilly".bold().bright_magenta(), &"<3".bright_magenta(), &"dedicated to my weather nerd wife", &"mari".bold().blue(), &"<3".bright_magenta(), &"♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡♡");

    config
}

pub fn add_profile() -> Config {
    let mut raw_hidden = String::new();
    let mut raw_imperial = String::new();

    println!("\n{} {} {}", "Welcome to the", "weatherfetch".bright_blue().bold(), "profile setup wizard!");

    println!("{}", "\nEnter a name for this profile: ".bold());
    let mut profile_name = String::new();
    std::io::stdin().read_line(&mut profile_name).expect("Failed to read input");
    profile_name = profile_name.trim().to_string();

    println!("{}", "\nHide your location? (y/N) ".bold());
    std::io::stdin().read_line(&mut raw_hidden).expect("Failed to read input");
    raw_hidden = raw_hidden.to_lowercase();
    let hide_location = raw_hidden.chars().next() == Some('y');

    println!("{}", "\nUse imperial units? (y/N) ".bold());
    std::io::stdin().read_line(&mut raw_imperial).expect("Failed to read input");
    raw_imperial = raw_imperial.to_lowercase();
    let use_imperial = raw_imperial.chars().next() == Some('y');

    println!("{}", "\nUse colored output? (y/N) ".bold());
    let mut raw_color = String::new();
    std::io::stdin().read_line(&mut raw_color).expect("Failed to read input");
    raw_color = raw_color.to_lowercase();
    let use_color = raw_color.chars().next() == Some('y');

    println!("{}", "\nShow weather icons? (y/N) ".bold());
    let mut raw_icon = String::new();
    std::io::stdin().read_line(&mut raw_icon).expect("Failed to read input");
    raw_icon = raw_icon.to_lowercase();
    let no_icon = raw_icon.chars().next() == Some('n');

    println!("{}", "\nShow forecast information? (y/N) ".bold());
    let mut raw_forecast = String::new();
    std::io::stdin().read_line(&mut raw_forecast).expect("Failed to read input");
    raw_forecast = raw_forecast.to_lowercase();
    let show_forecast = raw_forecast.chars().next() == Some('y');

    println!("{}", "\nUse custom location? (y/N) ".bold());
    let mut raw_location = String::new();
    std::io::stdin().read_line(&mut raw_location).expect("Failed to read input");
    raw_location = raw_location.to_lowercase();
    let is_custom_location_set = raw_location.chars().next() == Some('y');

    let config: Config;

    if is_custom_location_set {
        let mut raw_lat = String::new();
        let mut raw_lon = String::new();

        println!("{}", "\nEnter latitude: ".bold());
        std::io::stdin().read_line(&mut raw_lat).expect("Failed to read input");
        let lat: f64 = raw_lat.trim().parse().expect("Invalid latitude");

        println!("{}", "\nEnter longitude: ".bold());
        std::io::stdin().read_line(&mut raw_lon).expect("Failed to read input");
        let lon: f64 = raw_lon.trim().parse().expect("Invalid longitude");

        config = Config {
            hide_location: hide_location,
            use_imperial: use_imperial,
            use_color: use_color,
            no_icon: no_icon,
            show_forecast: show_forecast,
            custom_location: Some(CustomLocation { lat, lon }),
        };
    } else {
        config = Config {
            hide_location: hide_location,
            use_imperial: use_imperial,
            use_color: use_color,
            no_icon: no_icon,
            show_forecast: show_forecast,
            custom_location: None,
        };
    }

    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize profile config");
    std::fs::write(dirs::config_dir().unwrap().join("weatherfetch/profiles/").join(format!("{}.json", profile_name)), json).expect("Failed to write profile config");

    println!("Profile created successfully!");

    config
}