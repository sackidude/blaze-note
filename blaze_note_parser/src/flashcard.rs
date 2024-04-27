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

pub(crate) struct FlashcardBuilder {}

impl FlashcardBuilder {
    pub fn new() -> Self {
        FlashcardBuilder {}
    }

    pub fn build(self) -> Flashcard {
        Flashcard::FrontBack(FrontBack {
            front: "hello".into(),
            back: "world".into(),
        })
    }
}
