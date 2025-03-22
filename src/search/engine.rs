// use crate::bible::Bible;
use crate::bible::{BibleNumeric, ChapterNumeric};
// use crate::utils::kebob_to_title;
use std::collections::BTreeMap;

pub enum SearchResult<'a> {
    Book(&'a BTreeMap<u32, ChapterNumeric>),
    Chapter(&'a ChapterNumeric),
    Verse(&'a String),
}

pub fn search_by_reference<'a>(
    bible: &'a BibleNumeric,
    book: &str,
    chapter: Option<u32>,
    verse: Option<u32>,
) -> Option<SearchResult<'a>> {
    let book_data = bible.books.get(book)?;
    match (chapter, verse) {
        (None, _) => Some(SearchResult::Book(&book_data.chapters)),
        (Some(ch), None) => {
            let chapter_data = book_data.chapters.get(&ch)?;
            Some(SearchResult::Chapter(chapter_data))
        }
        (Some(ch), Some(v)) => {
            let chapter_data = book_data.chapters.get(&ch)?;
            let verse_text = chapter_data.verses.get(&v)?;
            Some(SearchResult::Verse(verse_text))
        }
    }
}