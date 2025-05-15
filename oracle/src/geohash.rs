//! Geohash implementation for location services.
//!
//! This module provides functionality to get the current geographical location
//! based on IP address and convert it to a geohash string.

use async_trait::async_trait;
use geohash::Coord;
use oracle::{Location, LocationError};

/// Module for retrieving geographical location data using IP geolocation.
///
/// This module interacts with the ipinfo.io API to determine the current
/// geographical location based on the device's IP address.
mod ip_info {
    use serde::Deserialize;
    /// The base URL for the ipinfo.io API service.
    const IPINFO: &str = "https://ipinfo.io";
    
    /// Structure for deserializing the ipinfo.io API response.
    #[derive(Deserialize)]
    struct IpInfo {
        /// The location string in format "latitude,longitude"
        loc: String,
    }

    /// Fetches the current geographical coordinates using IP geolocation.
    ///
    /// Makes an HTTP request to ipinfo.io to determine the current location
    /// based on the device's IP address. Parses the response and extracts
    /// latitude and longitude coordinates.
    ///
    /// # Returns
    ///
    /// * `Result<(f64, f64), String>` - A tuple of (latitude, longitude) if successful,
    ///   or an error message string if the request or parsing failed.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The HTTP request to ipinfo.io fails
    /// - The response cannot be parsed as valid JSON
    /// - The location format is invalid (not "latitude,longitude")
    /// - The latitude or longitude values cannot be parsed as valid floating-point numbers
    pub async fn get_ip() -> Result<(f64, f64), String> {
        let response = reqwest::get(IPINFO).await.map_err(|e| e.to_string())?;
        let ip_info: IpInfo = response.json().await.map_err(|e| e.to_string())?;
        let parts: Vec<&str> = ip_info.loc.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid location format: {}", ip_info.loc));
        }

        let lat = parts[0]
            .trim()
            .parse::<f64>()
            .map_err(|e| format!("Invalid latitude: {}", e))?;
        let lon = parts[1]
            .trim()
            .parse::<f64>()
            .map_err(|e| format!("Invalid longitude: {}", e))?;

        Ok((lat, lon))
    }
}

/// Implementation of the `Location` trait using geohash encoding.
///
/// This struct provides functionality to get the current geographical location 
/// and encode it as a geohash string with variable precision.
pub struct Geohash;

#[async_trait]
impl Location for Geohash {
    /// The output type is a String representing the geohash.
    type Output = String;

    /// Gets the current location and encodes it as a geohash string.
    ///
    /// This function:
    /// 1. Retrieves the current geographical coordinates using IP geolocation
    /// 2. Encodes these coordinates as a geohash string with the specified accuracy
    ///
    /// # Arguments
    ///
    /// * `accuarcy` - The desired accuracy of the geohash (1-12), which determines
    ///   the length of the generated geohash string. Higher values provide more precise location.
    ///
    /// # Returns
    ///
    /// * `Result<String, LocationError>` - A geohash string if successful, or
    ///   a LocationError if obtaining the location or encoding failed.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Failed to obtain the current location (LocationError::Location)
    /// - Failed to encode the coordinates as a geohash (LocationError::Output)
    async fn current_location(accuarcy: u8) -> Result<Self::Output, LocationError> {
        let (x, y) = ip_info::get_ip()
            .await
            .map_err(|_| LocationError::Location)?;

        geohash::encode(Coord { x, y }, accuarcy as usize)
            .map_err(|e| LocationError::Output(e.to_string()))
    }
}
