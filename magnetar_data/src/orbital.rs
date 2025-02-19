use std::time::Duration;
use std::f64::consts::PI;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Represents an Objects Orbital Parameters
pub struct OrbitalParameters {
    /// Semi Major Axis in km
    pub semi_major_axis: f64,
    /// Orbital Eccentricity, 0 implies perfectly circular orbit, 1 parabolic, >1 hyperbolic, between 0 and 1 ellipsoidal
    pub eccentricity: f64,
    /// Position of the periapsis in degrees, where the orbit "points"
    pub longitude_of_periapsis: u16,
    /// Current position of the object in the orbit in degrees
    pub mean_anomaly: f64,
}

impl OrbitalParameters {
    /// Calculates the Objects next position in Orbit and moves it there.
    pub fn step_forward(&mut self, time_step: Duration) {
        if self.semi_major_axis == 0.0 {
            return;
        }
        let time_seconds = time_step.as_secs_f64();
        let mean_motion = (360.0) / (self.semi_major_axis.powf(1.5)); // Replace 2π with 360
        self.mean_anomaly += mean_motion * time_seconds;
        self.mean_anomaly = self.mean_anomaly % 360.0; // Keep within 0 to 360 degrees
    } 
}
