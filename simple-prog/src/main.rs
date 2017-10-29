use std::cell::Cell;

/// Define an interface
trait Logger {
    /// ...that has a method log() borrowing the logger object and taking
    /// a message.
    fn log(&self, message: &str);
}

/// This is our actual object class
struct LogPrinter {
    /// Cell is needed to provide inner mutability, so an immutable borrow
    /// of LogPrinter can still increment the message ID.
    message_id: Cell<u32>,
}

impl LogPrinter {
    /// Implementation of the constructor for LogPrinter.
    ///
    /// There's nothing special about a constructor, it's just another
    /// function on the class (other languages call "static methods")
    pub fn new() -> LogPrinter {
        LogPrinter {message_id: Cell::new(0)}
    }
}

/// Implementation of Logger for LogPrinter.
///
/// You can implement any trait (even from other crates) on your types,
/// or your traits on any type from other crates.
impl Logger for LogPrinter {
    fn log (&self, message: &str) {
        // id will be of type u32, but we don't need to explicitly say that
        let id = self.message_id.get();
        println!("{}: {}", id, message);
        self.message_id.set(id + 1);
    }
}

fn main() {
    let logger = LogPrinter::new();
    logger.log("Hello, world!");
    logger.log("I'm writing Rust!");
}
