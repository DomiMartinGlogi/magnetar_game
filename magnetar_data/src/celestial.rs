use std::collections::HashMap;
use std::time::Duration;
use serde::Deserialize;
pub(crate) use crate::orbital::OrbitalParameters;

#[derive(Debug, Deserialize)]
pub enum ObjectType {
    Star,
    Rocky,
    Jovian,
    IceGiant
}

#[derive(Debug)]
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
    pub fn step_forward(&mut self, time_step: Duration) {
        self.orbital_params.step_forward(time_step);
        for child in self.children.iter_mut() {
            child.step_forward(time_step);
        }
    }
}
