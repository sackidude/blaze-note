use blaze_note_parser::parse_flashcards;

#[test]
fn basic_card_parsing() {
    let note_str = String::from("# {{Hello,| World!}}");
    let flashcards = parse_flashcards(&note_str).expect("failed to parse document");
    assert_eq!(flashcards.len(), 1);

    //TODO! Add more here
}
