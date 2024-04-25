use crate::{
    error::{Error, Result},
    flashcard::Flashcard,
};

pub fn parse_to_cards(document: &str) -> Result<Vec<Flashcard>> {
    // First seperate the document into all of the individual cards and then
    // parse them.
    let mut last_char: char = '\0';
    let mut in_card = false;
    for c in document.chars() {
        match c {
            '{' => {
                // This is a new opening to a card and your aren't already in
                // a card.
                if last_char == '{' && !in_card {
                    in_card = true;
                }
            }
            '}' => {
                if last_char == '}' && in_card {
                    if in_card {
                        in_card = false;
                    } else {
                        // TODO!: Think about this more.
                        return Err(Error::InvalidNoteSyntax);
                    }
                }
            }
            _ => (),
        }
        last_char = c;
    }

    let cards: Vec<Flashcard> = vec![];

    Ok(cards)
}

#[test]
fn basic() {}

#[test]
fn utf8() {}
