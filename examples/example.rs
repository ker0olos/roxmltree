use std::{fs, path::PathBuf};

fn main() {
    let input = PathBuf::from("examples/face.svg");

    let contents = fs::read_to_string(input).unwrap();

    let doc = roxmltree::Document::parse(&contents).unwrap();
}
