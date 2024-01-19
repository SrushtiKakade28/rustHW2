use std::io;

// Define the WordCounter struct
struct WordCounter {
    text: String,
}

// Implementation of WordCounter
impl WordCounter {
    // Constructor to create a new WordCounter instance
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: String::from(text),
        }
    }

    // Function to count words in the text
    fn count_words(&self) -> usize {
        // Split the text by whitespace characters and count the elements
        self.text.split_whitespace().count()
    }
}

fn main() {
    // Prompt user for text input
    println!("Enter a text to count words:");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read line");

    // Trim leading and trailing whitespaces
    let trimmed_text = input_text.trim();

    // Check if the text is empty
    if trimmed_text.is_empty() {
        println!("Error: Empty input. Please enter some text.");
        return;
    }

    // Create a WordCounter instance
    let word_counter = WordCounter::new(trimmed_text);

    // Call count_words and print the result
    let word_count = word_counter.count_words();
    println!("Word count: {}", word_count);
}
