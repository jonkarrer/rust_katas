fn disemvowel(comment: &str) -> String {
    // * identify vowels
    fn id_vowel(s:&str) -> &str {
        // * remove each vowel
        match s {
            "a" => "",
            "e" => "",
            "i" => "",
            "o" => "",
            "u" => "",
            _ => s
        }
    }
     // * Loop through string
    let str_arr: Vec<&str> = comment.split("").map(|s:&str| id_vowel(s)).collect();

    // * return string
    return str_arr.join("");
}

fn main() {
   println!("{}", disemvowel("Hello World"));
}

// ! Top Answer

// fn disemvowel(s: &str) -> String {
//     s.chars()
//         .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
//         .collect()
// }