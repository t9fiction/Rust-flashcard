use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use std::fs::{self};
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
        Flashcard {
            question,
            answer,
            category,
        }
    }
}

struct FlashcardCategories {
    categories: HashMap<String, Vec<Flashcard>>,
}

impl FlashcardCategories {
    fn new() -> Self {
        FlashcardCategories {
            categories: HashMap::new(),
        }
    }

    fn add_new_category(&mut self, category: String) {
        let category_lower = category.to_lowercase();
        if self.categories.contains_key(&category_lower) {
            println!("Category '{}' already exists!", category);
        } else {
            self.categories.insert(category_lower, Vec::new());
            println!("Category '{}' added successfully.", category);
        }
    }

    fn remove_category(&mut self, category: String) {
        let category_lower = category.to_lowercase();
        if self.categories.remove(&category_lower).is_some() {
            println!("Category '{}' removed successfully.", category);
        } else {
            println!("Category '{}' does not exist!", category);
        }
    }

    fn list_categories(&self) {
        if self.categories.is_empty() {
            println!("No categories found!");
        } else {
            for category in self.categories.keys() {
                println!("{}", category.to_uppercase());
            }
        }
    }

    fn add_flashcard(&mut self, question: String, answer: String, category: String) {
        let category_lower = category.to_lowercase();
        let flashcard = Flashcard::new(question, answer, category_lower.clone());

        // If the category doesn't exist, create it
        self.categories
            .entry(category_lower)
            .or_insert_with(Vec::new)
            .push(flashcard);

        println!("Flashcard added to the category.");
    }

    fn remove_flashcard(&mut self, category: String, question: String) {
        let category_lower = category.to_lowercase();
        if let Some(flashcards) = self.categories.get_mut(&category_lower) {
            let len_before = flashcards.len();
            flashcards.retain(|flashcard| flashcard.question != question);
            if len_before == flashcards.len() {
                println!("No flashcard found with the question '{}'.", question);
            } else {
                println!("Flashcard removed from category '{}'.", category);
            }
        } else {
            println!("Category '{}' not found.", category);
        }
    }

    fn display_flashcards(&self, category: &str) {
        let category_lower = category.to_lowercase();
        match self.categories.get(&category_lower) {
            Some(flashcards) => {
                if flashcards.is_empty() {
                    println!("No flashcards found in category '{}'.", category);
                } else {
                    for (index, flashcard) in flashcards.iter().enumerate() {
                        println!("{}. Question: {}", index + 1, flashcard.question);
                        println!("   Answer: {}", flashcard.answer);
                        println!("------------------------");
                    }
                }
            }
            None => {
                println!("Category '{}' not found.", category);
            }
        }
    }

    fn quiz_user(&self, category: &str) {
        let category_lower = category.to_lowercase();
        if let Some(flashcards) = self.categories.get(&category_lower) {
            let mut correct_answers = 0;
            let mut wrong_answers = 0;

            // Shuffle flashcards randomly
            let mut flashcards = flashcards.clone();
            let mut rng = rand::thread_rng();
            flashcards.shuffle(&mut rng);

            for flashcard in flashcards {
                println!("\nQuestion: {}", flashcard.question);
                print!("Your answer: ");
                io::stdout().flush().unwrap();

                let mut user_answer = String::new();
                io::stdin()
                    .read_line(&mut user_answer)
                    .expect("Failed to read line");

                if user_answer.trim().to_lowercase() == flashcard.answer.to_lowercase() {
                    println!("Correct!");
                    correct_answers += 1;
                } else {
                    wrong_answers += 1;
                    println!("Incorrect. The correct answer is: {}", flashcard.answer);
                }
            }

            println!(
                "\nQuiz finished! You got {} correct and {} incorrect.",
                correct_answers, wrong_answers
            );
        } else {
            println!("Category '{}' not found.", category);
        }
    }

    // fn get_flashcards(&self) -> Vec<Flashcard> {
    //     self.categories
    //         .values()
    //         .flat_map(|flashcards| flashcards.clone())
    //         .collect()
    // }

    fn load_from_file(&mut self, file_name: &str) {
        match fs::read_to_string(file_name) {
            Ok(contents) => {
                let categories: HashMap<String, Vec<Flashcard>> =
                    serde_json::from_str(&contents).expect("Failed to parse categories from file");
                self.categories = categories;
                println!("Categories and flashcards loaded from file: {}", file_name);
            }
            Err(err) => {
                println!(
                    "Error reading file ({}). Starting with an empty flashcard set.",
                    err
                );
            }
        }
    }

