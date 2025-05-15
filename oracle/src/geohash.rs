use async_trait::async_trait;
use geohash::Coord;
use oracle::Location;

mod ip_info {
    use serde::Deserialize;
    const IPINFO: &str = "https://ipinfo.io";
    #[derive(Deserialize)]
    struct IpInfo {
        loc: String,
    }

    pub async fn get_ip() -> Result<(f64, f64), String> {
        let response = reqwest::get(IPINFO).await.map_err(|e| e.to_string())?;
        let ip_info: IpInfo = response.json().await.map_err(|e| e.to_string())?;

        let gps: Vec<f64> = ip_info
            .loc
            .split(",")
            .map(|s| s.trim().parse().expect("valid float number"))
            .collect();

        Ok((gps[0], gps[1]))
    }
}

pub struct Geohash;

#[async_trait]
impl Location for Geohash {
    type Output = String;

    async fn current_location(accuarcy: u8) -> Result<Self::Output, String> {
        let (x, y) = ip_info::get_ip().await?;
        geohash::encode(Coord { x, y }, accuarcy as usize).map_err(|e| e.to_string())
    }
}
