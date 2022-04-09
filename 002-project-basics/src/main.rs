use std::io;

fn main() {
    // Testing to print to screen
    println!("Hello World!");
    println!("My name is {} and i am {} years old", "Alex", 29);
    println!("a + b = {}", 3 + 9);
    println!("{0} has a {2} and {0} has a {1}", "Alex", "cat", "dog");
    println!(
        "{firstName} {lastName}",
        firstName = "Alex",
        lastName = "Smith"
    );
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 50, 50, 50);
    println!("Array: {:?}", [1, 2, 3]);

    // Reads user input and writes it to output
    let mut input = String::new();
    println!("Say something");

    // Rust provides pattern matching via the match keyword, which can be used like a C switch. The
    // first matching arm is evaluated and all possible values must be covered.
    //Read all bytes until a newline (the 0xA byte) is reached, and append them to the provided
    //buffer.

    // read_line
    // -----------------------------------
    // This function will read bytes from the underlying stream until the newline delimiter (the 0xA byte)
    // or EOF is found. Once found, all bytes up to, and including, the delimiter (if found) will be
    // appended to buf.
    // If successful, this function will return the total number of bytes read.
    // If this function returns Ok(0), the stream has reached EOF.
    // This function is blocking and should be used carefully: it is possible for an attacker to
    // continuously send bytes without ever sending a newline or EOF.
    //
    // Errors
    // --------------------------------------
    // This function has the same error semantics as read_until and will also return an error if
    // the read bytes are not valid UTF-8. If an I/O error is encountered then buf may contain some
    // bytes already read in the event that all data read so far was valid UTF-8.
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("You said {} which equals the size of {}", input, n);
        }
        Err(e) => {
            println!("Something went wrong {}", e);
        }
    }
}
