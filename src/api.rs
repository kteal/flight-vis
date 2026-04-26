use serde::Deserialize;
use anyhow::Result;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Flight {
    pub icao24: String,
    pub callsign: Option<String>,
    pub origin_country: String,
    pub time_position: Option<i64>,
    pub last_contact: i64,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub baro_altitude: Option<f32>,
    pub on_ground: bool,
    pub velocity: Option<f32>,
    pub true_track: Option<f32>,
    pub vertical_rate: Option<f32>,
}

fn get_client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?)
}

pub async fn get_current_location() -> Result<Location> {
    let client = get_client()?;
    let resp = client.get("https://ipapi.co/json/").send().await?.json::<Location>().await?;
    Ok(resp)
}

#[derive(Deserialize)]
struct OpenSkyResponse {
    states: Option<Vec<Vec<serde_json::Value>>>,
}

pub async fn get_flights(lat: f64, lon: f64, radius_deg: f64) -> Result<Vec<Flight>> {
    let lamin = lat - radius_deg;
    let lamax = lat + radius_deg;
    let lomin = lon - radius_deg;
    let lomax = lon + radius_deg;

    let url = format!(
        "https://opensky-network.org/api/states/all?lamin={:.4}&lomin={:.4}&lamax={:.4}&lomax={:.4}",
        lamin, lomin, lamax, lomax
    );

    let client = get_client()?;
    let resp = client.get(url).send().await?.json::<OpenSkyResponse>().await?;

    let flights = resp.states.unwrap_or_default().into_iter().filter_map(|s| {
        if s.len() < 12 { return None; }
        
        Some(Flight {
            icao24: s[0].as_str()?.to_string(),
            callsign: s[1].as_str().map(|c| c.trim().to_string()),
            origin_country: s[2].as_str()?.to_string(),
            time_position: s[3].as_i64(),
            last_contact: s[4].as_i64()?,
            longitude: s[5].as_f64(),
            latitude: s[6].as_f64(),
            baro_altitude: s[7].as_f64().map(|v| v as f32),
            on_ground: s[8].as_bool()?,
            velocity: s[9].as_f64().map(|v| v as f32),
            true_track: s[10].as_f64().map(|v| v as f32),
            vertical_rate: s[11].as_f64().map(|v| v as f32),
        })
    }).collect();

    Ok(flights)
}
