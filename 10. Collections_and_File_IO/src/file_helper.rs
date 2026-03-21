use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::Path;

pub fn parse_file(file_name: &str) -> Result<String, String> {
    let path = Path::new(file_name);
    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                return Err(format!("Error: The file '{}' was not found.", file_name));
            } else {
                return Err(format!("Problem opening the file: {:?}", error));
            }
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => {
            return Err(format!("Problem reading the file contents: {:?}", e));
        }
    }
}
