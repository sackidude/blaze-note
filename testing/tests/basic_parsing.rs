use blaze_note_parser::{flashcard::Flashcard, parse_flashcards};

#[test]
fn front_back() {
    let flashcards = parse_flashcards("{{a|b}}").expect("failed to parse document");
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
fn reveal() {
    let flashcards = parse_flashcards("{{a||b||c}}").expect("failed to parse document");
    assert_eq!(flashcards.len(), 1);
    let card = &flashcards[0];
    if let Flashcard::Reveal(card) = card {
        assert_eq!(card.before(), "a");
        assert_eq!(card.reveal(), "b");
        assert_eq!(card.after(), "c");
    } else {
        panic!("incorrect card type");
    }
}

#[test]
fn list() {
    let flashcards = parse_flashcards("{{a|>1.b2.c}}").expect("failed to parse document");
    assert_eq!(flashcards.len(), 1);
    let card = &flashcards[0];
    if let Flashcard::OrderedList(card) = card {
        assert_eq!(card.question(), "a");
        let entries = card.entries();
        assert_eq!(entries[0], "b");
        assert_eq!(entries[1], "c");
    } else {
        panic!("incorrect card type");
    }
}
