use std::io::{self, Write};
use std::time::Duration;
use magnetar_data::celestial::Object;
use magnetar_data::yaml_parser::load_yaml;

/// Parse a timestep string like "1d6h" into a Duration.
/// Supported units: d (days), h (hours), m (minutes), s (seconds)
fn parse_timestep(input: &str) -> Option<Duration> {
    let mut total_seconds: u64 = 0;
    let mut num_buf = String::new();
    for c in input.chars() {
        if c.is_digit(10) {
            num_buf.push(c);
        } else {
            if num_buf.is_empty() {
                return None;
            }
            let value: u64 = num_buf.parse().ok()?;
            num_buf.clear();
            match c {
                'd' | 'D' => total_seconds += value * 24 * 3600,
                'h' | 'H' => total_seconds += value * 3600,
                'm' | 'M' => total_seconds += value * 60,
                's' | 'S' => total_seconds += value,
                _ => return None,
            }
        }
    }
    if !num_buf.is_empty() {
        return None;
    }
    Some(Duration::from_secs(total_seconds))
}

/// Recursively render an object (and its children) as a block of text lines.
/// The returned Vec<String> holds the block’s lines with the given indent.
fn render_object_block(object: &Object, indent: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let indent_str = " ".repeat(indent);
    // First line: object name.
    lines.push(format!("{}- {}", indent_str, object.name));
    // Second line: mean anomaly (stored in degrees).
    lines.push(format!("{}  Mean Anomaly: {:.3}°", indent_str, object.orbital_params.mean_anomaly));
    // Append each child's block (with increased indent) immediately after the parent.
    for child in &object.children {
        let child_block = render_object_block(child, indent + 2);
        lines.extend(child_block);
    }
    lines
}

/// Given a slice of objects, render their blocks and print them side by side in four columns.
/// Each object’s block is kept intact so that its children remain with the parent.
fn display_siblings(objects: &[Object], indent: usize, col_width: usize) {
    // Render each object's block.
    let rendered_blocks: Vec<Vec<String>> = objects.iter()
        .map(|obj| render_object_block(obj, indent))
        .collect();

    // Process the blocks in chunks of 4 (four columns per row).
    for chunk in rendered_blocks.chunks(4) {
        // Determine the maximum number of lines in this row.
        let height = chunk.iter().map(|block| block.len()).max().unwrap_or(0);
        for i in 0..height {
            for block in chunk {
                let line = if i < block.len() { &block[i] } else { "" };
                print!("{:<width$}", line, width = col_width);
            }
            println!();
        }
        println!(); // Blank line between rows.
    }
}

/// Display the full table. First, print the top-level object (e.g. "Sol"),
/// then, if it has children, display them in four columns.
fn display_table(system: &Object, col_width: usize) {
    // Print the top-level object.
    println!("- {}", system.name);
    println!("  Mean Anomaly: {:.3}°", system.orbital_params.mean_anomaly);
    println!();
    // Now display the children in four columns if they exist.
    if !system.children.is_empty() {
        display_siblings(&system.children, 2, col_width);
    }
}

fn main() {
    // Load the celestial system from YAML.
    let mut system = load_yaml("../data/celestial/sol.yaml")
        .expect("Failed to load YAML");

    // Clear the screen once at startup.
    print!("\x1B[2J");
    io::stdout().flush().unwrap();

    loop {
        // Move the cursor to the top left to rewrite the same area.
        print!("\x1B[H");
        // Display the table with 4 columns (using a fixed column width, e.g., 40 characters).
        display_table(&system, 40);
        io::stdout().flush().unwrap();

        // Prompt for a timestep.
        println!("\nEnter timestep (e.g., 1d6h) or press Enter to exit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        match parse_timestep(input) {
            Some(duration) => system.step_forward(duration),
            None => {
                println!("Invalid timestep format: {}", input);
                println!("Press Enter to try again...");
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).unwrap();
            }
        }
    }
}
