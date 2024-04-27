use blaze_note_parser::{error, flashcard::Flashcard, parse_flashcards};

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

#[test]
fn all_in_one() {
    let flashcards =
        parse_flashcards("{{a|b}}{{c||d||e}}{{f|>1.g2.h}}").expect("failed to parse document");
    assert_eq!(flashcards.len(), 3);

    println!("{:?}", flashcards);

    let card = &flashcards[0];
    if let Flashcard::FrontBack(card) = card {
        assert_eq!(card.front(), "a");
        assert_eq!(card.back(), "b");
    } else {
        panic!("incorrect card type");
    }

    let card = &flashcards[1];
    if let Flashcard::Reveal(card) = card {
        assert_eq!(card.before(), "c");
        assert_eq!(card.reveal(), "d");
        assert_eq!(card.after(), "e");
    } else {
        panic!("incorrect card type");
    }

    let card = &flashcards[2];
    if let Flashcard::OrderedList(card) = card {
        assert_eq!(card.question(), "f");
        let entries = card.entries();
        assert_eq!(entries[0], "g");
        assert_eq!(entries[1], "h");
    } else {
        panic!("incorrect card type");
    }
}

#[test]
fn err_empty_card() {
    let err = parse_flashcards("{{}}").expect_err("didn't get error");
    if let error::Error::EmptyCard = err {
        ()
    } else {
        panic!("expected error: EmptyCard. Got error: {}", err);
    }
}

#[test]
fn err_unclosed_brackets() {
    let err = parse_flashcards("{{").expect_err("didn't get error");
    if let error::Error::UnclosedBrackets = err {
        ()
    } else {
        panic!("expected error: UnclosedBrackets. Got error: {}", err);
    }
}

#[test]
fn no_opening_brackets() {
    let err = parse_flashcards("}}").expect_err("didn't get error");
    if let error::Error::NoOpeningBrackets = err {
        ()
    } else {
        panic!("expected error: NoOpeningBrackets. Got error: {}", err);
    }
}

#[test]
fn only_bar_front_back_card() {
    let cards = parse_flashcards("{{|}}").expect("failed to parse");
    let card = &cards[0];

    if let Flashcard::FrontBack(card) = card {
        assert_eq!(card.front(), "");
        assert_eq!(card.back(), "");
    } else {
        panic!(
            "Incorrect flashcard type. Expect FrontBack, got: {:?}",
            card
        );
    }
}

#[test]
fn only_bar_reveal_card() {
    let cards = parse_flashcards("{{||||}}").expect("failed to parse");
    let card = &cards[0];

    if let Flashcard::Reveal(card) = card {
        assert_eq!(card.before(), "");
        assert_eq!(card.reveal(), "");
        assert_eq!(card.after(), "");
    } else {
        panic!(
            "Incorrect flashcard type. Expect FrontBack, got: {:?}",
            card
        );
    }
}
