use crate::config;

pub fn parse(args: Vec<String>) -> Option<config::Config> {
    let mut custom_profile: Option<config::Config> = None;

    if args.contains(&String::from("--profile")) || args.contains(&String::from("-p")) {
        let profile_name = args.iter()
            .position(|x| x == "--profile" || x == "-p")
            .and_then(|idx| args.get(idx + 1))
            .map(|s| s.as_str())
            .unwrap_or("");
        
        let config_path = dirs::config_dir().unwrap().join("weatherfetch/profiles/").join(format!("{}.json", profile_name));
        if config_path.exists() {
            let config_data = std::fs::read_to_string(config_path).expect("Failed to read profile config");
            if let Ok(profile_config) = serde_json::from_str::<config::Config>(&config_data) {
                custom_profile = Some(profile_config);
            } else {
                println!("Failed to parse profile config! defaulting to generated config...");
            }
        } else {
            println!("Profile not found! defaulting to generated config...");
        }
    } else if args.contains(&String::from("--add-profile")) {
        return Some(config::add_profile()); 
    }

    let current_config: config::Config;
    if custom_profile.is_none() {
        current_config = match config::get() {
            Some(config) => config,
            None => return None,
        };
    } else {
        current_config = custom_profile.unwrap();
    }

    let mut hide_location = current_config.hide_location;
    let mut use_imperial = current_config.use_imperial;
    let mut use_color = current_config.use_color;
    let mut no_icon = current_config.no_icon;
    let mut show_forecast = current_config.show_forecast;
    let mut is_custom_location_set = current_config.custom_location.is_some();

    let mut lon: f64;
    let mut lat: f64;

    if is_custom_location_set {
        lat = current_config.custom_location.as_ref().unwrap().lat;
        lon = current_config.custom_location.as_ref().unwrap().lon;
    } else {
        lat = 0.0;
        lon = 0.0;
    }
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!("Usage: weatherfetch [OPTIONS]");
        println!("\nOptions:");
        println!("  --hide-location           Hide location information");
        println!("  --show-location           Show location information");
        println!("  --use-imperial            Use imperial units (Fahrenheit, mph, etc.)");
        println!("  --use-metric              Use metric units (Celsius, km/h, etc.)");
        println!("  --no-color                Disable colored output");
        println!("  --color                   Enable colored output");
        println!("  --no-icon                 Disable weather icons");
        println!("  --icon                    Enable weather icons");
        println!("  --show-forecast, -f       Show forecast information");
        println!("  --hide-forecast           Hide forecast information");
        println!("  --lat <latitude>          Set custom latitude for weather data");
        println!("  --lon <longitude>         Set custom longitude for weather data");
        println!("  --profile, -p <name>     Load a profile by name");
        println!("  --add-profile             Create a new profile with custom settings");
        return Some(config::Config {
            hide_location: false,
            use_imperial: false,
            use_color: false,
            no_icon: false,
            show_forecast: false,
            custom_location: Some(config::CustomLocation {
                lat: 0.0,
                lon: 0.0,
            }),
        });
    }
    if args.contains(&String::from("--hide-location")) {
        hide_location = true;
    }
    if args.contains(&String::from("--show-location")) {
        hide_location = false;
    }
    if args.contains(&String::from("--use-imperial")) {
        use_imperial = true;
    }
    if args.contains(&String::from("--use-metric")) {
        use_imperial = false;
    }
    if args.contains(&String::from("--no-color")) {
        use_color = false;
    }
    if args.contains(&String::from("--color")) {
        use_color = true;
    }
    if args.contains(&String::from("--no-icon")) {
        no_icon = true;
    }
    if args.contains(&String::from("--icon")) {
        no_icon = false;
    }
    if args.contains(&String::from("--show-forecast")) || args.contains(&String::from("-f")) {
        show_forecast = true;
    }
    if args.contains(&String::from("--hide-forecast")) {
        show_forecast = false;
    }
    if (args.contains(&String::from("--lon")) && !args.contains(&String::from("--lat"))) || (!args.contains(&String::from("--lon")) && args.contains(&String::from("--lat"))) {
        println!("Both --lat and --lon must be provided to set a custom location! defaulting to Cedar Point");
        lat = 41.4822;
        lon = -82.6832;
        is_custom_location_set = true;
    } else {
        if args.contains(&String::from("--lat")) {
            if !is_custom_location_set {
                is_custom_location_set = true;
            }
            let lat_str = args.iter()
                .position(|x| x == "--lat")
                .and_then(|idx| args.get(idx + 1))
                .map(|s| s.as_str())
                .unwrap_or("");
            lat = lat_str.parse::<f64>().unwrap_or(41.4822);
            if lat == 41.4822 {
                println!("Invalid latitude value provided! defaulting to Cedar Point");
            }
        }
        if args.contains(&String::from("--lon")) {
            if !is_custom_location_set {
                is_custom_location_set = true;
            }
            let lon_str = args.iter()
                .position(|x| x == "--lon")
                .and_then(|idx| args.get(idx + 1))
                .map(|s| s.as_str())
                .unwrap_or("");
            lon = lon_str.parse::<f64>().unwrap_or_else(|_| {
                println!("Invalid longitude value provided! defaulting to Cedar Point");
                -82.6832
            });
        }
    }

    if is_custom_location_set {
        if lat.is_nan() || lon.is_nan() {
            return Some(config::Config {
                hide_location: hide_location,
                use_imperial: use_imperial,
                use_color: use_color,
                no_icon: no_icon,
                show_forecast: show_forecast,
                custom_location: current_config.custom_location,
            });
        } else {
            return Some(config::Config {
                hide_location: hide_location,
                use_imperial: use_imperial,
                use_color: use_color,
                no_icon: no_icon,
                show_forecast: show_forecast,
                custom_location: Some(config::CustomLocation {
                    lat: lat,
                    lon: lon,
                }),
            });
        }
    } else {
        Some(config::Config {
            hide_location: hide_location,
            use_imperial: use_imperial,
            use_color: use_color,
            no_icon: no_icon,
            show_forecast: show_forecast,
            custom_location: None,
        })
    }
}