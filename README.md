# Rust Flashcard App

## Overview
This is a simple flashcard application written in Rust. The app allows users to create, manage, and quiz themselves on flashcards. It provides two modes: **Admin mode** (where users can manage categories and flashcards) and **User mode** (where users can only take quizzes and view categories). The flashcards are stored in categories, and all data is saved to a JSON file for persistence across sessions.

## Features
- **Add, Remove, List Categories**: Admin users can manage categories.
- **Add, Remove Flashcards**: Admin users can add/remove flashcards in each category.
- **Quiz Yourself**: Users can take quizzes based on the flashcards in a category.
- **Save and Load**: Flashcards and categories are stored in a JSON file and loaded when the app starts.
- **Category Management**: Admin can manage categories and their flashcards.

## Prerequisites
- Rust 1.56.0 or higher
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/t9fiction/Rust-flashcard.git
cd Rust-flashcard
```

2. Install dependencies and run the app:

```bash
cargo run
```

This will start the application.

## Usage

### Modes

#### Admin Mode
In Admin mode, users can manage categories and flashcards. To switch to Admin mode, choose option **3** in the **User Menu**.

Once in Admin mode, the following actions are available:
1. **Add a new category**: Enter a name for the new category.
2. **Remove a category**: Enter the name of the category to remove.
3. **List all categories**: View all existing categories.
4. **Add a flashcard**: Add a new flashcard to an existing category (question, answer, category).
5. **Remove a flashcard**: Remove a flashcard from a category by specifying the question.
6. **Display flashcards in a category**: View all flashcards in a specific category.
7. **Quiz yourself**: Take a quiz based on flashcards in a category.
8. **Save flashcards to file**: Save current flashcards and categories to a file.
9. **Change to user mode**: Switch to User mode where only quizzes can be taken.

#### User Mode
In User mode, users can:
1. **List all categories**: View a list of all available categories.
2. **Quiz yourself**: Take a quiz based on flashcards from a specific category.
3. **Change to Admin mode**: Switch to Admin mode for full control over categories and flashcards.

### Menu Navigation
The application presents a simple command-line menu system for both Admin and User modes. Select the corresponding number to choose an action.

### File Persistence
Flashcards and categories are saved in a file named `flashcards.json`. Any updates to categories or flashcards will automatically be saved when exiting or choosing the "Save flashcards to file" option.

## Example

**Admin Menu**:
```
Admin Menu:
1. Add a new category
2. Remove a category
3. List all categories
4. Add a flashcard
5. Remove a flashcard
6. Display flashcards in a category
7. Quiz yourself
8. Save flashcards to file
9. Change to user mode
0. Exit
```

**User Menu**:
```
User Menu:
1. List all categories
2. Quiz yourself
3. Change to Admin mode
0. Exit
```

### Example of a Quiz

```text
Quiz finished! You got 3 correct and 2 incorrect.
--------------------------------------------------
```

## Contributing
If you'd like to contribute to this project, feel free to fork the repository and submit a pull request with your changes.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments
- [serde](https://serde.rs/) for serialization and deserialization of data.
- [rand](https://crates.io/crates/rand) for shuffling flashcards randomly during quizzes.

---