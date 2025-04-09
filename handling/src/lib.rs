use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    // Open the file in append mode
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Failed to open or create the file");

    // Write the content to the file
    use std::io::Write;
    file.write_all(content.as_bytes())
        .expect("Failed to write to the file");
}