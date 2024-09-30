// This is a simple example of a HashMap in rust

use std::collections::HashMap;

fn main() {
	let mut book_reviews = HashMap::<String, String>::new();
	book_reviews.insert(
		"Adventures of Huckleberry Finn".to_string(),
		"My favorite book.".to_string(),
	);
	book_reviews.insert(
		"Grimms' Fairy Tales".to_string(),
		"Masterpiece.".to_string(),
	);
	if !book_reviews.contains_key("Les Misérables") {
		println!("We've got {} reviews, but Les Misérables ain't one.",
            book_reviews.len());
	}
	book_reviews.remove("The Adventures of Sherlock Holmes");
	let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
	for &book in &to_find {
		match book_reviews.get(book) {
			Some(review) => println!("{book}: {review}"),
			None => println!("{book} is unreviewed.")
		}
	}
	println!("Review for Jane: {}", book_reviews["Grimms' Fairy Tales"]);
	for (book, review) in &book_reviews {
		println!("{book}: \"{review}\"");
	}
}
