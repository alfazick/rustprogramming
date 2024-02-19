fn working_with_vectors() {
    // Creating a vector
    let mut nums: Vec<i32> = Vec::new();
    nums.push(5);
    nums.push(6);
    nums.push(7);
    nums.push(8);

    // Accessing vector elements
    if let Some(last) = nums.get(3) {
        println!("Last element is {:?}", last);
    }

    // Iterating over vector values
    for num in &nums {
        println!("Num: {}", num);
    }

    // Using enums to store different types
    #[derive(Debug)]
    enum MyData {
        Int(i32),
        Text(String),
        Vector(Vec<i32>),
    }

    let mut mix: Vec<MyData> = Vec::new();
    mix.push(MyData::Int(100));
    mix.push(MyData::Text("Hello World".to_string()));
    mix.push(MyData::Vector(vec![1, 2, 5]));

    for elem in &mix {
        println!("Element: {:?}", elem);
    }
}

fn understanding_strings() {
    // Strings are mutable
    let mut s = String::new();
    s.push_str("hello");
    s.push('z');
    println!("Updated string: {}", s);

    // Concatenation
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let s3 = s1 + &s2; // Note: s1 has been moved here and can no longer be used
    println!("Concatenated string: {}", s3);

    // Iterating over characters
    let hello = String::from("Здравствуйте");
    for c in hello.chars() {
        println!("{}", c);
    }

    // Bytes iteration
    for byte in hello.bytes() {
        println!("{}", byte);
    }
}

fn hashmap_entry_concept(){
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_names = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 20];

    for name in team_names {
        match scores.entry(name) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(initial_scores[0]);
                println!("Inserted for new team.");
            },
            std::collections::hash_map::Entry::Occupied(mut e) => {
                *e.get_mut() += initial_scores[1];
                println!("Updated existing team's score.");
            },
        }
    }

    println!("{:?}", scores);
}

fn utilizing_hashmaps() {

    use std::collections::HashMap;

    let mut populations: HashMap<String, i32> = HashMap::new();
    populations.insert("UTRGV".to_string(), 30000);
    populations.insert("Edinburg".to_string(), 97000);

    // Accessing values
    if let Some(pop) = populations.get("UTRGV") {
        println!("UTRGV population: {}", pop);
    }

    // Iterating over key-value pairs
    for (key, value) in &populations {
        println!("{}: {}", key, value);
    }

    // Updating a value
    *populations.entry("UTRGV".to_string()).or_insert(0) = 45000;
    println!("{:?}", populations);

    // Word count example
    let text = "hello world hello";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count);
}



fn main() {
    working_with_vectors();
    understanding_strings();
    hashmap_entry_concept();
    utilizing_hashmaps();
}

// time to leetcode
