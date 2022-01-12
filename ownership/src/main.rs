fn main() {
    let my_string = String::from("Hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // converts string to an array of bytes in order to loop through it

    // iter => returns each elem in a collection.   enumerate => wraps and returns as tuple
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // looks for the bytes that is a space
            return &s[0..i];
        }
    }
    &s[..]
}

// Needs to track a starting and an ending index
fn second_word(s: &String) -> &str {
    let s = String::from("Hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    // &s[0..2] = &s[..2]    &s[0..len] = &s[0..]

    world
}

fn borrowing_void() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");
    // Must return the string directly, not the reference because it's deallocated out of the scope
    s
}

fn borrowing() {
    let mut s = String::from("hello");

    // One mutable borrow at a time
    let r1 = &s;
    let r2 = &s;
    println!("{}   {}", r1, r2);

    // Works when the scopes don't overlap
    let r3 = &mut s;

    println!("{}", r3);
}
