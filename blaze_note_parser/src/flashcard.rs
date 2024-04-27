pub enum Flashcard {
    FrontBack(FrontBack),
    Reveal(Reveal),
    OrderedList(List),
}

pub struct FrontBack {
    front: String,
    back: String,
}

impl FrontBack {
    pub fn new(front: String, back: String) -> Self {
        FrontBack { front, back }
    }
    pub fn front(&self) -> &str {
        &self.front
    }
    pub fn back(&self) -> &str {
        &self.back
    }
}
pub struct Reveal {
    before: String,
    reveal: String,
    after: String,
}

impl Reveal {
    pub fn new(before: String, reveal: String, after: String) -> Self {
        Reveal {
            before,
            reveal,
            after,
        }
    }

    pub fn before(&self) -> &str {
        &self.before
    }

    pub fn reveal(&self) -> &str {
        &self.reveal
    }

    pub fn after(&self) -> &str {
        &self.after
    }
}
pub struct List {
    question: String,
    entries: Vec<String>,
}

impl List {
    pub fn new(question: String, entries: Vec<String>) -> Self {
        List { question, entries }
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }
}

#[derive(Debug)]
pub(crate) enum FlashcardTypes {
    FrontBack,
    Reveal,
    OrderedList,
}
