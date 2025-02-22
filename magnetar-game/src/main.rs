use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseMotion, MouseWheel};
use bevy::prelude::*;
use magnetar_data::celestial::Object;
use magnetar_data::orbital::OrbitalParameters;
use magnetar_data::*;

// Define the Celestial component locally (not in celestial.rs)
#[derive(Component)]
pub struct Celestial {
    pub object: Object,
}

// Constants for scaling and camera controls
const SCALE_FACTOR: f32 = 0.1; // Scaling for celestial object radii
const CAMERA_PAN_SENSITIVITY: f32 = 0.005; // Sensitivity for mouse panning
const ZOOM_SENSITIVITY: f32 = 0.1; // Sensitivity for mouse wheel zoom
const MIN_ZOOM: f32 = 5.0; // Minimum camera zoom level
const MAX_ZOOM: f32 = 1000.0; // Maximum camera zoom level

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_system)
        .add_systems(Update, update_positions)
        .add_systems(Update, camera_movement_system)
        .add_systems(Update, camera_zoom_system)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .run();
}

// System to set up the initial scene
fn setup_system(mut commands: Commands) {
    // Spawn the main camera
    commands.spawn(Camera2d {
        ..Default::default()
    });

    // Load the celestial object from your YAML file.
    // Replace "path/to/your_file.yaml" with the actual file path.
    let object = yaml_parser::load_yaml("../data/celestial/sol.yaml")
        .expect("Failed to load YAML");

    // Spawn the object into the scene
    spawn_celestial(&mut commands, object, Vec3::ZERO);
}

// Spawns a celestial object in the scene.
fn spawn_celestial(commands: &mut Commands, object: Object, parent_position: Vec3) {
    let scale = object.radius as f32 * SCALE_FACTOR; // Adjust scale for object size

    commands.spawn((
        Celestial { object },
        Transform {
            translation: parent_position,
            scale: Vec3::splat(scale),
            ..Default::default()
        },
        GlobalTransform::default(),
    ));
}

// Updates positions based on orbital calculations.
fn update_positions(
    mut query: Query<(&mut Transform, &mut Celestial)>,
    time: Res<Time>,
) {
    let delta_time = time.delta(); // Fetch delta time for this frame
    for (mut transform, mut celestial) in query.iter_mut() {
        // Step forward the orbital parameters to calculate the new position.
        celestial.object.orbital_params.step_forward(delta_time);
        // Update transform using the calculated orbital position.
        transform.translation = calculate_orbital_position(&celestial.object.orbital_params);
    }
}

// Calculate the object's position based on its orbital parameters.
// The computation is done in f64 and then cast to f32 for the Vec3.
fn calculate_orbital_position(params: &OrbitalParameters) -> Vec3 {
    let a = params.semi_major_axis; // Semi-major axis in km
    let e = params.eccentricity;

    // Convert the mean anomaly (in radians) to true anomaly.
    let mean_anomaly_rad = params.mean_anomaly.to_radians();
    let true_anomaly = calculate_true_anomaly(mean_anomaly_rad, e);
    let radius = a * (1.0 - e.powi(2)) / (1.0 + e * true_anomaly.cos());

    // Convert polar (radius, angle) to Cartesian (x, y) and cast to f32.
    let x = radius * true_anomaly.cos();
    let y = radius * true_anomaly.sin();

    Vec3::new(x as f32, y as f32, 0.0)
}

// Converts eccentric anomaly to true anomaly using the standard formula:
// tan(true_anomaly/2) = sqrt((1+e)/(1-e)) * tan(eccentric_anomaly/2)
fn calculate_true_anomaly(mean_anomaly: f64, eccentricity: f64) -> f64 {
    // Solve Kepler's equation for eccentric anomaly via Newton's method.
    let mut eccentric_anomaly = mean_anomaly; // initial guess
    for _ in 0..5 {
        eccentric_anomaly -= (eccentric_anomaly - eccentricity * eccentric_anomaly.sin() - mean_anomaly)
            / (1.0 - eccentricity * eccentric_anomaly.cos());
    }
    2.0 * (((1.0 + eccentricity) / (1.0 - eccentricity)).sqrt() * (eccentric_anomaly / 2.0).tan()).atan()
}

// Handles camera movement based on mouse drag.
fn camera_movement_system(
    mut query: Query<&mut Transform, With<Camera>>,
    mut motion_events: EventReader<MouseMotion>,
    mut button_events: EventReader<MouseButtonInput>,
) {
    // Move camera when left mouse button is pressed.
    for mouseButtonEvent in button_events.read() {
        let button = mouseButtonEvent.button;
        let mouse_input = mouseButtonEvent.state;
        if button != MouseButton::Left {
            return;
        }
        if mouse_input.is_pressed() {
            for event in motion_events.read() {
                for mut transform in query.iter_mut() {
                    transform.translation.x -= event.delta.x * CAMERA_PAN_SENSITIVITY;
                    transform.translation.y += event.delta.y * CAMERA_PAN_SENSITIVITY; // Invert Y-axis for natural movement.
                }
            }
        }
    }
}

// Handles camera zoom with the mouse wheel.
fn camera_zoom_system(
    mut query: Query<&mut Transform, With<Camera>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    for event in scroll_events.read() {
        for mut transform in query.iter_mut() {
            transform.translation.z -= event.y * ZOOM_SENSITIVITY;
            transform.translation.z = transform.translation.z.clamp(MIN_ZOOM, MAX_ZOOM);
        }
    }
}
