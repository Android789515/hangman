use std::{fs, path::Path};

pub fn select_random_word() -> String {
    let words_file = Path::new("./src/words.txt");

    let mut rng = rand::rng();
    let random_index = rand::Rng::random_range(&mut rng, 0..999);

    fs::read_to_string(words_file)
        .unwrap()
        .split('\n')
        .enumerate()
        .fold(String::new(), |selected_word, (index, current_word)| {
            if index == random_index {
                String::from(current_word)
            } else {
                selected_word
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_random_word() {
        assert!(!select_random_word().is_empty());
    }
}
