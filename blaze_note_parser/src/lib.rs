struct Card {}

pub struct Note {
    html: String,
    cards: Vec<Card>,
}

pub fn parse(document: &str) -> Result<Note, ()> {
    Ok(Note {
        html: document.into(),
        cards: vec![],
    })
}
