/// Lexicon module for word validation
///
/// This module provides fast word validation using a HashSet-based approach
/// optimized for various word list formats (CSW24, ENABLE, etc.).

use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Lexicon structure containing all valid words
#[derive(Debug, Clone)]
pub struct Lexicon {
    /// All valid words stored in a HashSet for O(1) lookup
    words: HashSet<String>,
    /// Total word count
    word_count: usize,
    /// Name of the loaded lexicon (e.g., "CSW24", "ENABLE", "Custom")
    pub lexicon_name: String,
}

impl Lexicon {
    /// Creates a new empty lexicon
    pub fn new() -> Self {
        Self {
            words: HashSet::new(),
            word_count: 0,
            lexicon_name: "Empty".to_string(),
        }
    }

    /// Loads lexicon from a text file (one word per line)
    ///
    /// # Arguments
    /// * `path` - Path to the lexicon file (e.g., "CSW24.txt")
    ///
    /// # Returns
    /// * `Result<Lexicon, String>` - Loaded lexicon or error message
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path_ref = path.as_ref();
        let content = fs::read_to_string(path_ref)
            .map_err(|e| format!("Failed to read lexicon file: {}", e))?;

        let mut words = HashSet::new();

        for line in content.lines() {
            let word = line.trim().trim_matches('\r').to_uppercase();
            if !word.is_empty() {
                words.insert(word);
            }
        }

        let word_count = words.len();

        // Extract lexicon name from filename (e.g., "CSW24.txt" -> "CSW24")
        let lexicon_name = path_ref
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        Ok(Self { words, word_count, lexicon_name })
    }

    /// Attempts to load a lexicon from multiple sources (in priority order)
    ///
    /// Tries to load lexicons in the following order:
    /// 1. CSW24.txt (if user has licensed copy)
    /// 2. TML.txt (TileMania Lexicon - RE-Enable filtered to CSW24 words, avoids US/UK confusion)
    /// 3. RE-ENABLE.txt (full public domain RE-Enable, 172K words)
    /// 4. ENABLE.txt (original public domain, 173K words)
    /// 5. custom.txt (user-provided)
    ///
    /// # Returns
    /// * `Result<Lexicon, String>` - First successfully loaded lexicon or error if none found
    pub fn load_default() -> Result<Self, String> {
        let paths = vec![
            "assets/lexicons/CSW24.txt",
            "assets/lexicons/TML.txt",
            "assets/lexicons/RE-ENABLE.txt",
            "assets/lexicons/ENABLE.txt",
            "assets/lexicons/custom.txt",
            "CSW24.txt",  // Try root directory as fallback
            "TML.txt",
            "RE-ENABLE.txt",
            "ENABLE.txt",
        ];

        let mut last_error = String::new();

        for path in paths {
            match Self::load_from_file(path) {
                Ok(lexicon) => {
                    println!("✓ Loaded {} lexicon with {} words from {}",
                             lexicon.lexicon_name, lexicon.word_count, path);
                    return Ok(lexicon);
                }
                Err(e) => {
                    last_error = e;
                }
            }
        }

        Err(format!(
            "No word list found! Tried multiple locations.\n\
             Last error: {}\n\n\
             To use TileMania, you must provide a word list:\n\
             1. Download RE-ENABLE (recommended): https://github.com/JakesMD/Re-Enable\n\
             2. Save as: assets/lexicons/RE-ENABLE.txt\n\
             OR\n\
             1. Download ENABLE (public domain): http://www.puzzlers.org/pub/wordlists/enable1.txt\n\
             2. Save as: assets/lexicons/ENABLE.txt\n\n\
             See assets/lexicons/README.md for more options and licensing information.",
            last_error
        ))
    }

    /// Validates if a word exists in the lexicon (case-insensitive)
    ///
    /// # Arguments
    /// * `word` - The word to validate
    ///
    /// # Returns
    /// * `bool` - true if valid, false otherwise
    pub fn is_valid(&self, word: &str) -> bool {
        let normalized = word.trim().to_uppercase();
        self.words.contains(&normalized)
    }

    /// Gets the total number of words in the lexicon
    pub fn word_count(&self) -> usize {
        self.word_count
    }

    /// Gets all words of a specific length
    ///
    /// # Arguments
    /// * `length` - Desired word length
    ///
    /// # Returns
    /// * `Vec<String>` - All words of that length (sorted)
    pub fn get_words_by_length(&self, length: usize) -> Vec<String> {
        let mut filtered: Vec<String> = self.words
            .iter()
            .filter(|w| w.len() == length)
            .cloned()
            .collect();

        filtered.sort();
        filtered
    }

    /// Gets all 2-letter words (optimized for Stage 1)
    pub fn get_two_letter_words(&self) -> Vec<String> {
        self.get_words_by_length(2)
    }

    /// Checks if a word exists and returns it in uppercase if valid
    pub fn validate_and_normalize(&self, word: &str) -> Option<String> {
        let normalized = word.trim().to_uppercase();
        if self.words.contains(&normalized) {
            Some(normalized)
        } else {
            None
        }
    }
}

impl Default for Lexicon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexicon_creation() {
        let lexicon = Lexicon::new();
        assert_eq!(lexicon.word_count(), 0);
    }

    #[test]
    fn test_word_validation() {
        let mut lexicon = Lexicon::new();
        lexicon.words.insert("AA".to_string());
        lexicon.words.insert("AB".to_string());
        lexicon.word_count = 2;

        assert!(lexicon.is_valid("AA"));
        assert!(lexicon.is_valid("aa")); // case-insensitive
        assert!(lexicon.is_valid("AB"));
        assert!(!lexicon.is_valid("ZZ"));
    }

    #[test]
    fn test_get_words_by_length() {
        let mut lexicon = Lexicon::new();
        lexicon.words.insert("AA".to_string());
        lexicon.words.insert("AB".to_string());
        lexicon.words.insert("ABC".to_string());
        lexicon.word_count = 3;

        let two_letter = lexicon.get_words_by_length(2);
        assert_eq!(two_letter.len(), 2);
        assert!(two_letter.contains(&"AA".to_string()));
        assert!(two_letter.contains(&"AB".to_string()));

        let three_letter = lexicon.get_words_by_length(3);
        assert_eq!(three_letter.len(), 1);
        assert!(three_letter.contains(&"ABC".to_string()));
    }
}
