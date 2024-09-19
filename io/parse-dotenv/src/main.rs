use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("Cargo.lock");
    let filename = path.display();

    let mut file = File::open(&path).unwrap();

    let mut text = String::new();
    if let Ok(nbytes) = file.read_to_string(&mut text) {
        println!("Opening file ({filename}) with a size of ({nbytes} bytes)");
    }

    let values = process_content(&text);
    for (key, value) in values.iter() {
        println!("{key} -> {value}");
    }
}

/**
 * This function is to parse a toml like file to values
 */
fn process_content(content: &String) -> HashMap<String, String> {
    let mut lines = HashMap::new();
    let mut prefix = "";

    for line in content.lines() {
        if line.starts_with('#') {
            continue;
        } else if line.starts_with('[') {
            prefix = line.trim_matches(['[', ']']);
        } else if let Some((key, value)) = line.split_once('=') {
            let fullkey: String = if prefix.is_empty() {
                key.to_string()
            } else {
                prefix.to_string() + "." + key
            };
            lines.insert(fullkey, value.to_string());
        } else {
            continue;
        }
    }

    lines
}
