use nanoid::nanoid;

pub fn generate_url_id(base: &str) -> String {
    let mut alphabet: Vec<char> = ('0'..='9').collect();
    alphabet.extend('a'..='z');

    let unique_part: String = nanoid!(8, &alphabet);

    let sanitised_base: String = base
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();

    let mut url_id: String = sanitised_base
        .split(" ")
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("-");

    url_id.push('-');
    url_id.extend(unique_part.chars());
    url_id
}