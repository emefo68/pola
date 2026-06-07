use csv::ReaderBuilder;
use std::{
    collections::HashMap,
    fs::File,
    io::{
        self,
        ErrorKind::{InvalidData, Unsupported},
        Read,
    },
    path::Path,
};

use crate::normalizer::get_word_count;

pub fn tokenize_file(path: &Path) -> Result<HashMap<String, usize>, io::Error> {
    let file_extension = path
        .extension()
        .ok_or(io::Error::new(Unsupported, "No file extension on file"))?
        .to_str();

    let content = match file_extension {
        Some("txt") => parse_txt_file(path),
        Some("md") => parse_txt_file(path),
        Some("csv") => parse_csv_file(path),
        Some("pdf") => parse_pdf_file(path),
        Some(_) => Err(io::Error::new(Unsupported, "Unsupported file extension")),
        None => Err(io::Error::new(Unsupported, "Unreadable extension Bytes")),
    }?;

    Ok(get_word_count(&content))
}

pub(crate) fn parse_txt_file(path: &Path) -> Result<String, io::Error> {
    let mut file_string = String::new();
    File::open(path)?.read_to_string(&mut file_string)?;
    Ok(file_string)
}

pub(crate) fn parse_csv_file(path: &Path) -> Result<String, io::Error> {
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;
    let mut file_string = String::new();
    for (index, record) in rdr.records().enumerate() {
        let record = record.map_err(|e| {
            io::Error::new(InvalidData, format!("CSV parse error on row {index}: {e}"))
        })?;
        for (field_index, field) in record.iter().enumerate() {
            if field_index > 0 {
                file_string.push(' ');
            }
            file_string.push_str(field);
        }
        file_string.push('\n');
    }
    Ok(file_string)
}

pub(crate) fn parse_pdf_file(path: &Path) -> Result<String, io::Error> {
    let bytes = std::fs::read(path)?;
    let out = pdf_extract::extract_text_from_mem(&bytes).map_err(|e| {
        io::Error::new(
            InvalidData,
            format!("PDF parse error for file {}: {e}", path.display()),
        )
    })?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_txt() {
        let file = std::env::temp_dir().join("test.txt");
        std::fs::write(&file, "Hello, Pola!").unwrap();
        let result = match tokenize_file(file.as_path()) {
            Ok(file_string) => file_string,
            Err(err) => panic!("Error checking file extension: {err:?}"),
        };
        assert_eq!(result.get("hello"), Some(&1))
    }

    #[test]
    fn test_tokenize_md() {
        let file = std::env::temp_dir().join("test.md");
        std::fs::write(&file, "### Hello, Pola!").unwrap();
        let result = match tokenize_file(file.as_path()) {
            Ok(file_string) => file_string,
            Err(err) => panic!("Error checking file extension: {err:?}"),
        };
        assert_eq!(result.get("hello"), Some(&1))
    }

    #[test]
    fn test_tokenize_csv() {
        let file = std::env::temp_dir().join("test.csv");
        std::fs::write(&file, "hello,pola,this\nis,a,test\nfor,a,function").unwrap();
        let result = match tokenize_file(file.as_path()) {
            Ok(file_string) => file_string,
            Err(err) => panic!("Error checking file extension: {err:?}"),
        };
        assert_eq!(result.get("a"), Some(&2))
    }

    #[test]
    fn test_tokenize_faulty_csv() {
        let file = std::env::temp_dir().join("test_faulty.csv");
        std::fs::write(&file, b"hello,pola,this,is,a,test\nfor,a,function\xFF\xFE").unwrap();
        let result = tokenize_file(file.as_path());
        assert!(result.is_err());
        assert!(result.unwrap_err().kind() == InvalidData)
    }

    #[test]
    fn test_tokenize_pdf() {
        let file = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/test.pdf"
        ));
        let result = match tokenize_file(file) {
            Ok(file_string) => file_string,
            Err(err) => panic!("Error checking file extension: {err:?}"),
        };
        assert_eq!(result.get("pola"), Some(&3))
    }

    #[test]
    fn test_tokenize_faulty_pdf() {
        let file = std::env::temp_dir().join("test_faulty.pdf");
        std::fs::write(&file, b"Hello, Pola! This is a test for a faulty PDF file.").unwrap();
        let result = tokenize_file(file.as_path());
        assert!(result.is_err());
        assert!(result.unwrap_err().kind() == InvalidData)
    }

    #[test]
    fn test_tokenize_unsupported() {
        let file = std::env::temp_dir().join("test.jpg");
        let result = tokenize_file(file.as_path());
        assert!(result.is_err());
        assert!(result.unwrap_err().kind() == Unsupported)
    }

    #[test]
    fn test_tokenize_no_extension() {
        let file = std::env::temp_dir().join("test");
        let result = tokenize_file(file.as_path());
        assert!(result.is_err());
        assert!(result.unwrap_err().kind() == Unsupported)
    }
}
