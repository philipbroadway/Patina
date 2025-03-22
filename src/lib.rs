pub mod bible;
pub mod search;
mod utils;

use bible::{Bible, BibleNumeric};

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

    pub fn list_books(&self) -> Vec<String> {
      self.bible.books.keys().cloned().collect()
    }

    pub fn list_chapters(&self, book: &str) -> Option<Vec<String>> {
      self.bible.books.get(book).map(|b| {
          b.chapters.keys().map(|&ch| ch.to_string()).collect()
      })
    }

    pub fn list_verses(&self, book: &str, chapter: u32) -> Option<Vec<String>> {
      self.bible.books.get(book).and_then(|b| {
          b.chapters.get(&chapter).map(|c| {
              c.verses.keys().map(|&v| v.to_string()).collect()
          })
      })
    }

    /// Search by book, chapter, and verse.
    pub fn search_by_reference(
        &self,
        book: &str,
        chapter: Option<u32>,
        verse: Option<u32>,
    ) -> Option<String> {
        match search::search_by_reference(&self.bible, book, chapter, verse) {
            Some(search::SearchResult::Verse(text)) => Some(text.to_owned()),
            Some(search::SearchResult::Chapter(ch)) => Some(format!(
                "Chapter:\n{}",
                ch.verses
                    .iter()
                    .map(|(v, t)| format!("{}: {}", v, t))
                    .collect::<Vec<_>>()
                    .join("\n")
            )),
            Some(search::SearchResult::Book(chapters)) => {
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
