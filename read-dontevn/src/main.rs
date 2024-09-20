use std::io::{Read, Result};
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("Cargo.lock");
    let filename = path.display();

    let mut file = File::open(&path).unwrap();

    let mut text = String::new();
    if let Ok(nbytes) = file.read_to_string(&mut text) {
        println!("Opening file ({filename}) with a size of ({nbytes} bytes)");
    }
    
    Ok(())
}
