use std::collections::HashMap;
use std::time::Duration;
use serde::Deserialize;
pub(crate) use crate::orbital::OrbitalParameters;

#[derive(Debug, Deserialize)]
/// Enum for all Object types
pub enum ObjectType {
    /// Denotes a given object is a star
    Star,
    /// Denotes the given object is rocky, so either an Asteroid or a Rocky planet like Earth or Mars
    Rocky,
    /// Denotes the given object is a Jovian style Gas Giant such as Jupiter or Saturn
    Jovian,
    /// Denotes the given object is an Icy Gas Giant, such as Uranus or Neptune
    IceGiant
}

#[derive(Debug)]
/// Represents a given Celestial Object such as a Star, Planet or Asteroid
/// All of these are basically handled the same way
pub struct Object {
    pub name: String,
    pub object_type: ObjectType,
    pub mass: f64,
    pub radius: f64,
    pub orbital_params: OrbitalParameters,
    pub atmosphere: HashMap<String, f64>,

    pub children: Vec<Object>,
}

impl Object {
    /// Step forward in time for a given object and propagates to any children
    /// 
    /// * `time_step` - How much time to step forward 
    pub fn step_forward(&mut self, time_step: Duration) {
        self.orbital_params.step_forward(time_step);
        for child in self.children.iter_mut() {
            child.step_forward(time_step);
        }
    }
}
