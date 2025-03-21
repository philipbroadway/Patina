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
        // Use the normalized Bible for lookups
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

    /// Search by text query of the format "book chapter:verse".
    pub fn search_by_text(&self, query: &str) -> Option<String> {
        let query_parts: Vec<&str> = query.rsplitn(2, ' ').collect();
        if query_parts.len() != 2 {
            return None;
        }
        let chapter_verse = query_parts[0];
        let book = query_parts[1].to_lowercase().replace(" ", "-");
        let parts: Vec<&str> = chapter_verse.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        let chapter = parts[0].parse::<u32>().ok();
        let verse = parts[1].parse::<u32>().ok();
        self.search_by_reference(&book, chapter, verse)
    }
}
