use std::time::Duration;
use std::f64::consts::PI;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OrbitalParameters {
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub longitude_of_periapsis: u16,
    pub mean_anomaly: f64,
}

impl OrbitalParameters {
    pub fn step_forward(&mut self, time_step: Duration) {
        let time_seconds = time_step.as_secs_f64();
        let mean_motion = (2.0 * PI) / (self.semi_major_axis.powf(1.5));
        self.mean_anomaly += mean_motion * time_seconds;
        self.mean_anomaly = self.mean_anomaly % (2.0 * PI); // Keep it within 0 to 2π            
    } 
}