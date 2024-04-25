use std::fs::read_to_string;

#[test]
fn main() {
    let hello_str: String = read_to_string("documents/basic.md")
        .unwrap()
        .parse()
        .unwrap();
    let _hello = blaze_note_parser::parse(&hello_str).unwrap();
}
