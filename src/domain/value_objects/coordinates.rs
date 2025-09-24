use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Coordinates value object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Error, Debug)]
pub enum CoordinatesError {
    #[error("Latitude must be between -90 and 90 degrees")]
    InvalidLatitude,
    #[error("Longitude must be between -180 and 180 degrees")]
    InvalidLongitude,
}

impl Coordinates {
    /// Create new coordinates
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, CoordinatesError> {
        if latitude < -90.0 || latitude > 90.0 {
            return Err(CoordinatesError::InvalidLatitude);
        }
        
        if longitude < -180.0 || longitude > 180.0 {
            return Err(CoordinatesError::InvalidLongitude);
        }
        
        Ok(Self { latitude, longitude })
    }

    /// Calculate distance to another point (Haversine formula)
    pub fn distance_to(&self, other: &Coordinates) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;
        
        let lat1_rad = self.latitude.to_radians();
        let lat2_rad = other.latitude.to_radians();
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lon = (other.longitude - self.longitude).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2) +
                lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        EARTH_RADIUS_KM * c
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.6}, {:.6})", self.latitude, self.longitude)
    }
}
