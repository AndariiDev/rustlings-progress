use std::string;

// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue"); // literal string is a &str

    string("red".to_string()); // because .to_string produces a string, transforming slice to string here

    string(String::from("hi")); // Make String from string slice with String::from

    string("rust is fun!".to_owned()); // Only strings can be owned

    string("nice weather".into()); // idk, does .into transform a string into a slice?

    string(format!("Interpolation {}", "Station")); // String formatting, clearly; format! creates a string

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]); // borrowed String = string_slice?

    string_slice("  hello there ".trim()); // .trim is only for string_slices, taking an &str and returning an &str slice

    string("Happy Monday!".replace("Mon", "Tues")); // Clearly a string; .replace returns a string.

    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // Again, clearly a string, just reformatted
}
