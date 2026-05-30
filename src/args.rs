use crate::config;

pub fn parse(args: Vec<String>) -> Option<config::Config> {
    let current_config = match config::get() {
        Some(config) => config,
        None => return None,
    };

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