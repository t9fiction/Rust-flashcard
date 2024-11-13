use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

// Define Flashcard struct
#[derive(Serialize, Deserialize, Clone)]
struct Flashcard {
    question: String,
    answer: String,
    category: String,
}

impl Flashcard {
    fn new(question: String, answer: String, category: String) -> Self {
        Flashcard { question, answer, category }
    }
}

struct FlashcardCategories {
    categories: HashMap<String, Vec<Flashcard>>,
}

impl FlashcardCategories {
    fn new() -> Self {
        FlashcardCategories {categories: HashMap::new()}
    }

    fn add_new_category(&mut self, category: String) {
        let category_lower = category.to_lowercase();
        if self.categories.contains_key(&category_lower) {
            println!("Category {} already exists!", category);
            return;
        }else {
            self.categories.insert(category_lower, Vec::new());
        }
    }

}

fn main() {
    println!("Hello, world!");
}
