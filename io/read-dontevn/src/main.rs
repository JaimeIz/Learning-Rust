use std::io::{Read, Result};
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new(".env");
    let fullpath = path.display(); 
    let mut file = File::open(path)?;

    let mut content = String::new();
    let bytes = file.read_to_string(&mut content)?;

    println!("File \"{fullpath}\" opened with a size of {bytes} bytes");

    

    Ok(())
}
