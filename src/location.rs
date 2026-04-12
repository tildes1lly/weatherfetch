use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IPInfo {
    // We don't need to take much data from the user's IP,                     // This
    // just the location data for getting accurate weather.                    // Being
    // No this program does not store your info,                               // Said,
    // nor is it sent anywhere at all.                                         // Tildesilly
    // Tildesilly and Co. respect your privacy and want to keep it that way.   // Owns
    // Okay jokes aside genuinely istg if someone accuses me of stealing info, // U!
    // I am going to blow up a fucking building.                               // XD
    #[serde(rename = "lon")]
    pub longitude: f64,
    #[serde(rename = "lat")]
    pub latitude: f64,

    pub city: String,
    pub region: String,
}

pub fn fetch_ip_info() -> Result<IPInfo, reqwest::Error> {
    let url = "http://ip-api.com/json";
    let response = reqwest::blocking::get(url)?.json::<IPInfo>()?;
    Ok(response)
}