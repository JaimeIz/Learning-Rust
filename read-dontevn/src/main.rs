use std::error;
use std::fs::File;
use std::io::{self, Error, Read};

fn main() -> io::Result<()> {
    let filename = "Cargo.toml";

    let (nbytes, _content) = read_file(filename)?;

    println!("Opening file ({filename}) with a size of ({nbytes} bytes)");

    Ok(())
}

fn read_file<'a>(filename: &str) -> Result<(usize, String), Error> {
    let mut file = File::open(filename)?;

    let mut buffer = String::new();

    let nbytes = file.read_to_string(&mut buffer)?;

    Ok((nbytes, buffer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_file() {
        let filename = "../tests-data/.env";

        let (nbytes, _content) = read_file(filename).unwrap();

        assert!(nbytes == 252, "ERROR: the size seems off");
    }
    
    #[test]
    #[should_panic(expected = "kind: NotFound")]
    fn open_non_existent_file() {
        let filename = "i-dont-exist";

        let (nbytes, _content) = read_file(filename).unwrap();

        assert!(nbytes == 252, "ERROR: the size seems off");
    }
}
