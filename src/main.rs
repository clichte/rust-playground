#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use std::collections::HashMap;

fn main() {
    enum SomeValue {
        StringValue(String),
        IntValue(i32),
    }

    let multi_array: [SomeValue; 4] = [
        SomeValue::StringValue(String::from("one")),
        SomeValue::IntValue(2),
        SomeValue::StringValue(String::from("three")),
        SomeValue::IntValue(4),
    ];

    for i in multi_array {
        match i {
            SomeValue::StringValue(data) => {
                println!("The string is: {}", data);
            },
            SomeValue::IntValue(data) => {
                println!("The int is: {}", data);
            }
        }
    }

    let mut string_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("{:?}", string_vector);
    string_vector.push("four");
    println!("{:?}", string_vector);

    #[derive(Debug)]
    enum CharacterValue {
        Name(String),
        Age(i32),
        Items(Vec<String>)
    }

    let mut profile: HashMap<&str, CharacterValue> = HashMap::new();

    profile.insert("name", CharacterValue::Name("Maxwell".to_string()));
    profile.insert("age", CharacterValue::Age(32));
    profile.insert("items", CharacterValue::Items(vec![
        "laptop".to_string(),
        "book".to_string(),
        "coat".to_string()
    ]));
    println!("{:?}", profile);

    match profile.get("name") {
        Some(value_data) => {
            match value_data {
                CharacterValue::Name(name) => {
                    println!("the name is: {}", name);
                },
                _ => panic!("name should be a string")
            }
        },
        None => {
            println!("name is not present");
        }
    }
}
