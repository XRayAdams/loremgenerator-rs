use rand::Rng;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppViewModel {
    pub max_words: usize,
    pub max_sentences: usize,
    pub paragraphs: usize,
    pub start_with_lorem: bool,
}

impl AppViewModel {
    fn get_config_path() -> PathBuf {
        let mut path = gtk4::glib::user_config_dir();
        path.push("loremgenerator");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    pub fn new() -> Self {
        Self::load().unwrap_or_else(|| Self {
            max_words: 15,
            max_sentences: 4,
            paragraphs: 5,
            start_with_lorem: true,
        })
    }

    fn load() -> Option<Self> {
        let path = Self::get_config_path();
        if let Ok(content) = fs::read_to_string(path) {
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    pub fn save(&self) {
        let path = Self::get_config_path();
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, content);
        }
    }

    pub fn set_max_words(&mut self, value: usize) {
        self.max_words = value;
        self.save();
    }

    pub fn set_max_sentences(&mut self, value: usize) {
        self.max_sentences = value;
        self.save();
    }

    pub fn set_paragraphs(&mut self, value: usize) {
        self.paragraphs = value;
        self.save();
    }

    pub fn set_start_with_lorem(&mut self, value: bool) {
        self.start_with_lorem = value;
        self.save();
    }

    pub fn get_app_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    pub fn generate(&self) -> String {
        use crate::static_data::StaticData;
        let w_min = 6;
        let mut rng = rand::thread_rng();
        let mut result = if self.start_with_lorem { "Lorem ipsum ".to_string() } else { String::new() };

        for i in 0..self.paragraphs {
            if !result.is_empty() && result != "Lorem ipsum " {
                result.push_str("\n\n");
            }

            let sen_count = rng.gen_range(1..=self.max_sentences.max(1));
            for j in 0..sen_count {
                let number_of_words = rng.gen_range(w_min..=(self.max_words.max(w_min)));
                for i2 in 0..number_of_words {
                    let word = StaticData::ALL_WORDS[rng.gen_range(0..StaticData::ALL_WORDS.len())];
                    let word = if i2 == 0 && (!self.start_with_lorem || (self.start_with_lorem && (i > 0 || (i == 0 && j > 0)))) {
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


}
