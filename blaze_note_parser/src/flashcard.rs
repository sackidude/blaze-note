pub enum Flashcard {
    FrontBack(FrontBack),
    Reveal(Reveal),
    UnorderedList(UnorderedList),
    OrderedList(OrderedList),
}

pub struct FrontBack {
    front: String,
    back: String,
}
pub struct Reveal {
    wrapper: String,
    reveal: String,
}
pub struct UnorderedList {
    question: String,
    entries: Vec<String>,
}
pub struct OrderedList {
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
