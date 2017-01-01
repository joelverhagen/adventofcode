use std::fs::File;
use std::io::BufReader;
use std::io::Bytes;
use std::io::Read;
use std::io;
use decompresserror::DecompressError;

pub struct FileChars {
    bytes: Bytes<BufReader<File>>,
}

impl FileChars {
    pub fn open(path: &str) -> Result<FileChars, io::Error> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err)   => return Err(err),
        };

        let reader = BufReader::new(file);
        let bytes = reader.bytes();

        Ok(FileChars {
            bytes: bytes,
        })
    } 
}

impl Iterator for FileChars {
    type Item = Result<char, DecompressError>;

    fn next(&mut self) -> Option<Result<char, DecompressError>> {
        match self.bytes.next() {
            Some(Ok(b)) if b < 128 => Some(Ok(b as char)),
            None                   => None,
            Some(Ok(_))            => Some(Err(DecompressError::NonAsciiCharEncountered)),
            Some(Err(_))           => Some(Err(DecompressError::CouldNotReadFile)),
        }
    }
}
