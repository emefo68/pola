use std::{
    fs::File,
    io::{self, ErrorKind::Unsupported, Read},
    path::Path,
};

pub fn check_file_extension(path: &Path) -> Result<String, io::Error> {
    let extension = path
        .extension()
        .ok_or(io::Error::new(Unsupported, "No file extension on file"))?
        .to_str();

    match extension {
        Some("txt") => parse_txt_file(path),
        Some(_) => Err(io::Error::new(Unsupported, "Unsupported file extension")),
        None => Err(io::Error::new(Unsupported, "Unreadable extension Bytes")),
    }
}

pub fn parse_txt_file(path: &Path) -> Result<String, io::Error> {
    let mut file_string = String::new();
    File::open(path)?.read_to_string(&mut file_string)?;
    Ok(file_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_supported() {
        let file = std::env::temp_dir().join("test.txt");
        std::fs::write(&file, "Hello, Pola!").unwrap();
        let result = match check_file_extension(file.as_path()) {
            Ok(file_string) => file_string,
            Err(err) => panic!("Error checking file extension: {err:?}"),
        };
        assert!(result == "Hello, Pola!")
    }

    #[test]
    fn test_extension_unsupported() {
        let file = std::env::temp_dir().join("test.jpg");
        let result = check_file_extension(file.as_path());
        assert!(result.is_err());
        assert!(result.unwrap_err().kind() == Unsupported)
    }

    #[test]
    fn test_extension_no_extension() {
        let file = std::env::temp_dir().join("test");
        let result = check_file_extension(file.as_path());
        assert!(result.is_err());
        assert!(result.unwrap_err().kind() == Unsupported)
    }
}
