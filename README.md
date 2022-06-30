# Rust Katas

> Code algorithms using rust.

## Instructions

Trolls are attacking your comment section!

A common way to deal with this situation is to remove all of the vowels from the trolls' comments, neutralizing the threat.

Your task is to write a function that takes a string and return a new string with all vowels removed.

For example, the string "This website is for losers LOL!" would become "Ths wbst s fr lsrs LL!".

Note: for this kata y isn't considered a vowel.

## Test cases

```rust
#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn test_disemvowel() {
        assert_eq!(disemvowel(""), "");
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }

    #[test]
    fn test_random() {
        for _ in 0..100 {
            let s = random_string();
            let expected = reference_solution(&s);
            assert_eq!(disemvowel(&s), expected);
        }
    }

    fn random_string() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789 ";
        let mut rng = rand::thread_rng();

        (0..rng.gen_range(0..200))
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    fn reference_solution(s: &str) -> String {
        s.chars()
            .filter(|&c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => false,
                _ => true,
            })
            .collect()
    }
}
```
