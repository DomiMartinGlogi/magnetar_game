// orbit_visualizer.rs
use std::time::Duration;
use magnetar_data::celestial::Object;
use magnetar_data::yaml_parser::load_yaml;
use std::thread::sleep;

fn display_orbits(object: &Object, depth: usize) {
    let indent = " ".repeat(depth * 2);
    println!("{}- {}", indent, object.name);

    for child in &object.children {
        display_orbits(child, depth + 1);
    }
}

fn main() {
    let system = load_yaml("../data/celestial/sol.yaml").expect("Failed to load YAML");

    loop {
        println!("\x1B[2J\x1B[1;1H"); // Clear screen
        display_orbits(&system, 0);

        sleep(Duration::from_secs(1));
    }
}