use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub struct BootRom {
    pub data: Vec<u8>
}

impl BootRom {
    pub fn new(path: &Path) -> io::Result<BootRom> {
        File::open(path)
            .and_then(|mut file| {
                let mut data: Vec<u8> = Vec::new();
                file.read_to_end(&mut data)
                    .map(|_| data)
            })
            .map(|data| BootRom { data })
    }
}