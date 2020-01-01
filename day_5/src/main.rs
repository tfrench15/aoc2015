
fn main() {
    let strings = load_input();

    let count = strings
        .iter()
        .filter(|word| has_three_vowels(&word))
        .filter(|word| has_double_letter(&word))
        .filter(|word| has_no_bad_pairs(&word))
        .collect::<Vec<&String>>()
        .len();

    println!("count is {}", count);
}

fn has_three_vowels(input: &str) -> bool {
    let mut count = 0;

    for ch in input.chars() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => continue,
        };
    }

    count >= 3
}

fn has_double_letter(input: &str) -> bool {
    for window in input
        .split("")
        .filter(|ch| !ch.is_empty())
        .collect::<Vec<&str>>()
        .windows(2) {
            if window[0] == window[1] {
                return true
            }
        }

    false
}

fn has_no_bad_pairs(input: &str) -> bool {
    !(input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy"))
}

fn load_input() -> Vec<String> {
    include_str!("input.txt")
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_three_vowel_word_is_nice() {
        let ok = super::has_three_vowels("avegoz");
        assert_eq!(ok, true);
    }

    #[test]
    fn test_two_vowel_word_is_not_nice() {
        let ok = super::has_three_vowels("avegtrpyt");
        assert_eq!(ok, false);
    }

    #[test]
    fn test_double_letter_word_is_nice() {
        let ok = super::has_double_letter("abweewfoefw");
        assert_eq!(ok, true);
    }

    #[test]
    fn test_no_double_letter_word_is_not_nice() {
        let ok = super::has_double_letter("abcdefghijklmnop");
        assert_eq!(ok, false);
    }

    #[test]
    fn test_contains_no_bad_pairs_is_nice() {
        let ok = super::has_no_bad_pairs("aemnorhello");
        assert_eq!(ok, true)
    }

    #[test]
    fn test_contains_bad_pair_is_not_nice() {
        let ok = super::has_no_bad_pairs("acwawepqoawepfas");
        assert_eq!(ok, false)
    }
}



