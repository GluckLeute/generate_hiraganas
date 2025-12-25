use rand::prelude::*;

fn main() {
    let max_words_count = 3;
    let all_words = vec!["あ","い", "う", "え", "お",
        "か", "き", "く", "け", "こ",
        "さ", "し", "す", "せ", "そ",
        "た", "ち", "つ", "て", "と",
        "な", "に", "ぬ", "ね", "の",
        "は", "ひ", "ふ", "へ", "ほ",
        "ま", "み", "む", "め", "も",
        "や", "ゆ", "よ",
        "ら", "り", "る", "れ", "ろ",
        "わ", "を", "ん"
    ];

    let ban_first_word = vec!["ん"];

    let banned_words: Vec<&str> = all_words
    .iter()
    .copied()
    .filter(|word| !ban_first_word.contains(word))
    .collect();

    for _j in 0..10{
        let mut result = Vec::new();
        for i in 0..max_words_count {
            if i == 0{
                result.push(*banned_words.choose(&mut rand::rng()).unwrap());
            } else {
                result.push(*all_words.choose(&mut rand::rng()).unwrap());
        }
        }
        let output = result.into_iter().collect::<String>();
        println!("{}", output);
    }
}