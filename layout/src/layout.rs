// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

/**
Display
*/
// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

pub fn do_layout() {
    println!("Hello, world!");

    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));


    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // Pretty print
    println!("{:#?}", peter);
}