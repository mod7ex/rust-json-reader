use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
        "article": "How to work with JSON in rust",
        "author": "Mourad EL CADI",
        "paragraph": [
            { "name": "Para number 1" },
            { "name": "Para number 2" },
            { "name": "Para number 3" },
            { "name": "Para number 4" }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);

    println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[0].name);
}

fn read_json_typed(raw_json: &str) -> Article {
    serde_json::from_str(raw_json).unwrap() 
}