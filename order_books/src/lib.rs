// Declare the library module and its submodules
pub mod library {
    pub mod writers {
        use super::books::Book;

        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>,
        }
    }

    pub mod books {
        pub struct Book {
            pub title: String,
            pub year: u64,
        }
    }
}

use library::books::Book;
use library::writers::Writer;

/// Sorts the writer's books alphabetically by title
pub fn order_books(writer: &mut Writer) {
    writer.books.sort_by(|a, b| a.title.cmp(&b.title));
}
