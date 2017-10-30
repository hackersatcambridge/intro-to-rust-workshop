use std::cell::Cell;

/// Log message type, can be one of the variants defined here.
///
/// Trailing commas are OK (encouraged in fact!)
enum LogMessage {
    HelloWorld,
    String(String),
    RegisterUser(u32, String),
}

/// Define an interface
trait Logger {
    /// ...that has a method log() borrowing the logger object and taking
    /// a message.
    fn log(&self, message: LogMessage);
}

/// This is our actual object class
struct LogPrinter {
    /// Cell is needed to provide inner mutability, so an immutable borrow
    /// of LogPrinter can still increment the message ID.
    message_id: Cell<u32>,

    /// The last `LogMessage::RegisterUser` ID that was seen.
    last_user_registration: Cell<u32>,
}

impl LogPrinter {
    /// Implementation of the constructor for LogPrinter.
    ///
    /// There's nothing special about a constructor, it's just another
    /// function on the class (other languages call "static methods")
    pub fn new() -> LogPrinter {
        LogPrinter {
            message_id: Cell::new(0),
            last_user_registration: Cell::new(0),
        }
    }
}

/// Implementation of Logger for LogPrinter.
///
/// You can implement any trait (even from other crates) on your types,
/// or your traits on any type from other crates.
impl Logger for LogPrinter {
    fn log (&self, message: LogMessage) {
        // id will be of type u32, but we don't need to explicitly say that
        let id = self.message_id.get();

        // Match on the message, deconstructing any contained parameters
        match message {
            LogMessage::HelloWorld => println!("{}: Hello, world!", id),
            LogMessage::String(s) => println!("{}: {}", id, s),
            LogMessage::RegisterUser(user_id, name) => {
                println!("{}: User {} registered", id, name);
                self.last_user_registration.set(user_id);
            },
        }

        self.message_id.set(id + 1);
    }
}

fn main() {
    let logger = LogPrinter::new();
    logger.log(LogMessage::HelloWorld);
    // A literal string is a `&str`, turn it into an owned `String` with
    // `.into()` or `.to_owned()`
    logger.log(LogMessage::String("I can log strings too".into()));
    logger.log(LogMessage::RegisterUser(42, "Robin".to_owned()));
}
