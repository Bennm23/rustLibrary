use std::{fs::{self, File, OpenOptions}, io::Write};


pub struct Writer<'a> {
    filename : &'a str,
    append   : bool,
}

impl<'a> Writer<'a> {
    pub fn new(filename : &'a str) -> Self {
        Writer {
            filename,
            append: true,
        }
    }

    pub fn write(&self, text : &str) {
        let res = OpenOptions::new()
            .write(true)
            .append(self.append)
            .open(self.filename);

        let _ = match res {
            Err(e) => panic!("Failed to Open File {:?}", e),
            Ok(mut file) => {
                match writeln!(file, "{}", text) {
                    Ok(_) => {},
                    Err(e) => panic!("Problem Writing To File: {:?}", e),
                };
            }
        };

    }

    pub fn clear(&self) {

        let _ = match File::create(self.filename) {
            Ok(file) => file.set_len(0),
            Err(e) => panic!("Failed To Open File {:?}", e),
        };
    }
}
