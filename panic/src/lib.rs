use std::fs::File;

// a function that tries to open a file and panics if the file does not exist.
pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
}