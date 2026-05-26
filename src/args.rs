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

    Some(config::Config {
        hide_location: hide_location,
        use_imperial: use_imperial,
        use_color: use_color,
        no_icon: no_icon,
    })
}