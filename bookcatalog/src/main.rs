use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not create file '{}'. {}", filename, e);
            return;
        }
    };
    for book in books {
        if let Err(e) = writeln!(file, "{},{},{}", book.title, book.author, book.year) {
            eprintln!("Error: Could not write book '{}' to file. {}", book.title, e);
            return;
        }
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not open file '{}'. {}", filename, e);
            return Vec::new();
        }
    };

    let reader = BufReader::new(file);
    let mut books = Vec::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line from file: {}", e);
                continue;
            }
        };

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let title = parts[0].trim().to_string();
            let author = parts[1].trim().to_string();

            let year = match parts[2].trim().parse::<u16>() {
                Ok(y) => y,
                Err(e) => {
                    eprintln!("Error parsing year '{}': {}", parts[2].trim(), e);
                    continue;
                }
            };

            books.push(Book { title, author, year });
        } else {
            eprintln!("Skipping malformed line (expected 3 fields): {}", line);
        }
    }

    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "A Game of Thrones".to_string(), author: "George R. R. Martin".to_string(), year: 1996 },
    ];

    let filename = "books.txt";

    save_books(&books, filename);
    println!("Books saved to '{}'.", filename);
    println!("---");

    let loaded_books = load_books(filename);
    println!("Successfully loaded {} books from file:", loaded_books.len());

    for book in loaded_books {
        println!("  Title: \"{}\", Author: {}, Year: {}", book.title, book.author, book.year);
    }
}
