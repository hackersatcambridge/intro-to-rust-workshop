//! Enable some nightly-only compiler features, and the rocket_codegen plugin
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

/// Return a literal string, to satisfy the compiler we need to tell it that
/// the lifetime of the string is static (lives forever).
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Return a string that we construct, so it is an owned `String`.
#[get("/hello/<name>")]
fn hello(name: String) -> String {
    // as_ref() is needed to compare against literal strings, which are &str
    match name.as_ref() {
        "robin" => {
            format!("Execute order 66.")
        },
        other => {
            format!("Hello, {}!", other)
        }
    }
}

#[get("/fizzbuzz/<n>")]
fn fizzbuzz(n: String) -> String {
    // .parse() returns a Result, we match on the result and can respond to
    // errors
    match n.parse::<u32>() {
        Ok(number) => {
            // Create a range from 1 to n (inclusive, the +1 is because Rust
            // ranges are exclusive and so won't include n itself)
            (1..number+1).map(|x| {
                // Now we map each number to the FizzBuzz'd equivalent String
                if x % 15 == 0 {
                    format!("FizzBuzz")
                } else if x % 3 == 0 {
                    format!("Fizz")
                } else if x % 5 == 0 {
                    format!("Buzz")
                } else {
                    format!("{}", x)
                }
            }).fold(String::new(), |string, fizzbuzz| {
                // Finally, fold the sequence of strings into a single String
                string + &fizzbuzz + "\n"
            })
        },
        Err(_) => {
            format!("Not a number!")
        }
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, fizzbuzz])
        .launch();
}
