# Flashcard App for Learning Rust

Welcome to the **Rust Flashcard App**! This app is designed to help you learn and reinforce your knowledge of Rust programming through interactive flashcards. Whether you're a beginner or someone looking to solidify your Rust skills, this app provides a quick and effective way to practice key Rust concepts.

## Features

- **Interactive Flashcards**: Browse through a collection of flashcards covering various Rust topics.
- **Category-Based Organization**: Categories help organize flashcards by topics such as syntax, ownership, memory safety, and more.
- **Results Tracking**: Track your quiz results and see your progress.
- **Responsive Design**: The app is designed to be user-friendly, with responsive layouts to work on different screen sizes.
- **Simple UI/UX**: Easy to navigate through flashcards, with a clean design for focused learning.

## Installation

To run this app locally, follow the instructions below.

### Prerequisites

- Node.js and npm (or yarn) installed on your system.
- Basic understanding of HTML, CSS, JavaScript, and React.

### Steps to Run Locally

1. **Clone the repository:**

   ```bash
   git clone https://github.com/t9fiction/Rust-flashcard.git
   ```

2. **Navigate into the project directory:**

   ```bash
   cd Rust-flashcard
   ```

3. **Install dependencies:**

   Using npm:

   ```bash
   npm install
   ```

   Or using yarn:

   ```bash
   yarn install
   ```

4. **Run the app:**

   ```bash
   npm start
   ```

   Or with yarn:

   ```bash
   yarn start
   ```

   The app will open in your browser at `http://localhost:3000`.

## Usage

- **Categories**: The flashcards are divided into categories. You can view and select different categories like "Rust Basics", "Ownership", "Lifetimes", etc.
- **Flashcards**: Each flashcard displays a question about Rust, and you can click to reveal the answer. 
- **Quiz Mode**: After going through the flashcards, you can test your knowledge by taking a quiz. The quiz will display a series of questions, and you will select the correct answer.
- **Results**: Once you finish the quiz, your score and correct/incorrect answers will be displayed.

## Example Flashcard

### Question:
**What is the difference between `String` and `&str` in Rust?**

### Answer:
- `String`: A growable, heap-allocated string that is owned and mutable.
- `&str`: A string slice, which is a reference to a string, usually immutable.

## Contributing

If you'd like to contribute to this project, follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-name`).
3. Make your changes.
4. Commit your changes (`git commit -m 'Add feature'`).
5. Push to your branch (`git push origin feature-name`).
6. Create a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
