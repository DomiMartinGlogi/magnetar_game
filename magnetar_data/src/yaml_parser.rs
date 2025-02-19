// yaml_parser.rs
use serde_yaml;
use std::fs;
use std::collections::HashMap;
use crate::celestial::{Object, ObjectType, OrbitalParameters};

pub fn load_yaml(file_path: &str) -> Result<Object, String> {
    let file_content = fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;
    let parsed_yaml: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&file_content).map_err(|e| format!("Failed to parse YAML: {}", e))?;

    // Handle the special "StarSystem" case
    if let Some(star_system) = parsed_yaml.get("StarSystem") {
        if let Some(system_mapping) = star_system.as_mapping() {
            if let Some((system_name, system_value)) = system_mapping.iter().next() {
                // StarSystem's first child element (e.g., "Sol")
                let name = system_name.as_str().unwrap_or("Unnamed").to_owned();
                return parse_object(name, system_value.clone());
            }
        }
        return Err("Malformed StarSystem definition".to_string());
    }

    // Default fallback: Parse as single object
    if let Some((name, value)) = parsed_yaml.into_iter().next() {
        parse_object(name, value)
    } else {
        Err("No valid object found in YAML".to_string())
    }
}

fn parse_object(name: String, value: serde_yaml::Value) -> Result<Object, String> {
    let obj_type = value.get("type").and_then(|v| v.as_str()).ok_or(format!("{} : Missing object type", name))?;
    let mass = value.get("mass").and_then(|v| v.as_f64()).ok_or("Missing mass")?;
    let radius = value.get("radius").and_then(|v| v.as_f64()).ok_or("Missing radius")?;

    let semi_major_axis = value.get("semi-major-axis").and_then(|v| v.as_f64());
    let eccentricity = value.get("eccentricity").and_then(|v| v.as_f64());
    let longitude_of_periapsis = value.get("longitude-of-periapsis").and_then(|v| v.as_f64());
    let mean_anomaly = value.get("mean-anomaly").and_then(|v| v.as_f64());

    let orbital_params = if let (Some(sma), Some(ecc), Some(lop), Some(ma)) = (semi_major_axis, eccentricity, longitude_of_periapsis, mean_anomaly) {
        Some(OrbitalParameters {
            semi_major_axis: sma,
            eccentricity: ecc,
            longitude_of_periapsis: lop as u16,
            mean_anomaly: ma,
        })
    } else {
        None
    };

    let atmosphere = value.get("atmosphere")
        .and_then(|v| v.as_mapping())
        .map(|map| {
            map.iter()
                .filter_map(|(k, v)| k.as_str().zip(v.as_f64()))
                .map(|(k, v)| (k.to_string(), v)) // Convert &str to String
                .collect::<HashMap<String, f64>>() // Collect explicitly into HashMap<String, f64>
        })
        .unwrap_or_default(); // Fallback to empty HashMap if "atmosphere" is missing or invalid


    let children = value.get("parentTo").and_then(|v| v.as_sequence()).map(|seq| {
        seq.iter()
            .filter_map(|child| child.as_mapping().and_then(|map| {
                if let Some((child_name, child_value)) = map.iter().next() {
                    child_name.as_str().map(|cn| parse_object(cn.to_string(), child_value.clone()).ok()).flatten()
                } else {
                    None
                }
            }))
            .collect()
    });

    Ok(Object {
        name,
        object_type: match obj_type {
            "STAR" => ObjectType::Star,
            "ROCKY" => ObjectType::Rocky,
            "JOVIAN" => ObjectType::Jovian,
            "ICE_GIANT" => ObjectType::IceGiant,
            _ => return Err("Invalid object type".to_string()),
        },
        mass,
        radius,
        orbital_params: orbital_params.unwrap_or(OrbitalParameters {
            semi_major_axis: 0.0,
            eccentricity: 0.0,
            longitude_of_periapsis: 0,
            mean_anomaly: 0.0,
        }),
        atmosphere,
        children: children.unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_yaml() {
        let result = load_yaml("test_data/valid.yaml");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_yaml() {
        let result = load_yaml("test_data/invalid.yaml");
        assert!(result.is_err());
    }
}