    fn save_to_file(&self, file_name: &str) {
        let json = serde_json::to_string(&self.categories).expect("Failed to serialize categories");
        fs::write(file_name, json).expect("Failed to write categories to file");
        println!("Categories and flashcards saved to file: {}", file_name);
    }
}

fn main() {
    let file_name = "flashcards.json";
    let mut flashcard_categories = FlashcardCategories::new();
    let mut is_admin = false;

    // Load flashcards at the start
    flashcard_categories.load_from_file(file_name);

    loop {
        if is_admin {
            println!("\nAdmin Menu:");
            println!("1. Add a new category");
            println!("2. Remove a category");
            println!("3. List all categories");
            println!("4. Add a flashcard");
            println!("5. Remove a flashcard");
            println!("6. Display flashcards in a category");
            println!("7. Quiz yourself");
            println!("8. Save flashcards to file");
            println!("9. Change to user mode");
            println!("0. Exit");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: u32 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            };

            match choice {
                1 => {
                    println!("Enter the name of the new category: ");
                    let mut category = String::new();
                    io::stdin()
                        .read_line(&mut category)
                        .expect("Failed to read input");
                    flashcard_categories.add_new_category(category.trim().to_string());
                }
                2 => {
                    println!("Enter the name of the category to remove: ");
                    let mut category = String::new();
                    io::stdin()
                        .read_line(&mut category)
                        .expect("Failed to read input");
                    flashcard_categories.remove_category(category.trim().to_string());
                }
                3 => {
                    flashcard_categories.list_categories();
                }
                4 => {
                    println!("Enter the category: ");
                    let mut category = String::new();
                    io::stdin()
                        .read_line(&mut category)
                        .expect("Failed to read input");

                    println!("Enter the question: ");
                    let mut question = String::new();
                    io::stdin()
                        .read_line(&mut question)
                        .expect("Failed to read input");

                    println!("Enter the answer: ");
                    let mut answer = String::new();
                    io::stdin()
                        .read_line(&mut answer)
                        .expect("Failed to read input");

                    flashcard_categories.add_flashcard(
                        question.trim().to_string(),
                        answer.trim().to_string(),
                        category.trim().to_string(),
                    );
                }
                5 => {
                    println!("Enter the category: ");
                    let mut category = String::new();
                    io::stdin()
                        .read_line(&mut category)
                        .expect("Failed to read input");

                    println!("Enter the question of the flashcard to remove: ");
                    let mut question = String::new();
                    io::stdin()
                        .read_line(&mut question)
                        .expect("Failed to read input");
                    flashcard_categories
                        .remove_flashcard(category.trim().to_string(), question.trim().to_string());
                }
                6 => {
                    println!("Enter the category: ");
                    let mut category = String::new();
                    io::stdin()
                        .read_line(&mut category)
                        .expect("Failed to read input");

                    flashcard_categories.display_flashcards(category.trim());
                }
                7 => {
                    println!("Enter the category for quiz: ");
                    let mut category = String::new();
                    io::stdin()
                        .read_line(&mut category)
                        .expect("Failed to read input");

                    flashcard_categories.quiz_user(category.trim());
                }
                8 => {
                    flashcard_categories.save_to_file(file_name);
                }
                9 => {
                    is_admin = false;
                    println!("Switched to user mode.");
                }
                0 => {
                    flashcard_categories.save_to_file(file_name);
                    println!("Goodbye!");
                    break;
                }
                _ => {
                    println!("Invalid choice! Please enter a number between 0 and 9.");
                }
            }
        } else {
            println!("\nUser Menu:");
            println!("1. List all categories");
            println!("2. Quiz yourself");
            println!("3. Change to user mode");
            println!("0. Exit");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: u32 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            };

            match choice {
                1 => {
                    flashcard_categories.list_categories();
                }
                2 => {
                    println!("Enter the category for quiz: ");
                    let mut category = String::new();
                    io::stdin()
                        .read_line(&mut category)
                        .expect("Failed to read input");

                    flashcard_categories.quiz_user(category.trim());
                }
                3 => {
                    is_admin = true;
                    println!("Switched to admin mode.");
                }
                0 => {
                    flashcard_categories.save_to_file(file_name);
                    println!("Goodbye!");
                    break;
                }
                _ => {
                    println!("Invalid choice! Please enter a number between 0 and 2.");
                }
            }
        }
    }
}
