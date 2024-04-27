pub enum Flashcard {
    FrontBack(FrontBack),
    Reveal(Reveal),
    OrderedList(List),
}

pub struct FrontBack {
    front: String,
    back: String,
}
pub struct Reveal {
    wrapper: String,
    reveal: String,
}
pub struct List {
    question: String,
    entries: Vec<String>,
}

// I don't love derive Clone and Copy, but removing it leads to a big fight with
// the borrow checker that I don't feel like doing right now
#[derive(Clone, Copy)]
pub(crate) struct FlashcardBuilder {
    card_type: Option<FlashcardTypes>,
}

#[derive(Clone, Copy)]
pub(crate) enum FlashcardTypes {
    FrontBack,
    Reveal,
    OrderedList,
}

impl FlashcardBuilder {
    pub fn new() -> Self {
        FlashcardBuilder { card_type: None }
    }

    pub fn build(self) -> crate::error::Result<Flashcard> {
        if let Some(card_type) = self.card_type {
            return Ok(match card_type {
                FlashcardTypes::FrontBack => Flashcard::FrontBack(FrontBack {
                    front: "Hello".into(),
                    back: "World".into(),
                }),
                FlashcardTypes::Reveal => Flashcard::Reveal(Reveal {
                    wrapper: "Hello".into(),
                    reveal: "World".into(),
                }),
                FlashcardTypes::OrderedList => Flashcard::OrderedList(List {
                    question: "Hello".into(),
                    entries: vec!["World".into()],
                }),
            });
        }
        Err(crate::error::Error::EmptyCard)
    }

    pub fn card_type(mut self, card_type: FlashcardTypes) -> Self {
        self.card_type = Some(card_type);
        self
    }
}
