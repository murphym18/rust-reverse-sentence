fn main() {
    let examples = vec![
        "michael likes rust",
        "",
        "1",
    ];

    for s in examples {
        let sentence = String::from(s);
        println!("'{}' -> '{}'", &sentence, reverse_sentence(&sentence));
    }
}

fn reverse_sentence(tmp: &String) -> String {
    let v: Vec<&str> = tmp.split_whitespace().rev().collect();
    let mut result = String::new();
    for word in v {
        result.push_str(word);
        result.push(' ');
    }
    let _ = result.pop();
    result
}
