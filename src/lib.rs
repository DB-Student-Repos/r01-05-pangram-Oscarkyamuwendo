/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    // Initialize a boolean array to keep track of each letter occurrence
    let mut seen = [false; 26];

    // Iterate over each character in the sentence
    for c in sentence.chars() {
        // Convert the character to lowercase and check if it's a letter
        if let Some(index) = c.to_ascii_lowercase().to_digit(36).map(|d| d as usize - 10) {
            // If it's a letter, mark it as seen
            seen[index] = true;
        }
    }

    // Check if all letters have been seen at least once
    seen.iter().all(|&b| b)
}
