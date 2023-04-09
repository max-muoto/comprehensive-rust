struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        return Library { books: Vec::new() };
    }

    fn len(&self) -> usize {
        return self.books.len();
    }

    fn is_empty(&self) -> bool {
        return self.books.len() == 0;
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }

    fn oldest_book<'a>(&'a self) -> Option<&'a Book> {
        let mut oldest_book: Option<&'a Book> = None;
        for book in &self.books {
            if oldest_book.is_none() || book.year < oldest_book.unwrap().year {
                oldest_book = Some(book);
            }
        }
        oldest_book
    }
}

// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
fn main() {
    let mut library = Library::new();

    println!("Our library is empty: {}", library.is_empty());

    let favorite_book = Book::new("Lord of the Rings", 1954);
    println!("Our favorite book {favorite_book} should go in the library");
    library.add_book(favorite_book);
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("My oldest book is {book}"),
        None => println!("My library is empty!"),
    }

    println!("Our library has {} books", library.len());
    for book in library.books {
        println!("{book}");
    }
}

#[test]
fn test_library_len() {
    let mut library = Library::new();
    assert_eq!(library.len(), 0);
    assert!(library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    assert_eq!(library.len(), 2);
    assert!(!library.is_empty());
}

#[test]
fn test_library_is_empty() {
    let mut library = Library::new();
    assert!(library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    assert!(!library.is_empty());
}

#[test]
fn test_library_print_books() {
    let mut library = Library::new();
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    // We could try and capture stdout, but let us just call the
    // method to start with.
    library.print_books();
}

#[test]
fn test_library_oldest_book() {
    let mut library = Library::new();
    assert!(library.oldest_book().is_none());

    library.add_book(Book::new("Lord of the Rings", 1954));
    assert_eq!(
        library.oldest_book().map(|b| b.title.as_str()),
        Some("Lord of the Rings")
    );

    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    assert_eq!(
        library.oldest_book().map(|b| b.title.as_str()),
        Some("Alice's Adventures in Wonderland")
    );
}
