fn main() {
    let s = "Hello Poop Boy What on Earth are you trying to say to me";
    const VOWEL_SUFFIX: &str = "hay";
    const CONSONANT_SUFFIX: &str = "ay";

    for word in s.split_whitespace() {
        let first_char = word.char_indices().next().unwrap().1;
        if is_vowel(first_char) {
            print!("{}-{} ", word[..].to_string().to_lowercase(), VOWEL_SUFFIX);
        } else {
            print!(
                "{}-{}{} ",
                word[1..].to_string().to_lowercase(),
                first_char.to_lowercase(),
                CONSONANT_SUFFIX
            );
        }
    }
}

fn is_vowel(c: char) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    for vowel in VOWELS {
        if c == vowel {
            return true;
        }
    }

    false
}
