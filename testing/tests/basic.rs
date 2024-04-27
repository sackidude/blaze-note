use std::str::CharIndices;

use blaze_note_parser::{flashcard::Flashcard, parse_flashcards};

#[test]
fn basic_front_back_card_parsing() {
    let note_str = String::from("{{a|b}}");
    let flashcards = parse_flashcards(&note_str).expect("failed to parse document");
    assert_eq!(flashcards.len(), 1);
    let card = &flashcards[0];
    if let Flashcard::FrontBack(card) = card {
        assert_eq!(card.front(), "a");
        assert_eq!(card.back(), "b");
    } else {
        panic!("incorrect card type");
    }
}

#[test]
fn basic_reveal_parsing() {
    let note_str = String::from("{{a||b||c}}");
    let flashcards = parse_flashcards(&note_str).expect("failed to parse document");
    assert_eq!(flashcards.len(), 1);
    let card = &flashcards[0];
    if let Flashcard::Reveal(card) = card {
        assert_eq!(card.before(), "a");
        assert_eq!(card.reveal(), "a");
        assert_eq!(card.after(), "a");
    } else {
        panic!("incorrect card type");
    }
}
