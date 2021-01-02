use std::io::BufReader;
use std::path::Path;
use std::fs::File;

/// open a file as a buffer
pub fn open(p: impl AsRef<Path>) -> BufReader<File>
{
    BufReader::new(File::open(p).unwrap())
}