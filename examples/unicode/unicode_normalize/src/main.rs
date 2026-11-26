use unicode_normalization::UnicodeNormalization;

fn main() {
    // Create a string with various Unicode characters
    let text = "Hello, 世界! 🌍 Здравствуй, мир!";
    println!("Original text: {text}");

    // Get the length of the string in bytes
    println!("Length in bytes: {}", text.len());

    // Get the length of the string in characters
    println!("Length in characters: {}", text.chars().count());

    // Iterate over characters
    println!("\nIterating over characters:");
    for (i, c) in text.chars().enumerate() {
        println!("Character {} is '{}' (U+{:04X})", i, c, c as u32);
    }

    // Slice the string
    let slice = &text[7..13];
    println!("\nSlice (bytes 7-13): {slice}");

    // Convert to uppercase
    println!("\nUppercase: {}", text.to_uppercase());

    // Check if the string contains a substring
    println!("\nContains '世界': {}", text.contains("世界"));

    // Split the string
    println!("\nSplit by spaces:");
    for word in text.split_whitespace() {
        println!("{word}");
    }

    // Reverse the string
    let reversed: String = text.chars().rev().collect();
    println!("\nReversed: {reversed}");

    // Create a string with combining characters
    let combined = "e\u{0301}";  // 'e' with acute accent
    println!("\nCombined character: {combined}");
    println!("Length of combined character: {}", combined.chars().count());

    // Normalize the string
    let normalized = combined.nfc().collect::<String>();
    println!("Normalized: {normalized}");
    println!("Length of normalized: {}", normalized.chars().count());
}
