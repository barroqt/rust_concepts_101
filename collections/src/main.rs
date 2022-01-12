fn main() {
    {
        let _v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no 3rd element"),
    }

    // two ways to reference an element
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // Access vector in turn 
    let mut v = vec![100, 32, 57];

    // Loop through
    // for i in &v {
    //     println!("{}", i);
    // }

    // iterate over mutable reference allows to make changes
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // Use struct or enum to store different types of variables in vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings

    let _s = String::new();
    let data = "initial comments";

    let mut s = data.to_string(); // also works on literal strings
    // Can use String::from to create from literal string

    s.push_str("bar"); // puts "bar" at the end of the string

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // The + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String {}

    // formattable also like this:
    // let _s = format!("{}-{}-{}", s1, s2, s3);

    // it's not possible to access the index of a String
    // let s1 = String::from("hello");
    // let h = s1[0];
    // Does not compile
    
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    } // Access specific byte of string


    // Hash maps

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // <_, _> needed because possible to collect many different data types
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect(); 
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // iterate over key/value
    for (key, value) in &scores {
        println!{"{}: {}", key, value};
    }

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Score is replaced

    scores.entry(String::from("Yellow")).or_insert(50); // entry API checks whether there is a value associated to the key

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // returns a mutable reference (&mut V) to the value for this key
        *count += 1; // we must dereference count because a mutable variable is stored in count
    }

    println!("{:?}", map);
}
