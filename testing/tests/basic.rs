use blaze_note_parser::{flashcard::Flashcard, parse_flashcards};

#[test]
fn basic_card_parsing() {
    let note_str = String::from("# {{Hello,| World!}}");
    let flashcards = parse_flashcards(&note_str).expect("failed to parse document");
    assert_eq!(flashcards.len(), 1);
    let card = &flashcards[0];
    if let Flashcard::FrontBack(card) = card {
        assert_eq!(card.front(), "Hello,");
        assert_eq!(card.back(), " World!");
    } else {
        panic!("incorrect card type");
    }
    //TODO! Add more here
}
