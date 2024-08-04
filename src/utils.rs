use nanoid::nanoid;

pub fn generate_url_id(base: &str) -> String {
    let mut alphabet: Vec<char> = ('0'..='9').collect();
    alphabet.extend('a'..='z');

    println!("Alphabet is {:?}", alphabet);

    let unique_part: String = nanoid!(8, &alphabet);

    let mut url_id: String = base
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-");

    println!("{url_id}");

    url_id.push('-');
    url_id.extend(unique_part.chars());
    url_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_url_id() {
        assert!(generate_url_id("Hello - World").starts_with("hello-world-"));
    }
}