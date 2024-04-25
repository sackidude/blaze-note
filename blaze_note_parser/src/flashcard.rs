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

impl FrontBack {
    pub fn get_front(&self) -> &str {
        &self.front
    }
    pub fn get_back(&self) -> &str {
        &self.back
    }
}

impl Reveal {
    pub fn get_wrapper(&self) -> &str {
        &self.wrapper
    }
    pub fn get_reveal(&self) -> &str {
        &self.reveal
    }
}

impl List {
    pub fn get_question(&self) -> &str {
        &self.question
    }
    pub fn get_reveal(&self) -> &[String] {
        &self.entries
    }
}
