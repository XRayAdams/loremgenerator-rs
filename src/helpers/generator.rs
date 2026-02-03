// Copyright (c) 2025, 2026 Konstantin Adamov. Licensed under MIT.

use rand::Rng;
pub fn generate(start_with_lorem: bool, paragraphs : usize, max_sentences: usize, max_words: usize) -> String {
        use super::static_data::StaticData;
        let w_min = 6;
        let mut rng = rand::thread_rng();
        let mut result = if start_with_lorem { "Lorem ipsum ".to_string() } else { String::new() };
        for i in 0..paragraphs {
            if !result.is_empty() && result != "Lorem ipsum " {
                result.push_str("\n\n");
            }

            let sen_count = rng.gen_range(1..=max_sentences.max(1));
            for j in 0..sen_count {
                let number_of_words = rng.gen_range(w_min..=(max_words.max(w_min)));
                for i2 in 0..number_of_words {
                    let word = StaticData::ALL_WORDS[rng.gen_range(0..StaticData::ALL_WORDS.len())];
                    let word = if i2 == 0 && (!start_with_lorem || (start_with_lorem && (i > 0 || (i == 0 && j > 0)))) {
                        let mut c = word.chars();
                        match c.next() {
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                            None => String::new(),
                        }
                    } else {
                        word.to_string()
                    };
                    if i2 > 0 {
                        result.push(' ');
                    }
                    result.push_str(&word);
                }
                if rng.gen_range(0..100) < 80 {
                    result.push('.');
                } else {
                    result.push('?');
                }
                if j < sen_count - 1 {
                    result.push(' ');
                }
            }
        }
        result
    }
