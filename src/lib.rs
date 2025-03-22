pub mod bible;
pub mod search;
mod utils;

use bible::{Bible, BibleNumeric};
use search::{SearchResult, search_by_reference};
// use utils::kebob_to_title;

/// Struct to represent the Patina Bible search utility.
pub struct Patina {
    bible: BibleNumeric,
}

impl Patina {
    pub fn new() -> Self {
        let bible_json = include_str!("data/bible.json");
        let bible: Bible = serde_json::from_str(bible_json).expect("Failed to parse JSON");

        let bible_numeric = bible.normalize();
        Patina {
            bible: bible_numeric,
        }
    }

    /// Search by book, chapter, and verse.
    pub fn search_by_reference(
        &self,
        book: &str,
        chapter: Option<u32>,
        verse: Option<u32>,
    ) -> Option<String> {
        match search_by_reference(&self.bible, book, chapter, verse) {
            Some(SearchResult::Verse(text)) => Some(text.to_owned()),
            Some(SearchResult::Chapter(ch)) => Some(format!(
                "Chapter:\n{}",
                ch.verses
                    .iter()
                    .map(|(v, t)| format!("{}: {}", v, t))
                    .collect::<Vec<_>>()
                    .join("\n")
            )),
            Some(SearchResult::Book(chapters)) => {
                let out = chapters
                    .iter()
                    .map(|(ch_num, ch)| {
                        let verses = ch
                            .verses
                            .iter()
                            .map(|(v, t)| format!("{}: {}", v, t))
                            .collect::<Vec<_>>()
                            .join("\n");
                        format!("Chapter {}:\n{}", ch_num, verses)
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n");
                Some(out)
            }
            None => Some(format!("Reference not found.")),
        }
    }
}
