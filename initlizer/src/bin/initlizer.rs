use initlizer::convert_txt_to_db;
use std;

fn main() {
    let filename = std::env::args().nth(1).expect("no pattern given");
    convert_txt_to_db(&filename);
}
