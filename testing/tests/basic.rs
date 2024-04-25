use std::fs::read_to_string;

#[test]
fn main() {
    let hello_str: String = read_to_string("../documents/basic.md")
        .unwrap()
        .parse()
        .unwrap();
    let hello = blaze_note_parser::parse(&hello_str).expect("Failed to parse note: {}");
}
