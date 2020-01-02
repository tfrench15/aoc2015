use std::collections::HashMap;

fn main() {
    let strings = load_input();

    let count = strings
        .iter()
        .filter(|word| has_repeating_pair(&word))
        .filter(|word| has_one_letter_repeat(&word))
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

fn has_repeating_pair(input: &str) -> bool {
    let mut counts = HashMap::new();

    let slice = input
        .split("")
        .filter(|ch| !ch.is_empty())
        .collect::<Vec<&str>>();

    for window in slice.windows(2) {
        let first = window[0];
        let second = window[1];

        let counter = counts.entry((first, second)).or_insert(0);
        *counter += 1;
    }

    // re-scan the list to remove overlaps
    for window in slice.windows(3) {
        let first = window[0];
        let second = window[1];
        let third = window[2];

        if first == second && second == third {
            if let Some(counter) = counts.get_mut(&(first, second)) {
                *counter -= 1;
            }
        }
    }

    for val in counts.values() {
        if *val >= 2 {
            return true
        }
    }

    false
}

fn has_one_letter_repeat(input: &str) -> bool {
    let slice = input
        .split("")
        .filter(|ch| !ch.is_empty())
        .collect::<Vec<&str>>();

    for window in slice.windows(3) {
        if window[0] == window[2] {
            return true
        }
    }

    false
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

    #[test]
    fn test_has_repeating_pair_is_nice() {
        let ok = super::has_repeating_pair("abcdefgcdxyz");
        assert_eq!(ok, true);
    }

    #[test]
    fn test_no_repeating_pair_is_not_nice() {
        let ok = super::has_repeating_pair("abcdefghijklmnop");
        assert_eq!(ok, false);
    }

    #[test]
    fn test_overlapping_pairs_are_not_nice() {
        let ok = super::has_repeating_pair("abcdefghhhijklmnop");
        assert_eq!(ok, false);
    }

    #[test]
    fn test_has_one_letter_repeat_is_nice() {
        let ok = super::has_one_letter_repeat("asawepwladasdlfwe");
        assert_eq!(ok, true);
    }

    #[test]
    fn test_no_repeating_letters_is_not_nice() {
        let ok = super::has_one_letter_repeat("abcdefghijklmnop");
        assert_eq!(ok, false);
    }

    #[test]
    fn part_two_case_one() {
        let input = String::from("qjhvhtzxzqqjkmpb");

        assert!(super::has_repeating_pair(&input) && super::has_one_letter_repeat(&input));
    }

    #[test]
    fn part_two_case_two() {
        let input = String::from("xxyxx");

        println!("has repeating pair output: {}", super::has_repeating_pair(&input));
        println!("has one letter repeat output: {}", super::has_one_letter_repeat(&input));

        assert!(super::has_repeating_pair(&input) && super::has_one_letter_repeat(&input));
    }

    #[test]
    fn part_two_case_three() {
        let input = String::from("uurcxstgmygtbstg");

        assert!(!(super::has_repeating_pair(&input) && super::has_one_letter_repeat(&input)))
    }

    #[test]
    fn part_two_case_four() {
        let input = String::from("ieodomkazucvgmuy");

        assert!(!(super::has_repeating_pair(&input) && super::has_one_letter_repeat(&input)))
    }
}



