use std::fs::read_to_string;

use blaze_note_parser::{flashcard::Flashcard, parse};

#[test]
fn main() {
    let note_str: String = read_to_string("documents/basic.md")
        .expect("failed to read file")
        .parse()
        .expect("failed to parse read file");

    let note = parse(&note_str).expect("failed to parse document");
    if let Flashcard::FrontBack(card) = &note.get_cards()[0] {
        assert_eq!(card.get_front(), "Hello,");
        assert_eq!(card.get_back(), "_World_!");
    } else {
        panic!("incorrect flashcard type")
    }
}
