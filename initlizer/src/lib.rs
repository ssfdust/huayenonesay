extern crate database;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
use std::path::Path;
use database::{establish_connection, write_saying};

fn list_filecontent(filename: &str) -> io::Result<Vec<String>> {
    File::open(filename).and_then(|file_in| {
        let file_reader = BufReader::with_capacity(2048, file_in);
        Ok(file_reader
            .lines()
            .map(|line_in| {
                line_in.unwrap_or(String::new())
            })
            .collect::<Vec<String>>())
    })
}

pub fn convert_txt_to_db(filename: &str) {
    let chapter = Path::new(filename).file_stem().map(|chapter| chapter.to_str().unwrap()).unwrap();
    match list_filecontent(filename) {
        Ok(contents) => {
            for content in contents {
                write_saying(&establish_connection(), content, chapter.to_owned());
            }
        }
        _ => ()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
